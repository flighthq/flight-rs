import type {
  IrDeclaration,
  IrExpression,
  IrFunctionDeclaration,
  IrParameter,
  IrStatement,
  IrType,
  IrVariable,
  IrVariableDeclaration,
} from '../model/ir.ts';

export interface RustModule {
  declarations: IrDeclaration[];
  imports?: RustImport[];
  inlineFunctions?: IrFunctionDeclaration[];
  semanticTypes?: Readonly<Record<string, IrType>>;
  source: string;
  typeImports: string[];
}

export interface RustImport {
  module: string;
  names: Array<{ imported: string; local: string }>;
}

interface EmitContext {
  anonymousTypes: ReadonlyMap<string, string>;
  callbackTypes: ReadonlySet<string>;
  constantNames: ReadonlyMap<string, string>;
  constantValues: ReadonlyMap<string, number>;
  continueEpilogue: readonly string[];
  currentReturnType?: IrType | undefined;
  erasedValueNames: ReadonlySet<string>;
  functions: ReadonlyMap<string, IrFunctionDeclaration>;
  inlineFunctions: ReadonlyMap<string, IrFunctionDeclaration>;
  mutatedNames: ReadonlySet<string>;
  mutatingFunctions: ReadonlyMap<string, ReadonlySet<number>>;
  namedTypes: ReadonlyMap<string, IrType>;
  symbolTypes: Map<string, IrType>;
}

export class RustEmissionError extends Error {}

export function emitRustModule(module: RustModule): string {
  const constantNames = new Map(
    module.declarations
      .filter(
        (declaration): declaration is IrVariableDeclaration =>
          declaration.kind === 'variable' && declaration.initializer?.kind !== 'function',
      )
      .map((declaration) => [declaration.name, screamingSnakeCase(declaration.name)]),
  );
  const constantValues = new Map<string, number>();
  for (const declaration of module.declarations) {
    if (declaration.kind !== 'variable' || !declaration.initializer || declaration.initializer.kind === 'function') {
      continue;
    }
    const value = evaluateConstant(declaration.initializer, constantValues);
    if (value !== undefined) constantValues.set(declaration.name, value);
  }
  const inlineFunctions = new Map((module.inlineFunctions ?? []).map((declaration) => [declaration.name, declaration]));
  const mutatingFunctions = collectMutatingFunctionParameters([
    ...module.declarations,
    ...(module.inlineFunctions ?? []),
  ]);
  const context: EmitContext = {
    anonymousTypes: new Map(),
    callbackTypes: new Set(['EasingFunction', 'ScalarRemap']),
    constantNames,
    constantValues,
    continueEpilogue: [],
    erasedValueNames: new Set(
      module.declarations.filter((declaration) => declaration.kind === 'type').map((declaration) => declaration.name),
    ),
    functions: new Map(
      module.declarations
        .filter((declaration): declaration is IrFunctionDeclaration => declaration.kind === 'function')
        .map((declaration) => [declaration.name, declaration]),
    ),
    inlineFunctions,
    mutatedNames: new Set(),
    mutatingFunctions,
    namedTypes: new Map([
      ...Object.entries(module.semanticTypes ?? {}),
      ...module.declarations
        .filter((declaration) => declaration.kind === 'type')
        .map((declaration) => [declaration.name, declaration.type] as const),
    ]),
    symbolTypes: new Map(),
  };
  const declarations = module.declarations.map((declaration) => emitDeclaration(declaration, context)).join('\n\n');
  const importGroups: RustImport[] = [
    ...(module.typeImports.length > 0
      ? [
          {
            module: 'crate',
            names: [...new Set(module.typeImports)].map((name) => ({ imported: name, local: name })),
          },
        ]
      : []),
    ...(module.imports ?? []),
  ];
  const imports = importGroups
    .map((group) => {
      const names = [
        ...new Map(
          group.names
            .filter(({ local }) => new RegExp(`\\b${local}\\b`, 'u').test(declarations))
            .map((item) => [item.local, item]),
        ).values(),
      ].sort((left, right) => left.local.localeCompare(right.local));
      if (names.length === 0) return '';
      const bindings = names
        .map(({ imported, local }) => (imported === local ? imported : `${imported} as ${local}`))
        .join(', ');
      return `use ${group.module}::{${bindings}};`;
    })
    .filter(Boolean)
    .join('\n');
  const numericCoercions = declarations.includes('__flight_js_to_')
    ? [
        '#[inline]',
        'fn __flight_js_to_u32(value: f64) -> u32 {',
        indent(
          [
            'if !value.is_finite() || value == 0.0 { return 0; }',
            'value.trunc().rem_euclid(4294967296.0_f64) as u32',
          ].join('\n'),
        ),
        '}',
        ...(declarations.includes('__flight_js_to_i32(')
          ? [
              '',
              '#[inline]',
              'fn __flight_js_to_i32(value: f64) -> i32 {',
              indent('__flight_js_to_u32(value) as i32'),
              '}',
            ]
          : []),
      ].join('\n')
    : '';
  return [
    `// @generated from ${module.source}; do not edit.`,
    '#![allow(clippy::excessive_precision)]',
    '#![allow(non_upper_case_globals)]',
    '#![allow(unused_braces)]',
    '#![allow(unused_imports)]',
    '#![allow(unused_mut)]',
    '#![allow(unused_parens)]',
    '',
    `${imports.length > 0 ? `${imports}\n\n` : ''}${numericCoercions.length > 0 ? `${numericCoercions}\n\n` : ''}${declarations}`,
    '',
  ].join('\n');
}

function emitDeclaration(declaration: IrDeclaration, context: EmitContext): string {
  const provenance = `// Source: ${declaration.origin.source}:${String(declaration.origin.line)} (${declaration.origin.fingerprint})`;
  switch (declaration.kind) {
    case 'function':
      return `${provenance}\n${emitFunctionDeclaration(declaration, context)}`;
    case 'variable':
      return `${provenance}\n${emitTopLevelVariable(declaration, context)}`;
    case 'type':
      return `${provenance}\n${emitTypeDeclaration(declaration.name, declaration.exported, declaration.type, context)}`;
    case 'class':
    case 'enum':
      throw new RustEmissionError(
        `${declaration.origin.source}:${String(declaration.origin.line)}: unsupported Rust declaration ${declaration.kind} ${declaration.name}`,
      );
  }
}

function emitTopLevelVariable(declaration: IrVariableDeclaration, context: EmitContext): string {
  if (context.erasedValueNames.has(declaration.name)) {
    return `// TypeScript value namespace ${declaration.name} is represented by its generated Rust type.`;
  }
  if (!declaration.initializer) {
    throw new RustEmissionError(`${declaration.origin.source}: uninitialized top-level variable ${declaration.name}`);
  }
  if (declaration.initializer.kind === 'function') {
    return emitLiftedFunction(declaration, declaration.initializer, context);
  }
  const visibility = declaration.exported ? 'pub ' : '';
  const name = context.constantNames.get(declaration.name) ?? screamingSnakeCase(declaration.name);
  const type = declaration.type ? emitType(declaration.type, context) : inferExpressionType(declaration.initializer);
  const folded = context.constantValues.get(declaration.name);
  const initializer = folded === undefined ? emitExpression(declaration.initializer, context) : emitLiteral(folded);
  return `${visibility}const ${name}: ${type} = ${initializer};`;
}

function emitLiftedFunction(
  declaration: IrVariableDeclaration,
  expression: Extract<IrExpression, { kind: 'function' }>,
  context: EmitContext,
): string {
  const callback = declaration.type?.kind === 'named' && context.callbackTypes.has(declaration.type.name);
  const returns = expression.returns ?? (callback ? primitive('Float') : undefined);
  if (!returns) {
    throw new RustEmissionError(`${declaration.origin.source}: cannot infer return type for ${declaration.name}`);
  }
  const nextContext = functionContext(context, declaration.name, expression, returns);
  registerParameters(expression.parameters, nextContext, callback ? [primitive('Float')] : []);
  const parameters = expression.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, callback ? primitive('Float') : undefined, expression),
  );
  const body = expression.expression
    ? `{\n${indent(`return ${emitExpression(expression.expression, nextContext, returns)};`)}\n}`
    : emitStatementsAsBlock(expression.body, nextContext);
  return `${emitAnonymousDefinitions(nextContext)}${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> ${emitType(returns, nextContext)} ${body}`;
}

function emitFunctionDeclaration(declaration: IrFunctionDeclaration, context: EmitContext): string {
  if (declaration.async) {
    throw new RustEmissionError(
      `${declaration.origin.source}: async Rust lowering is not implemented for ${declaration.name}`,
    );
  }
  const nextContext = functionContext(context, declaration.name, declaration, declaration.returns);
  registerParameters(declaration.parameters, nextContext);
  const parameters = declaration.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, undefined, declaration),
  );
  const defaults = declaration.parameters.flatMap((parameter) => {
    if (!parameter.initializer) return [];
    const name = safeName(parameter.name);
    return [`let ${name} = ${name}.unwrap_or(${emitExpression(parameter.initializer, nextContext, parameter.type)});`];
  });
  const body = emitStatementsAsBlock(
    defaults.length > 0
      ? [{ declarations: [], kind: 'variable' } as IrStatement, ...declaration.body]
      : declaration.body,
    nextContext,
    defaults,
  );
  return `${emitAnonymousDefinitions(nextContext)}${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> ${emitType(declaration.returns, nextContext)} ${body}`;
}

function emitParameter(
  parameter: IrParameter,
  context: EmitContext,
  fallbackType: IrType | undefined,
  owner: IrExpression | IrFunctionDeclaration,
  borrowRecords = true,
): string {
  const type = parameter.type.kind === 'dynamic' && fallbackType ? fallbackType : parameter.type;
  const emitted = emitType(type, context);
  const optional = parameter.optional || parameter.initializer;
  const resolved = resolveSemanticType(type, context);
  const referenceLike =
    resolved?.kind === 'anonymous' ||
    resolved?.kind === 'array' ||
    (resolved?.kind === 'named' && Boolean(typedArrayType(resolved.name)));
  const assigned = referenceLike ? context.mutatedNames.has(parameter.name) : assignsName(owner, parameter.name);
  const captured = capturesParameterInReturnedClosure(owner, parameter.name);
  const borrowed = referenceLike && !optional && !parameter.rest && borrowRecords && !captured;
  const name = `${assigned && !borrowed ? 'mut ' : ''}${safeName(parameter.name)}`;
  const storage = optional ? `Option<${emitted}>` : emitted;
  return `${name}: ${borrowed ? `${assigned ? '&mut ' : '&'}${storage}` : storage}`;
}

function emitStatementsAsBlock(statements: IrStatement[], context: EmitContext, prefix: string[] = []): string {
  const lines = [...prefix, ...statements.flatMap((statement) => emitStatement(statement, context))];
  return `{\n${indent(lines.join('\n'))}\n}`;
}

function emitStatement(statement: IrStatement, context: EmitContext): string[] {
  switch (statement.kind) {
    case 'block':
      return [emitStatementsAsBlock(statement.statements, context)];
    case 'break':
      return ['break;'];
    case 'continue':
      return [...context.continueEpilogue, 'continue;'];
    case 'do': {
      const condition = emitCondition(statement.condition, context);
      const conditionCheck = `if !(${condition}) { break; }`;
      const loopContext = { ...context, continueEpilogue: [conditionCheck] };
      return ['loop {', indent(emitStatement(statement.body, loopContext).join('\n')), indent(conditionCheck), '}'];
    }
    case 'expression':
      return [
        statement.expression.kind === 'assignment'
          ? `${emitAssignmentStatement(statement.expression, context)};`
          : `${emitExpression(statement.expression, context)};`,
      ];
    case 'for':
      return emitForStatement(statement, context);
    case 'forOf': {
      if (statement.async) throw new RustEmissionError('async for-of Rust lowering is not implemented');
      const iterableType = inferIrExpressionType(statement.iterable, context);
      const elementType = iterableType?.kind === 'array' ? iterableType.element : undefined;
      if (elementType) context.symbolTypes.set(statement.variable, elementType);
      const iterable = emitExpression(statement.iterable, context);
      const loopContext = { ...context, continueEpilogue: [] };
      return [
        `for ${safeName(statement.variable)} in ${parenthesize(iterable)}.iter().cloned() ${emitStatementAsBlock(statement.body, loopContext)}`,
      ];
    }
    case 'if': {
      const lines = [
        `if ${emitCondition(statement.condition, context)} ${emitStatementAsBlock(statement.consequent, context)}`,
      ];
      if (statement.otherwise) {
        lines[0] += ` else ${emitStatementAsBlock(statement.otherwise, context)}`;
      }
      return lines;
    }
    case 'return':
      return [
        statement.expression
          ? `return ${emitExpression(statement.expression, context, context.currentReturnType)};`
          : 'return;',
      ];
    case 'switch':
      return emitSwitchStatement(statement, context);
    case 'throw':
      return [`panic!(${emitThrowMessage(statement.expression, context)});`];
    case 'try':
      throw new RustEmissionError('try Rust lowering is not implemented');
    case 'variable':
      return statement.declarations.flatMap((variable) => emitLocalVariable(variable, context));
    case 'while':
      return [
        `while ${emitCondition(statement.condition, context)} ${emitStatementAsBlock(statement.body, {
          ...context,
          continueEpilogue: [],
        })}`,
      ];
  }
}

function emitSwitchStatement(statement: Extract<IrStatement, { kind: 'switch' }>, context: EmitContext): string[] {
  const value = emitExpression(statement.expression, context);
  const branches: string[] = [];
  let defaultBranch: string | undefined;
  for (const switchCase of statement.cases) {
    if (!caseTerminates(switchCase.statements)) {
      throw new RustEmissionError('fall-through switch cases require explicit Rust lowering');
    }
    const body = emitStatementsAsBlock(switchCase.statements, context);
    if (switchCase.expression) {
      const condition = emitExpression(switchCase.expression, context);
      branches.push(`${branches.length === 0 ? 'if' : 'else if'} __switch_value == ${condition} ${body}`);
    } else {
      defaultBranch = `${branches.length === 0 ? '' : 'else '}${body}`;
    }
  }
  if (defaultBranch) branches.push(defaultBranch);
  return ['{', indent(`let __switch_value = ${value};\n${branches.join(' ')}`), '}'];
}

function caseTerminates(statements: IrStatement[]): boolean {
  const last = statements.at(-1);
  if (!last) return false;
  if (last.kind === 'return' || last.kind === 'throw') return true;
  if (last.kind === 'block') return caseTerminates(last.statements);
  return false;
}

function emitForStatement(statement: Extract<IrStatement, { kind: 'for' }>, context: EmitContext): string[] {
  const initializer = Array.isArray(statement.initializer)
    ? statement.initializer.flatMap((variable) => emitLocalVariable(variable, context))
    : statement.initializer
      ? [`${emitExpression(statement.initializer, context)};`]
      : [];
  const condition = statement.condition ? emitCondition(statement.condition, context) : 'true';
  const increment = statement.increment ? `${emitExpression(statement.increment, context)};` : undefined;
  const body = emitStatementAsBlock(
    statement.body,
    { ...context, continueEpilogue: increment ? [increment] : [] },
    statement.increment,
  );
  return ['{', indent([...initializer, `while ${condition} ${body}`].join('\n')), '}'];
}

function emitStatementAsBlock(
  statement: IrStatement,
  context: EmitContext,
  increment?: IrExpression | undefined,
): string {
  const statements = statement.kind === 'block' ? statement.statements : [statement];
  const lines = statements.flatMap((item) => emitStatement(item, context));
  if (increment) lines.push(`${emitExpression(increment, context)};`);
  return `{\n${indent(lines.join('\n'))}\n}`;
}

function emitLocalVariable(variable: IrVariable, context: EmitContext): string[] {
  const mutable = variable.mutable || context.mutatedNames.has(variable.name);
  if (!variable.initializer) {
    if (!variable.type) throw new RustEmissionError(`cannot infer uninitialized local ${variable.name}`);
    context.symbolTypes.set(variable.name, variable.type);
    return [`let ${mutable ? 'mut ' : ''}${safeName(variable.name)}: ${emitType(variable.type, context)};`];
  }
  const expected = variable.type;
  const inferred = expected ?? inferIrExpressionType(variable.initializer, context);
  if (
    inferred &&
    isReferenceLike(inferred, context) &&
    (variable.initializer.kind === 'property' || variable.initializer.kind === 'identifier')
  ) {
    context.symbolTypes.set(variable.name, inferred);
    const reference = mutable ? '&mut ' : '&';
    return [`let ${safeName(variable.name)} = ${reference}${emitPlaceExpression(variable.initializer, context)};`];
  }
  const initializer = emitExpression(variable.initializer, context, expected);
  if (inferred) context.symbolTypes.set(variable.name, inferred);
  const annotation = expected && expected.kind !== 'dynamic' ? `: ${emitType(expected, context)}` : '';
  return [`let ${mutable ? 'mut ' : ''}${safeName(variable.name)}${annotation} = ${initializer};`];
}

function emitExpression(expression: IrExpression, context: EmitContext, expectedType?: IrType | undefined): string {
  switch (expression.kind) {
    case 'array':
      return `vec![${expression.elements.map((item) => emitExpression(item, context)).join(', ')}]`;
    case 'assignment':
      return emitAssignment(expression, context);
    case 'await':
      throw new RustEmissionError('await Rust lowering is not implemented');
    case 'binary':
      return coerceExpression(emitBinary(expression, context), expectedType);
    case 'call':
      return coerceExpression(emitCall(expression, context), expectedType);
    case 'cast':
      return `(${emitExpression(expression.expression, context)} as ${emitType(expression.type, context)})`;
    case 'conditional':
      return `if ${emitCondition(expression.condition, context)} { ${emitExpression(expression.whenTrue, context, expectedType)} } else { ${emitExpression(expression.whenFalse, context, expectedType)} }`;
    case 'element':
      return coerceExpression(emitElementRead(expression, context), expectedType);
    case 'function':
      return emitClosure(expression, context, expectedType);
    case 'identifier':
      return coerceExpression(emitIdentifier(expression.name, context), expectedType);
    case 'literal':
      return emitLiteral(expression.value, expectedType, context);
    case 'new':
      return emitNew(expression, context);
    case 'object':
      return emitObject(expression, context, expectedType);
    case 'property':
      return coerceExpression(emitProperty(expression, context), expectedType);
    case 'regexp':
      throw new RustEmissionError('regular expression Rust lowering is not implemented');
    case 'spread':
      throw new RustEmissionError('spread Rust lowering is not implemented');
    case 'template':
      throw new RustEmissionError('template-string Rust lowering is not implemented');
    case 'unary':
      return emitUnary(expression, context);
  }
}

function emitCall(expression: Extract<IrExpression, { kind: 'call' }>, context: EmitContext): string {
  if (expression.optional) throw new RustEmissionError('optional call Rust lowering is not implemented');
  if (isArrayFillConstruction(expression)) {
    const length = expression.callee.object.arguments[0];
    const value = expression.arguments[0];
    if (!length || !value) throw new RustEmissionError('new Array(length).fill(value) requires both arguments');
    return `vec![${emitExpression(value, context)}; ${parenthesize(emitExpression(length, context))} as usize]`;
  }
  if (expression.callee.kind === 'identifier') {
    const inline = context.inlineFunctions.get(expression.callee.name);
    if (inline) return emitInlineFunctionCall(expression, inline, context);
    const declaration = context.functions.get(expression.callee.name);
    if (declaration) return emitKnownFunctionCall(expression, declaration, context);
  }
  if (expression.callee.kind === 'property' && expression.callee.object.kind === 'identifier') {
    const owner = expression.callee.object.name;
    const method = expression.callee.name;
    if (owner === 'Math') {
      return emitMathCall(
        method,
        expression.arguments.map((argument) => emitExpression(argument, context)),
      );
    }
    if (owner === 'Number' && method === 'isFinite' && expression.arguments[0]) {
      return `${parenthesize(emitExpression(expression.arguments[0], context))}.is_finite()`;
    }
  }
  if (expression.callee.kind === 'property') {
    const ownerType = inferIrExpressionType(expression.callee.object, context);
    const method = expression.callee.name;
    const owner = emitExpression(expression.callee.object, context);
    if (ownerType?.kind === 'array' && method === 'push') {
      const argument = expression.arguments[0];
      if (!argument) throw new RustEmissionError('Array.push requires an argument');
      return `${owner}.push(${emitExpression(argument, context, ownerType.element)})`;
    }
    if (ownerType?.kind === 'array' && method === 'reduce') {
      const callback = expression.arguments[0];
      const initial = expression.arguments[1];
      if (callback?.kind !== 'function' || !initial) {
        throw new RustEmissionError('Array.reduce requires a callback and initial value');
      }
      const accumulatorType = inferIrExpressionType(initial, context) ?? primitive('Float');
      const closureType: IrType = {
        kind: 'function',
        parameters: [accumulatorType, ownerType.element],
        returns: accumulatorType,
      };
      return `${parenthesize(owner)}.iter().cloned().fold(${emitExpression(initial, context)}, ${emitClosure(callback, context, closureType, false)})`;
    }
  }
  const arguments_ = expression.arguments.map((argument) => emitExpression(argument, context));
  const callee = emitExpression(expression.callee, context);
  return `${expression.callee.kind === 'property' ? parenthesize(callee) : callee}(${arguments_.join(', ')})`;
}

function emitInlineFunctionCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
): string {
  if (declaration.body.some((statement) => containsStatementKind(statement, 'return'))) {
    throw new RustEmissionError(`inline dependency ${declaration.name} contains a return statement`);
  }
  const bindings = new Map<string, IrExpression>();
  declaration.parameters.forEach((parameter, index) => {
    const argument = expression.arguments[index];
    if (argument) bindings.set(parameter.name, argument);
  });
  const body = substituteIdentifiers(declaration.body, bindings);
  return emitStatementsAsBlock(body, context);
}

function emitKnownFunctionCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
): string {
  const restIndex = declaration.parameters.findIndex((parameter) => parameter.rest);
  if (restIndex < 0) {
    const arguments_ = expression.arguments.map((argument, index) =>
      emitExpression(argument, context, declaration.parameters[index]?.type),
    );
    return `${snakeCase(declaration.name)}(${arguments_.join(', ')})`;
  }
  const fixed = expression.arguments
    .slice(0, restIndex)
    .map((argument, index) => emitExpression(argument, context, declaration.parameters[index]?.type));
  const rest = declaration.parameters[restIndex]!;
  const element = rest.type.kind === 'array' ? rest.type.element : undefined;
  const values = expression.arguments.slice(restIndex).map((argument) => emitExpression(argument, context, element));
  return `${snakeCase(declaration.name)}(${[...fixed, `vec![${values.join(', ')}]`].join(', ')})`;
}

function emitMathCall(method: string, arguments_: string[]): string {
  const first = arguments_[0];
  if (!first) throw new RustEmissionError(`Math.${method} requires an argument`);
  switch (method) {
    case 'abs':
    case 'asin':
    case 'ceil':
    case 'cos':
    case 'floor':
    case 'round':
    case 'sin':
    case 'sqrt':
      return `${parenthesize(first)}.${method}()`;
    case 'max':
    case 'min':
      return arguments_.slice(1).reduce((value, item) => `${parenthesize(value)}.${method}(${item})`, first);
    case 'imul':
      if (!arguments_[1]) throw new RustEmissionError('Math.imul requires two arguments');
      return `__flight_js_to_i32(${first}).wrapping_mul(__flight_js_to_i32(${arguments_[1]})) as f64`;
    case 'pow':
      if (!arguments_[1]) throw new RustEmissionError('Math.pow requires two arguments');
      return `${parenthesize(first)}.powf(${arguments_[1]})`;
    default:
      throw new RustEmissionError(`Math.${method} Rust lowering is not implemented`);
  }
}

function emitProperty(expression: Extract<IrExpression, { kind: 'property' }>, context: EmitContext): string {
  if (expression.optional) throw new RustEmissionError('optional property Rust lowering is not implemented');
  if (expression.object.kind === 'identifier') {
    if (expression.object.name === 'Math' && expression.name === 'PI') return 'std::f64::consts::PI';
    if (expression.object.name === 'Float' && expression.name === 'INFINITY') return 'f64::INFINITY';
    if (expression.object.name === 'Float' && expression.name === 'NAN') return 'f64::NAN';
  }
  const objectType = inferIrExpressionType(expression.object, context);
  if (objectType?.kind === 'array' && expression.name === 'length') {
    return `(${emitPlaceExpression(expression.object, context)}.len() as f64)`;
  }
  if (objectType?.kind === 'named' && typedArrayType(objectType.name) && expression.name === 'length') {
    return `(${emitPlaceExpression(expression.object, context)}.len() as f64)`;
  }
  const place = emitPropertyPlace(expression, context);
  const type = inferIrExpressionType(expression, context);
  return type && !isCopyType(type, context) ? `${parenthesize(place)}.clone()` : place;
}

function emitPropertyPlace(expression: Extract<IrExpression, { kind: 'property' }>, context: EmitContext): string {
  return `${emitPlaceExpression(expression.object, context)}.${safeName(expression.name)}`;
}

function emitPlaceExpression(expression: IrExpression, context: EmitContext): string {
  switch (expression.kind) {
    case 'identifier':
      return emitIdentifier(expression.name, context);
    case 'property':
      return emitPropertyPlace(expression, context);
    case 'element':
      return emitElement(expression, context);
    default:
      return emitExpression(expression, context);
  }
}

function emitBinary(expression: Extract<IrExpression, { kind: 'binary' }>, context: EmitContext): string {
  const left = emitExpression(expression.left, context);
  const leftType = inferIrExpressionType(expression.left, context);
  const right = emitExpression(
    expression.right,
    context,
    expression.operator === '??' || expression.operator === '??undefined'
      ? leftType?.kind === 'nullable'
        ? leftType.inner
        : leftType
      : undefined,
  );
  if (
    expression.right.kind === 'literal' &&
    expression.right.value === null &&
    (expression.operator === '===' || expression.operator === '!==')
  ) {
    return `${parenthesize(left)}.${expression.operator === '===' ? 'is_none' : 'is_some'}()`;
  }
  if (expression.operator === '**') return `${parenthesize(left)}.powf(${right})`;
  if (expression.operator === '>>>') {
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '>>' || expression.operator === '<<') {
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '&' || expression.operator === '|' || expression.operator === '^') {
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '??' || expression.operator === '??undefined') {
    return `${parenthesize(left)}.unwrap_or(${right})`;
  }
  if (
    (expression.operator === '&&' || expression.operator === '||') &&
    inferIrExpressionType(expression.left, context)?.kind === 'primitive' &&
    inferIrExpressionType(expression.left, context)?.kind === 'primitive' &&
    (inferIrExpressionType(expression.left, context) as Extract<IrType, { kind: 'primitive' }>).name !== 'Bool'
  ) {
    const condition = `${parenthesize(left)} != 0.0_f64`;
    return expression.operator === '&&'
      ? `if ${condition} { ${right} } else { ${left} }`
      : `if ${condition} { ${left} } else { ${right} }`;
  }
  const operator =
    expression.operator === '==='
      ? '=='
      : expression.operator === '!=='
        ? '!='
        : expression.operator === '&&'
          ? '&&'
          : expression.operator === '||'
            ? '||'
            : expression.operator;
  return `(${left} ${operator} ${right})`;
}

function emitAssignment(expression: Extract<IrExpression, { kind: 'assignment' }>, context: EmitContext): string {
  const left =
    expression.left.kind === 'element'
      ? emitElement(expression.left, context)
      : emitPlaceExpression(expression.left, context);
  const right = emitExpression(expression.right, context, inferIrExpressionType(expression.left, context));
  const assignment = emitAssignmentOperation(left, right, expression.operator);
  return `{ ${assignment}; ${left} }`;
}

function emitAssignmentStatement(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
): string {
  const left =
    expression.left.kind === 'element'
      ? emitElement(expression.left, context)
      : emitPlaceExpression(expression.left, context);
  const right = emitExpression(expression.right, context, inferIrExpressionType(expression.left, context));
  return emitAssignmentOperation(left, right, expression.operator);
}

function emitAssignmentOperation(left: string, right: string, operator: string): string {
  const bitwise = new Set(['&=', '|=', '^=', '<<=', '>>=', '>>>=']);
  return bitwise.has(operator)
    ? `${left} = ${emitBitwiseOperation(left, right, operator.slice(0, -1))}`
    : `${left} ${operator} ${right}`;
}

function emitBitwiseOperation(left: string, right: string, operator: string): string {
  const shift = `(__flight_js_to_u32(${right}) & 31)`;
  if (operator === '>>>') return `(__flight_js_to_u32(${left}) >> ${shift}) as f64`;
  if (operator === '>>') return `(__flight_js_to_i32(${left}) >> ${shift}) as f64`;
  if (operator === '<<') return `__flight_js_to_i32(${left}).wrapping_shl(${shift}) as f64`;
  return `(__flight_js_to_i32(${left}) ${operator} __flight_js_to_i32(${right})) as f64`;
}

function emitUnary(expression: Extract<IrExpression, { kind: 'unary' }>, context: EmitContext): string {
  if (expression.operator === '++' || expression.operator === '--') {
    const operand = emitPlaceExpression(expression.operand, context);
    const operator = expression.operator === '++' ? '+=' : '-=';
    return `{ ${operand} ${operator} 1.0; ${operand} }`;
  }
  const operand = emitExpression(expression.operand, context);
  if (expression.operator === 'void') return `{ ${operand}; () }`;
  if (expression.operator === 'typeof' || expression.operator === 'delete') {
    throw new RustEmissionError(`${expression.operator} Rust lowering is not implemented`);
  }
  return `(${expression.operator}${operand})`;
}

function emitClosure(
  expression: Extract<IrExpression, { kind: 'function' }>,
  context: EmitContext,
  expectedType?: IrType | undefined,
  wrapFunction = true,
): string {
  const callback =
    expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
      ? true
      : expectedType?.kind === 'function';
  const returns =
    expression.returns ??
    (expectedType?.kind === 'function'
      ? expectedType.returns
      : expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
        ? primitive('Float')
        : primitive('Float'));
  const nextContext = functionContext(context, expression.name ?? 'closure', expression, returns);
  const fallbackParameter =
    expectedType?.kind === 'function'
      ? expectedType.parameters
      : expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
        ? [primitive('Float')]
        : [];
  registerParameters(expression.parameters, nextContext, fallbackParameter);
  const parameters = expression.parameters.map((parameter, index) =>
    emitParameter(parameter, nextContext, fallbackParameter[index], expression, false),
  );
  const body = expression.expression
    ? `{ ${emitExpression(expression.expression, nextContext, returns)} }`
    : emitStatementsAsBlock(expression.body, nextContext);
  const closure = `move |${parameters.join(', ')}| -> ${emitType(returns, nextContext)} ${body}`;
  return callback && wrapFunction ? `std::sync::Arc::new(${closure})` : closure;
}

function emitType(type: IrType, context: EmitContext): string {
  switch (type.kind) {
    case 'anonymous': {
      const name = context.anonymousTypes.get(typeKey(type));
      if (!name) throw new RustEmissionError('anonymous structural type has no synthesized Rust identity');
      return name;
    }
    case 'array':
      return `Vec<${emitType(type.element, context)}>`;
    case 'dynamic':
      return 'crate::OpaqueHostValue';
    case 'function':
      return `std::sync::Arc<dyn Fn(${type.parameters.map((item) => emitType(item, context)).join(', ')}) -> ${emitType(type.returns, context)} + Send + Sync + 'static>`;
    case 'named': {
      const typedArray = typedArrayType(type.name);
      if (typedArray) return `Vec<${typedArray.rust}>`;
      if (type.name === 'RustF32') return 'f32';
      if (type.name === 'RustF64') return 'f64';
      if (type.name === 'RustI8') return 'i8';
      if (type.name === 'RustI16') return 'i16';
      if (type.name === 'RustI32') return 'i32';
      if (type.name === 'RustU8') return 'u8';
      if (type.name === 'RustU16') return 'u16';
      if (type.name === 'RustU32') return 'u32';
      return `${type.name}${type.arguments.length > 0 ? `<${type.arguments.map((item) => emitType(item, context)).join(', ')}>` : ''}`;
    }
    case 'nullable':
      return `Option<${emitType(type.inner, context)}>`;
    case 'primitive':
      switch (type.name) {
        case 'Bool':
          return 'bool';
        case 'Float':
          return 'f64';
        case 'Int':
          return 'i32';
        case 'String':
          return 'String';
        case 'Void':
          return '()';
      }
  }
}

function emitTypeDeclaration(name: string, exported: boolean, type: IrType, context: EmitContext): string {
  const visibility = exported ? 'pub ' : '';
  if (type.kind !== 'anonymous') {
    return `${visibility}type ${name} = ${emitType(type, context)};`;
  }
  const fields = flattenStructFields(type, context);
  return [
    '#[derive(Clone)]',
    `${visibility}struct ${name} {`,
    indent(
      fields
        .map(
          (field) =>
            `pub ${safeName(field.name)}: ${field.optional ? `Option<${emitType(field.type, context)}>` : emitType(field.type, context)},`,
        )
        .join('\n'),
    ),
    '}',
  ].join('\n');
}

function flattenStructFields(
  type: Extract<IrType, { kind: 'anonymous' }>,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): Extract<IrType, { kind: 'anonymous' }>['fields'] {
  const inherited = type.extends.flatMap((base) => {
    if (base.kind === 'anonymous') return flattenStructFields(base, context, visited);
    if (base.kind !== 'named' || visited.has(base.name)) return [];
    const resolved = context.namedTypes.get(base.name);
    return resolved?.kind === 'anonymous'
      ? flattenStructFields(resolved, context, new Set([...visited, base.name]))
      : [];
  });
  return [...new Map([...inherited, ...type.fields].map((field) => [field.name, field])).values()];
}

function functionContext(context: EmitContext, ownerName: string, owner: unknown, returns: IrType): EmitContext {
  const anonymousTypes = new Map(context.anonymousTypes);
  let index = anonymousTypes.size + 1;
  for (const type of collectAnonymousTypes(owner)) {
    const key = typeKey(type);
    if (!anonymousTypes.has(key)) {
      anonymousTypes.set(key, `${pascalCase(ownerName)}Record${String(index++)}`);
    }
  }
  return {
    ...context,
    anonymousTypes,
    continueEpilogue: [],
    currentReturnType: returns,
    mutatedNames: collectMutatedNames(owner, context.mutatingFunctions),
    symbolTypes: new Map(context.symbolTypes),
  };
}

function registerParameters(parameters: IrParameter[], context: EmitContext, fallbackTypes: IrType[] = []): void {
  parameters.forEach((parameter, index) => {
    const type = parameter.type.kind === 'dynamic' && fallbackTypes[index] ? fallbackTypes[index]! : parameter.type;
    context.symbolTypes.set(
      parameter.name,
      parameter.optional && !parameter.initializer ? { inner: type, kind: 'nullable' } : type,
    );
  });
}

function emitAnonymousDefinitions(context: EmitContext): string {
  if (context.anonymousTypes.size === 0) return '';
  const definitions = [...context.anonymousTypes.entries()].map(([key, name]) => {
    const type = JSON.parse(key) as IrType;
    if (type.kind !== 'anonymous') throw new RustEmissionError(`invalid anonymous type identity ${name}`);
    return [
      '#[derive(Clone)]',
      `struct ${name} {`,
      indent(
        type.fields
          .map(
            (field) =>
              `${safeName(field.name)}: ${field.optional ? `Option<${emitType(field.type, context)}>` : emitType(field.type, context)},`,
          )
          .join('\n'),
      ),
      '}',
    ].join('\n');
  });
  return `${definitions.join('\n\n')}\n\n`;
}

function collectAnonymousTypes(value: unknown): IrType[] {
  const found = new Map<string, IrType>();
  const visit = (item: unknown): void => {
    if (!item || typeof item !== 'object') return;
    if ('kind' in item && item.kind === 'anonymous' && 'fields' in item && 'extends' in item) {
      const type = item as IrType;
      found.set(typeKey(type), type);
    }
    for (const child of Object.values(item)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
  return [...found.values()];
}

function collectMutatingFunctionParameters(declarations: IrDeclaration[]): ReadonlyMap<string, ReadonlySet<number>> {
  const functions = declarations.filter(
    (declaration): declaration is IrFunctionDeclaration => declaration.kind === 'function',
  );
  const signatures = new Map<string, ReadonlySet<number>>();
  for (let pass = 0; pass <= functions.length; pass++) {
    let changed = false;
    for (const declaration of functions) {
      const mutated = collectMutatedNames(declaration, signatures);
      const indexes = new Set(
        declaration.parameters.flatMap((parameter, index) => (mutated.has(parameter.name) ? [index] : [])),
      );
      const previous = signatures.get(declaration.name);
      if (!previous || [...indexes].some((index) => !previous.has(index))) {
        signatures.set(declaration.name, indexes);
        changed = true;
      }
    }
    if (!changed) break;
  }
  return signatures;
}

function collectMutatedNames(
  value: unknown,
  mutatingFunctions: ReadonlyMap<string, ReadonlySet<number>> = new Map(),
): ReadonlySet<string> {
  const names = new Set<string>();
  const aliases = new Map<string, string>();
  const visit = (item: unknown): void => {
    if (!item || typeof item !== 'object') return;
    if (
      'name' in item &&
      typeof item.name === 'string' &&
      'initializer' in item &&
      item.initializer &&
      typeof item.initializer === 'object'
    ) {
      const root = expressionRootIdentifier(item.initializer);
      if (root && root !== item.name) aliases.set(item.name, root);
    }
    if ('kind' in item && item.kind === 'assignment' && 'left' in item) {
      const root = expressionRootIdentifier(item.left);
      if (root) names.add(root);
    }
    if (
      'kind' in item &&
      item.kind === 'unary' &&
      'operator' in item &&
      (item.operator === '++' || item.operator === '--') &&
      'operand' in item
    ) {
      const root = expressionRootIdentifier(item.operand);
      if (root) names.add(root);
    }
    if (
      'kind' in item &&
      item.kind === 'call' &&
      'callee' in item &&
      item.callee &&
      typeof item.callee === 'object' &&
      'kind' in item.callee &&
      item.callee.kind === 'property' &&
      'name' in item.callee &&
      typeof item.callee.name === 'string' &&
      mutationMethods.has(item.callee.name) &&
      'object' in item.callee
    ) {
      const root = expressionRootIdentifier(item.callee.object);
      if (root) names.add(root);
    }
    if (
      'kind' in item &&
      item.kind === 'call' &&
      'callee' in item &&
      item.callee &&
      typeof item.callee === 'object' &&
      'kind' in item.callee &&
      item.callee.kind === 'identifier' &&
      'name' in item.callee &&
      typeof item.callee.name === 'string' &&
      'arguments' in item &&
      Array.isArray(item.arguments)
    ) {
      for (const index of mutatingFunctions.get(item.callee.name) ?? []) {
        const root = expressionRootIdentifier(item.arguments[index]);
        if (root) names.add(root);
      }
    }
    for (const child of Object.values(item)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
  let changed = true;
  while (changed) {
    changed = false;
    for (const [alias, root] of aliases) {
      if (names.has(alias) && !names.has(root)) {
        names.add(root);
        changed = true;
      }
    }
  }
  return names;
}

function expressionRootIdentifier(value: unknown): string | undefined {
  if (!value || typeof value !== 'object' || !('kind' in value)) return undefined;
  if (value.kind === 'identifier' && 'name' in value && typeof value.name === 'string') return value.name;
  if ((value.kind === 'element' || value.kind === 'property') && 'object' in value) {
    return expressionRootIdentifier(value.object);
  }
  return undefined;
}

function capturesParameterInReturnedClosure(owner: IrExpression | IrFunctionDeclaration, name: string): boolean {
  if (!('body' in owner) || !Array.isArray(owner.body)) return false;
  return owner.body.some(
    (statement) =>
      statement.kind === 'return' &&
      statement.expression?.kind === 'function' &&
      containsIdentifier(statement.expression, name),
  );
}

function containsIdentifier(value: unknown, name: string): boolean {
  if (!value || typeof value !== 'object') return false;
  if ('kind' in value && value.kind === 'identifier' && 'name' in value && value.name === name) {
    return true;
  }
  return Object.values(value).some((item) =>
    Array.isArray(item) ? item.some((child) => containsIdentifier(child, name)) : containsIdentifier(item, name),
  );
}

function containsStatementKind(value: unknown, kind: IrStatement['kind']): boolean {
  if (!value || typeof value !== 'object') return false;
  if ('kind' in value && value.kind === kind) return true;
  return Object.values(value).some((item) =>
    Array.isArray(item) ? item.some((child) => containsStatementKind(child, kind)) : containsStatementKind(item, kind),
  );
}

function substituteIdentifiers<T>(value: T, bindings: ReadonlyMap<string, IrExpression>): T {
  if (!value || typeof value !== 'object') return value;
  if ('kind' in value && value.kind === 'identifier' && 'name' in value && typeof value.name === 'string') {
    return (bindings.get(value.name) ?? value) as T;
  }
  if (Array.isArray(value)) {
    return value.map((item) => substituteIdentifiers(item, bindings)) as T;
  }
  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [key, substituteIdentifiers(item, bindings)]),
  ) as T;
}

function isReferenceLike(type: IrType, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context);
  return (
    resolved?.kind === 'anonymous' ||
    resolved?.kind === 'array' ||
    (resolved?.kind === 'named' && Boolean(typedArrayType(resolved.name)))
  );
}

function isCopyType(type: IrType, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context);
  if (!resolved) return false;
  if (resolved.kind === 'primitive') return resolved.name !== 'String';
  if (resolved.kind === 'nullable') return isCopyType(resolved.inner, context);
  return false;
}

function emitNew(expression: Extract<IrExpression, { kind: 'new' }>, context: EmitContext): string {
  const globalType = runtimeGlobalType(expression.callee);
  const typedArray = globalType ? typedArrayType(globalType) : undefined;
  if (typedArray) {
    const length = expression.arguments[0] ? emitExpression(expression.arguments[0], context) : '0.0_f64';
    return `vec![${typedArray.zero}; ${parenthesize(length)} as usize]`;
  }
  throw new RustEmissionError(
    `new-expression Rust lowering is not implemented: ${emitExpression(expression.callee, context)}`,
  );
}

function emitObject(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const nullable = expectedType?.kind === 'nullable';
  const target = nullable ? expectedType.inner : expectedType;
  const resolved = resolveSemanticType(target, context);
  if (resolved?.kind !== 'anonymous' || !target) {
    throw new RustEmissionError('object literal requires an inferred structural type');
  }
  const name = emitType(target, context);
  const fields = new Map(resolved.fields.map((field) => [field.name, field]));
  const properties = expression.properties.map((property) => {
    if (property.kind !== 'property') {
      throw new RustEmissionError(`object ${property.kind} Rust lowering is not implemented`);
    }
    const field = fields.get(property.name);
    if (!field) throw new RustEmissionError(`object field ${property.name} is not present in structural type`);
    return `${safeName(property.name)}: ${emitExpression(property.value, context, field.type)},`;
  });
  const value = `${name} {\n${indent(properties.join('\n'))}\n}`;
  return nullable ? `Some(${value})` : value;
}

function emitElement(expression: Extract<IrExpression, { kind: 'element' }>, context: EmitContext): string {
  if (expression.optional) throw new RustEmissionError('optional element access Rust lowering is not implemented');
  const objectType = inferIrExpressionType(expression.object, context);
  const nullableCollection =
    objectType?.kind === 'nullable' &&
    (objectType.inner.kind === 'array' ||
      (objectType.inner.kind === 'named' && Boolean(typedArrayType(objectType.inner.name))));
  const root = expressionRootIdentifier(expression.object);
  const object = nullableCollection
    ? `${emitPlaceExpression(expression.object, context)}.${root && context.mutatedNames.has(root) ? 'as_mut' : 'as_ref'}().unwrap()`
    : emitPlaceExpression(expression.object, context);
  return `${object}[${emitExpression(expression.index, context)} as usize]`;
}

function emitElementRead(expression: Extract<IrExpression, { kind: 'element' }>, context: EmitContext): string {
  const place = emitElement(expression, context);
  const objectType = inferIrExpressionType(expression.object, context);
  return objectType?.kind === 'named' && typedArrayType(objectType.name) ? `(${place} as f64)` : `${place}.clone()`;
}

function inferIrExpressionType(expression: IrExpression, context: EmitContext): IrType | undefined {
  switch (expression.kind) {
    case 'identifier':
      return context.symbolTypes.get(expression.name);
    case 'literal':
      if (typeof expression.value === 'number') return primitive('Float');
      if (typeof expression.value === 'boolean') return primitive('Bool');
      if (typeof expression.value === 'string') return primitive('String');
      return undefined;
    case 'array':
      return expression.elements[0]
        ? { element: inferIrExpressionType(expression.elements[0], context) ?? { kind: 'dynamic' }, kind: 'array' }
        : undefined;
    case 'binary': {
      const left = inferIrExpressionType(expression.left, context);
      if (['===', '!==', '<', '<=', '>', '>='].includes(expression.operator)) {
        return primitive('Bool');
      }
      if (expression.operator === '&&' || expression.operator === '||') return left;
      if (expression.operator === '??' || expression.operator === '??undefined') {
        return left?.kind === 'nullable' ? left.inner : left;
      }
      return left ?? inferIrExpressionType(expression.right, context);
    }
    case 'conditional':
      return (
        inferIrExpressionType(expression.whenTrue, context) ?? inferIrExpressionType(expression.whenFalse, context)
      );
    case 'call':
      if (isArrayFillConstruction(expression) && expression.arguments[0]) {
        return {
          element: inferIrExpressionType(expression.arguments[0], context) ?? { kind: 'dynamic' },
          kind: 'array',
        };
      }
      return undefined;
    case 'element': {
      const inferred = inferIrExpressionType(expression.object, context);
      const object = inferred?.kind === 'nullable' ? inferred.inner : inferred;
      if (object?.kind === 'array') return object.element;
      if (object?.kind === 'named') return typedArrayElementType(object.name);
      return undefined;
    }
    case 'property': {
      const object = inferIrExpressionType(expression.object, context);
      if (object?.kind === 'anonymous') return object.fields.find((field) => field.name === expression.name)?.type;
      if (object?.kind === 'array' && expression.name === 'length') return primitive('Float');
      if (object?.kind === 'named' && expression.name === 'length' && typedArrayType(object.name)) {
        return primitive('Float');
      }
      if (object?.kind === 'named') {
        const shape = context.namedTypes.get(object.name);
        if (shape?.kind === 'anonymous') {
          const field = shape.fields.find((item) => item.name === expression.name);
          if (field) return field.optional ? { inner: field.type, kind: 'nullable' } : field.type;
        }
      }
      return undefined;
    }
    case 'new': {
      const name = runtimeGlobalType(expression.callee);
      return name && typedArrayType(name) ? { arguments: [], kind: 'named', name } : undefined;
    }
    default:
      return undefined;
  }
}

function isArrayFillConstruction(expression: Extract<IrExpression, { kind: 'call' }>): expression is Extract<
  IrExpression,
  { kind: 'call' }
> & {
  callee: Extract<IrExpression, { kind: 'property' }> & {
    object: Extract<IrExpression, { kind: 'new' }>;
  };
} {
  return (
    expression.callee.kind === 'property' &&
    expression.callee.name === 'fill' &&
    expression.callee.object.kind === 'new' &&
    expression.callee.object.callee.kind === 'identifier' &&
    expression.callee.object.callee.name === 'Array'
  );
}

function runtimeGlobalType(expression: IrExpression): string | undefined {
  if (
    expression.kind !== 'call' ||
    expression.callee.kind !== 'property' ||
    expression.callee.name !== 'globalValue' ||
    expression.callee.object.kind !== 'identifier' ||
    expression.callee.object.name !== '_Runtime' ||
    expression.arguments[0]?.kind !== 'literal' ||
    typeof expression.arguments[0].value !== 'string'
  ) {
    return undefined;
  }
  return expression.arguments[0].value;
}

function typedArrayType(name: string): { ir: string; rust: string; zero: string } | undefined {
  return typedArrays[name];
}

function typedArrayElementType(name: string): IrType | undefined {
  const typedArray = typedArrayType(name);
  return typedArray ? { arguments: [], kind: 'named', name: typedArray.ir } : undefined;
}

function coerceExpression(value: string, expectedType?: IrType): string {
  if (expectedType?.kind !== 'named') return value;
  const casts: Readonly<Record<string, string>> = {
    RustF32: 'f32',
    RustI8: 'i8',
    RustI16: 'i16',
    RustI32: 'i32',
    RustU8: 'u8',
    RustU16: 'u16',
    RustU32: 'u32',
  };
  const cast = casts[expectedType.name];
  return cast ? `${parenthesize(value)} as ${cast}` : value;
}

function inferExpressionType(expression: IrExpression): string {
  if (expression.kind === 'literal') {
    if (typeof expression.value === 'number') return 'f64';
    if (typeof expression.value === 'boolean') return 'bool';
    if (typeof expression.value === 'string') return "&'static str";
  }
  return 'f64';
}

function emitThrowMessage(expression: IrExpression, context: EmitContext): string {
  if (
    expression.kind === 'new' &&
    expression.callee.kind === 'identifier' &&
    expression.callee.name === 'Error' &&
    expression.arguments.length === 1
  ) {
    return emitExpression(expression.arguments[0]!, context);
  }
  return JSON.stringify('generated Flight function threw');
}

function emitIdentifier(name: string, context: EmitContext): string {
  return context.constantNames.get(name) ?? safeName(name);
}

function emitCondition(expression: IrExpression, context: EmitContext): string {
  const emitted = emitExpression(expression, context);
  const type = inferIrExpressionType(expression, context);
  if (type?.kind === 'primitive' && type.name === 'Bool') return emitted;
  if (type?.kind === 'primitive' && (type.name === 'Float' || type.name === 'Int')) {
    return `${parenthesize(emitted)} != 0.0_f64`;
  }
  if (type?.kind === 'nullable') return `${parenthesize(emitted)}.is_some()`;
  return emitted;
}

function emitLiteral(value: boolean | null | number | string, expectedType?: IrType, context?: EmitContext): string {
  if (value === null) return 'None';
  if (typeof value === 'string') {
    const literal = JSON.stringify(value);
    const resolved = context ? resolveSemanticType(expectedType, context) : expectedType;
    return resolved?.kind === 'primitive' && resolved.name === 'String' ? `${literal}.to_owned()` : literal;
  }
  if (typeof value === 'boolean') return String(value);
  if (Number.isNaN(value)) return 'f64::NAN';
  if (!Number.isFinite(value)) return value > 0 ? 'f64::INFINITY' : 'f64::NEG_INFINITY';
  const number = Number.isInteger(value)
    ? `${String(value)}.0_f64`
    : `${/[.eE]/u.test(String(value)) ? String(value) : `${String(value)}.0`}_f64`;
  return coerceExpression(number, expectedType);
}

function resolveSemanticType(type: IrType | undefined, context: EmitContext): IrType | undefined {
  const visited = new Set<string>();
  let resolved = type;
  while (resolved?.kind === 'named' && !visited.has(resolved.name)) {
    visited.add(resolved.name);
    resolved = context.namedTypes.get(resolved.name) ?? resolved;
    if (resolved.kind === 'named' && visited.has(resolved.name)) break;
  }
  return resolved;
}

function evaluateConstant(expression: IrExpression, values: ReadonlyMap<string, number>): number | undefined {
  switch (expression.kind) {
    case 'literal':
      return typeof expression.value === 'number' ? expression.value : undefined;
    case 'identifier':
      return values.get(expression.name);
    case 'property':
      return expression.object.kind === 'identifier' && expression.object.name === 'Math' && expression.name === 'PI'
        ? Math.PI
        : undefined;
    case 'unary': {
      const operand = evaluateConstant(expression.operand, values);
      if (operand === undefined) return undefined;
      if (expression.operator === '-') return -operand;
      if (expression.operator === '+') return operand;
      return undefined;
    }
    case 'binary': {
      const left = evaluateConstant(expression.left, values);
      const right = evaluateConstant(expression.right, values);
      if (left === undefined || right === undefined) return undefined;
      switch (expression.operator) {
        case '+':
          return left + right;
        case '-':
          return left - right;
        case '*':
          return left * right;
        case '/':
          return left / right;
        case '%':
          return left % right;
        case '**':
          return left ** right;
        default:
          return undefined;
      }
    }
    case 'call': {
      if (
        expression.callee.kind !== 'property' ||
        expression.callee.object.kind !== 'identifier' ||
        expression.callee.object.name !== 'Math'
      ) {
        return undefined;
      }
      const arguments_ = expression.arguments.map((argument) => evaluateConstant(argument, values));
      if (arguments_.some((argument) => argument === undefined)) return undefined;
      const numbers = arguments_ as number[];
      switch (expression.callee.name) {
        case 'abs':
          return Math.abs(numbers[0]!);
        case 'asin':
          return Math.asin(numbers[0]!);
        case 'cos':
          return Math.cos(numbers[0]!);
        case 'max':
          return Math.max(...numbers);
        case 'min':
          return Math.min(...numbers);
        case 'pow':
          return Math.pow(numbers[0]!, numbers[1]!);
        case 'sin':
          return Math.sin(numbers[0]!);
        case 'sqrt':
          return Math.sqrt(numbers[0]!);
        default:
          return undefined;
      }
    }
    default:
      return undefined;
  }
}

function primitive(name: 'Bool' | 'Float' | 'Int' | 'String' | 'Void'): IrType {
  return { kind: 'primitive', name };
}

function assignsName(value: unknown, name: string): boolean {
  if (!value || typeof value !== 'object') return false;
  if (
    'kind' in value &&
    value.kind === 'assignment' &&
    'left' in value &&
    value.left &&
    typeof value.left === 'object' &&
    'kind' in value.left &&
    value.left.kind === 'identifier' &&
    'name' in value.left &&
    value.left.name === name
  ) {
    return true;
  }
  if (
    'kind' in value &&
    value.kind === 'unary' &&
    'operator' in value &&
    (value.operator === '++' || value.operator === '--') &&
    'operand' in value &&
    value.operand &&
    typeof value.operand === 'object' &&
    'kind' in value.operand &&
    value.operand.kind === 'identifier' &&
    'name' in value.operand &&
    value.operand.name === name
  ) {
    return true;
  }
  return Object.values(value).some((item) =>
    Array.isArray(item) ? item.some((child) => assignsName(child, name)) : assignsName(item, name),
  );
}

function parenthesize(value: string): string {
  return `(${value})`;
}

function typeKey(type: IrType): string {
  return JSON.stringify(type);
}

function indent(value: string): string {
  return value
    .split('\n')
    .map((line) => (line.length > 0 ? `  ${line}` : line))
    .join('\n');
}

export function snakeCase(value: string): string {
  const separated = value
    .replace(/([a-z0-9])([A-Z])/gu, '$1_$2')
    .replace(/([A-Z]+)([A-Z][a-z])/gu, '$1_$2')
    .replace(/[-\s]+/gu, '_')
    .toLowerCase();
  return rustKeywords.has(separated) ? `${separated}_` : separated;
}

function screamingSnakeCase(value: string): string {
  return snakeCase(value).toUpperCase();
}

function safeName(value: string): string {
  return snakeCase(value);
}

function pascalCase(value: string): string {
  return snakeCase(value)
    .split('_')
    .filter(Boolean)
    .map((part) => `${part[0]?.toUpperCase() ?? ''}${part.slice(1)}`)
    .join('');
}

const typedArrays: Readonly<Record<string, { ir: string; rust: string; zero: string }>> = {
  Float32Array: { ir: 'RustF32', rust: 'f32', zero: '0.0_f32' },
  Float64Array: { ir: 'RustF64', rust: 'f64', zero: '0.0_f64' },
  Int8Array: { ir: 'RustI8', rust: 'i8', zero: '0_i8' },
  Int16Array: { ir: 'RustI16', rust: 'i16', zero: '0_i16' },
  Int32Array: { ir: 'RustI32', rust: 'i32', zero: '0_i32' },
  Uint8Array: { ir: 'RustU8', rust: 'u8', zero: '0_u8' },
  Uint8ClampedArray: { ir: 'RustU8', rust: 'u8', zero: '0_u8' },
  Uint16Array: { ir: 'RustU16', rust: 'u16', zero: '0_u16' },
  Uint32Array: { ir: 'RustU32', rust: 'u32', zero: '0_u32' },
};

const mutationMethods = new Set(['copyWithin', 'fill', 'pop', 'push', 'reverse', 'shift', 'sort', 'splice', 'unshift']);

const rustKeywords = new Set([
  'as',
  'async',
  'await',
  'break',
  'const',
  'continue',
  'crate',
  'dyn',
  'else',
  'enum',
  'extern',
  'false',
  'fn',
  'for',
  'if',
  'impl',
  'in',
  'let',
  'loop',
  'match',
  'mod',
  'move',
  'mut',
  'pub',
  'ref',
  'return',
  'self',
  'Self',
  'static',
  'struct',
  'super',
  'trait',
  'true',
  'type',
  'unsafe',
  'use',
  'where',
  'while',
]);

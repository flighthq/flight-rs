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
  source: string;
  typeImports: string[];
}

interface EmitContext {
  callbackTypes: ReadonlySet<string>;
  constantNames: ReadonlyMap<string, string>;
  constantValues: ReadonlyMap<string, number>;
  currentReturnType?: IrType | undefined;
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
  const context: EmitContext = {
    callbackTypes: new Set(['EasingFunction', 'ScalarRemap']),
    constantNames,
    constantValues,
  };
  const declarations = module.declarations.map((declaration) => emitDeclaration(declaration, context)).join('\n\n');
  const usedImports = [...new Set(module.typeImports)]
    .filter((name) => new RegExp(`\\b${name}\\b`, 'u').test(declarations))
    .sort();
  const imports = usedImports.length > 0 ? `use crate::{${usedImports.join(', ')}};\n\n` : '';
  return [
    `// @generated from ${module.source}; do not edit.`,
    '#![allow(clippy::excessive_precision)]',
    '#![allow(non_upper_case_globals)]',
    '#![allow(unused_braces)]',
    '#![allow(unused_imports)]',
    '#![allow(unused_parens)]',
    '',
    `${imports}${declarations}`,
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
    case 'class':
    case 'enum':
    case 'type':
      throw new RustEmissionError(
        `${declaration.origin.source}:${String(declaration.origin.line)}: unsupported Rust declaration ${declaration.kind} ${declaration.name}`,
      );
  }
}

function emitTopLevelVariable(declaration: IrVariableDeclaration, context: EmitContext): string {
  if (!declaration.initializer) {
    throw new RustEmissionError(`${declaration.origin.source}: uninitialized top-level variable ${declaration.name}`);
  }
  if (declaration.initializer.kind === 'function') {
    return emitLiftedFunction(declaration, declaration.initializer, context);
  }
  const visibility = declaration.exported ? 'pub ' : '';
  const name = context.constantNames.get(declaration.name) ?? screamingSnakeCase(declaration.name);
  const type = declaration.type ? emitType(declaration.type) : inferExpressionType(declaration.initializer);
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
  const nextContext = { ...context, currentReturnType: returns };
  const parameters = expression.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, callback ? primitive('Float') : undefined, expression),
  );
  const body = expression.expression
    ? `{\n${indent(`return ${emitExpression(expression.expression, nextContext, returns)};`)}\n}`
    : emitStatementsAsBlock(expression.body, nextContext);
  return `${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> ${emitType(returns)} ${body}`;
}

function emitFunctionDeclaration(declaration: IrFunctionDeclaration, context: EmitContext): string {
  if (declaration.async) {
    throw new RustEmissionError(
      `${declaration.origin.source}: async Rust lowering is not implemented for ${declaration.name}`,
    );
  }
  const nextContext = { ...context, currentReturnType: declaration.returns };
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
  return `${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> ${emitType(declaration.returns)} ${body}`;
}

function emitParameter(
  parameter: IrParameter,
  context: EmitContext,
  fallbackType: IrType | undefined,
  owner: IrExpression | IrFunctionDeclaration,
): string {
  const assigned = assignsName(owner, parameter.name);
  const name = `${assigned ? 'mut ' : ''}${safeName(parameter.name)}`;
  const type = parameter.type.kind === 'dynamic' && fallbackType ? fallbackType : parameter.type;
  const emitted = emitType(type);
  return `${name}: ${parameter.optional || parameter.initializer ? `Option<${emitted}>` : emitted}`;
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
      return ['continue;'];
    case 'do':
      return [
        'loop {',
        indent(emitStatement(statement.body, context).join('\n')),
        indent(`if !(${emitExpression(statement.condition, context)}) { break; }`),
        '}',
      ];
    case 'expression':
      return [`${emitExpression(statement.expression, context)};`];
    case 'for':
      return emitForStatement(statement, context);
    case 'forOf':
      throw new RustEmissionError('for-of Rust lowering is not implemented');
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
      throw new RustEmissionError('switch Rust lowering is not implemented');
    case 'throw':
      return [`panic!(${emitThrowMessage(statement.expression, context)});`];
    case 'try':
      throw new RustEmissionError('try Rust lowering is not implemented');
    case 'variable':
      return statement.declarations.flatMap((variable) => emitLocalVariable(variable, context));
    case 'while':
      return [`while ${emitCondition(statement.condition, context)} ${emitStatementAsBlock(statement.body, context)}`];
  }
}

function emitForStatement(statement: Extract<IrStatement, { kind: 'for' }>, context: EmitContext): string[] {
  const initializer = Array.isArray(statement.initializer)
    ? statement.initializer.flatMap((variable) => emitLocalVariable(variable, context))
    : statement.initializer
      ? [`${emitExpression(statement.initializer, context)};`]
      : [];
  const condition = statement.condition ? emitCondition(statement.condition, context) : 'true';
  const body = emitStatementAsBlock(statement.body, context, statement.increment);
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
  if (!variable.initializer) {
    if (!variable.type) throw new RustEmissionError(`cannot infer uninitialized local ${variable.name}`);
    return [`let ${variable.mutable ? 'mut ' : ''}${safeName(variable.name)}: ${emitType(variable.type)};`];
  }
  const expected = variable.type;
  const initializer = emitExpression(variable.initializer, context, expected);
  const annotation = expected && expected.kind !== 'dynamic' ? `: ${emitType(expected)}` : '';
  return [`let ${variable.mutable ? 'mut ' : ''}${safeName(variable.name)}${annotation} = ${initializer};`];
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
      return emitBinary(expression, context);
    case 'call':
      return emitCall(expression, context);
    case 'cast':
      return `(${emitExpression(expression.expression, context)} as ${emitType(expression.type)})`;
    case 'conditional':
      return `if ${emitCondition(expression.condition, context)} { ${emitExpression(expression.whenTrue, context, expectedType)} } else { ${emitExpression(expression.whenFalse, context, expectedType)} }`;
    case 'element':
      return `${emitExpression(expression.object, context)}[${emitExpression(expression.index, context)} as usize]`;
    case 'function':
      return emitClosure(expression, context, expectedType);
    case 'identifier':
      return emitIdentifier(expression.name, context);
    case 'literal':
      return emitLiteral(expression.value);
    case 'new':
      throw new RustEmissionError(
        `new-expression Rust lowering is not implemented: ${emitExpression(expression.callee, context)}`,
      );
    case 'object':
      throw new RustEmissionError('object literal Rust lowering is not implemented');
    case 'property':
      return emitProperty(expression, context);
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
  const arguments_ = expression.arguments.map((argument) => emitExpression(argument, context));
  if (expression.callee.kind === 'property' && expression.callee.object.kind === 'identifier') {
    const owner = expression.callee.object.name;
    const method = expression.callee.name;
    if (owner === 'Math') return emitMathCall(method, arguments_);
    if (owner === 'Number' && method === 'isFinite' && arguments_[0])
      return `${parenthesize(arguments_[0])}.is_finite()`;
  }
  return `${emitExpression(expression.callee, context)}(${arguments_.join(', ')})`;
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
  return `${emitExpression(expression.object, context)}.${safeName(expression.name)}`;
}

function emitBinary(expression: Extract<IrExpression, { kind: 'binary' }>, context: EmitContext): string {
  const left = emitExpression(expression.left, context);
  const right = emitExpression(expression.right, context);
  if (expression.operator === '**') return `${parenthesize(left)}.powf(${right})`;
  if (expression.operator === '??' || expression.operator === '??undefined') {
    return `${parenthesize(left)}.unwrap_or(${right})`;
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
  const left = emitExpression(expression.left, context);
  const right = emitExpression(expression.right, context);
  return `{ ${left} ${expression.operator} ${right}; ${left} }`;
}

function emitUnary(expression: Extract<IrExpression, { kind: 'unary' }>, context: EmitContext): string {
  const operand = emitExpression(expression.operand, context);
  if (expression.operator === '++' || expression.operator === '--') {
    const operator = expression.operator === '++' ? '+=' : '-=';
    return `{ ${operand} ${operator} 1.0; ${operand} }`;
  }
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
  const nextContext = { ...context, currentReturnType: returns };
  const fallbackParameter =
    expectedType?.kind === 'function'
      ? expectedType.parameters
      : expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
        ? [primitive('Float')]
        : [];
  const parameters = expression.parameters.map((parameter, index) =>
    emitParameter(parameter, nextContext, fallbackParameter[index], expression),
  );
  const body = expression.expression
    ? `{ ${emitExpression(expression.expression, nextContext, returns)} }`
    : emitStatementsAsBlock(expression.body, nextContext);
  const closure = `move |${parameters.join(', ')}| -> ${emitType(returns)} ${body}`;
  return callback ? `std::sync::Arc::new(${closure})` : closure;
}

function emitType(type: IrType): string {
  switch (type.kind) {
    case 'anonymous':
      throw new RustEmissionError('anonymous structural type Rust lowering is not implemented');
    case 'array':
      return `Vec<${emitType(type.element)}>`;
    case 'dynamic':
      throw new RustEmissionError('dynamic Rust type requires an explicit semantic mapping');
    case 'function':
      return `std::sync::Arc<dyn Fn(${type.parameters.map((item) => emitType(item)).join(', ')}) -> ${emitType(type.returns)} + Send + Sync + 'static>`;
    case 'named':
      return `${type.name}${type.arguments.length > 0 ? `<${type.arguments.map((item) => emitType(item)).join(', ')}>` : ''}`;
    case 'nullable':
      return `Option<${emitType(type.inner)}>`;
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
  return emitExpression(expression, context);
}

function emitLiteral(value: boolean | null | number | string): string {
  if (value === null) return 'None';
  if (typeof value === 'string') return JSON.stringify(value);
  if (typeof value === 'boolean') return String(value);
  if (Number.isNaN(value)) return 'f64::NAN';
  if (!Number.isFinite(value)) return value > 0 ? 'f64::INFINITY' : 'f64::NEG_INFINITY';
  if (Number.isInteger(value)) return `${String(value)}.0_f64`;
  const text = String(value);
  return `${/[.eE]/u.test(text) ? text : `${text}.0`}_f64`;
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

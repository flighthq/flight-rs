import type {
  IrClassDeclaration,
  IrDeclaration,
  IrEnumDeclaration,
  IrExpression,
  IrFunctionExecution,
  IrFunctionDeclaration,
  IrParameter,
  IrStatement,
  IrTaskOrigin,
  IrType,
  IrTypeField,
  IrVariable,
  IrVariableDeclaration,
} from '../model/ir.ts';
import { PORTABLE_TASK_RUST_LOWERING_REASON } from '../model/ir.ts';

export interface RustModule {
  constantPropertyValues?: Readonly<Record<string, boolean | number | string>>;
  declarations: IrDeclaration[];
  entityRuntimeAggregateAvailable?: boolean;
  enumNames?: readonly string[];
  imports?: RustImport[];
  inlineFunctions?: IrFunctionDeclaration[];
  semanticFunctions?: readonly IrFunctionDeclaration[];
  semanticTypeParameters?: Readonly<Record<string, readonly string[]>>;
  semanticTypes?: Readonly<Record<string, IrType>>;
  source: string;
  typeImports: string[];
}

export interface RustImport {
  module: string;
  names: Array<{
    imported: string;
    kind?: 'constant' | 'function' | 'mutable' | 'type' | 'value';
    local: string;
    public?: boolean;
  }>;
}

interface EmitContext {
  anonymousTypeOwner: string;
  anonymousTypeParameters: ReadonlyMap<string, readonly string[]>;
  anonymousTypes: ReadonlyMap<string, string>;
  atomicBoolNames: ReadonlySet<string>;
  borrowedNames: Set<string>;
  callbackArgumentStorage: ReadonlyMap<string, string>;
  callbackTypeParameters: ReadonlySet<string>;
  callbackTypes: ReadonlySet<string>;
  captureReturns: boolean;
  constantNames: ReadonlyMap<string, string>;
  constantPropertyValues: ReadonlyMap<string, boolean | number | string>;
  constantValues: ReadonlyMap<string, number>;
  continueEpilogue: readonly string[];
  currentReturnType?: IrType | undefined;
  entityRuntimeClosureError?: string | undefined;
  entityRuntimeAggregateAvailable: boolean;
  entityRuntimeFieldSlots: ReadonlyMap<string, string>;
  entityRuntimeGenericSlotTypes: ReadonlySet<string>;
  entityRuntimeLateFields: ReadonlySet<string>;
  entityRuntimeSlotTypes: ReadonlySet<string>;
  entityRuntimeTypes: ReadonlySet<string>;
  entityRuntimeUnavailableFields: ReadonlyMap<string, string>;
  entityTypeParameters: ReadonlySet<string>;
  entityTypes: ReadonlySet<string>;
  erasedValueNames: ReadonlySet<string>;
  enumNames: ReadonlySet<string>;
  errorValueNames: ReadonlySet<string>;
  excludedUnionVariants: ReadonlyMap<string, ReadonlySet<number>>;
  forwardClosureCaptureNames: ReadonlySet<string>;
  functions: ReadonlyMap<string, IrFunctionDeclaration>;
  inheritedAnonymousTypeKeys: ReadonlySet<string>;
  importedModules: ReadonlyMap<string, string>;
  importedTypeNames: ReadonlySet<string>;
  inlineFunctions: ReadonlyMap<string, IrFunctionDeclaration>;
  knownNullNames: Set<string>;
  lexicalTypeParameters: ReadonlySet<string>;
  lazyScalarNames: ReadonlySet<string>;
  localFunctionNames: ReadonlySet<string>;
  localTypeNames: ReadonlySet<string>;
  mutexCollectionNames: ReadonlySet<string>;
  mutexValueNames: ReadonlySet<string>;
  mutatedNames: ReadonlySet<string>;
  mutatingFunctions: ReadonlyMap<string, ReadonlySet<number>>;
  namedTypeParameters: ReadonlyMap<string, readonly string[]>;
  namedTypes: ReadonlyMap<string, IrType>;
  nonNullableNames: ReadonlySet<string>;
  nullCheckedNames: ReadonlySet<string>;
  numericNamespaceNames: ReadonlySet<string>;
  openInterfaceFields: ReadonlyMap<string, ReadonlySet<string>>;
  placeAliases: Map<string, IrExpression>;
  preservedNames: ReadonlySet<string>;
  rawClosureNames: Set<string>;
  recursiveClosureSlots: ReadonlyMap<string, string>;
  sharedCaptureNames: ReadonlySet<string>;
  symbolTypes: Map<string, IrType>;
  timerHandleNames: ReadonlySet<string>;
  taskOutputType?: IrType | undefined;
  unionNarrowings: Map<
    string,
    {
      index: number;
      unionName?: string | undefined;
      variants: readonly IrType[];
    }
  >;
  utf16ViewNames: ReadonlyMap<string, string>;
}

export class RustEmissionError extends Error {}

function rejectPortableTaskExecution(execution: IrFunctionExecution): void {
  if (execution.kind === 'sync') return;
  const { column, lexicalPath, line, source } = execution.origin;
  const reason = execution.kind === 'hostTaskPlaceholder' ? execution.reason : PORTABLE_TASK_RUST_LOWERING_REASON;
  throw new RustEmissionError(
    `${source}:${String(line)}:${String(column)}: ${execution.kind} ${lexicalPath}: ${reason}`,
  );
}

export function emitRustModule(module: RustModule): string {
  const constantNames = new Map([
    ...module.declarations
      .filter(
        (declaration): declaration is IrVariableDeclaration =>
          declaration.kind === 'variable' && declaration.initializer?.kind !== 'function',
      )
      .map((declaration) => [declaration.name, screamingSnakeCase(declaration.name)] as const),
    ...(module.imports ?? []).flatMap((group) =>
      group.names.flatMap((item) =>
        item.kind === 'constant' || item.kind === 'mutable'
          ? [[item.local, importedConstantBinding(item)] as const]
          : [],
      ),
    ),
  ]);
  const constantValues = new Map<string, number>();
  const constantPropertyValues = new Map<string, boolean | number | string>(
    Object.entries(module.constantPropertyValues ?? {}),
  );
  for (const declaration of module.declarations) {
    if (declaration.kind !== 'variable' || !declaration.initializer || declaration.initializer.kind === 'function') {
      continue;
    }
    const value = evaluateConstant(declaration.initializer, constantValues);
    if (value !== undefined) constantValues.set(declaration.name, value);
    const object = unwrapCasts(declaration.initializer);
    if (object.kind === 'object') {
      for (const property of object.properties) {
        if (property.kind === 'property' && property.value.kind === 'literal' && property.value.value !== null) {
          constantPropertyValues.set(`${declaration.name}.${property.name}`, property.value.value);
        }
      }
    }
  }
  const inlineFunctions = new Map((module.inlineFunctions ?? []).map((declaration) => [declaration.name, declaration]));
  const mutatingFunctions = collectMutatingFunctionParameters([
    ...module.declarations,
    ...(module.inlineFunctions ?? []),
    ...(module.semanticFunctions ?? []),
  ]);
  const moduleMutatedNames = collectMutatedNames(module.declarations, mutatingFunctions);
  const context: EmitContext = {
    anonymousTypeOwner: 'Module',
    anonymousTypeParameters: new Map(),
    anonymousTypes: new Map(),
    atomicBoolNames: new Set(
      module.declarations.flatMap((declaration) =>
        declaration.kind === 'variable' &&
        declaration.mutable &&
        declaration.initializer?.kind === 'literal' &&
        typeof declaration.initializer.value === 'boolean'
          ? [declaration.name]
          : [],
      ),
    ),
    borrowedNames: new Set(),
    callbackArgumentStorage: new Map(),
    callbackTypeParameters: new Set(),
    callbackTypes: new Set(['EasingFunction', 'ScalarRemap']),
    captureReturns: false,
    constantNames,
    constantPropertyValues,
    constantValues,
    continueEpilogue: [],
    entityRuntimeFieldSlots: new Map(),
    entityRuntimeGenericSlotTypes: new Set(),
    entityRuntimeLateFields: new Set(),
    entityRuntimeSlotTypes: new Set(),
    entityRuntimeTypes: new Set(),
    entityRuntimeUnavailableFields: new Map(),
    entityRuntimeAggregateAvailable: module.entityRuntimeAggregateAvailable ?? false,
    entityTypeParameters: new Set(),
    entityTypes: new Set(),
    erasedValueNames: new Set(
      module.declarations.filter((declaration) => declaration.kind === 'type').map((declaration) => declaration.name),
    ),
    enumNames: new Set([
      ...(module.enumNames ?? []),
      ...module.declarations
        .filter((declaration) => declaration.kind === 'enum')
        .map((declaration) => declaration.name),
      ...module.declarations.flatMap((declaration) =>
        declaration.kind === 'variable' &&
        declaration.exported &&
        (!declaration.type || declaration.type.kind === 'dynamic') &&
        !moduleMutatedNames.has(declaration.name) &&
        isNumericNamespaceInitializer(declaration.initializer)
          ? [declaration.name]
          : [],
      ),
    ]),
    errorValueNames: new Set(),
    excludedUnionVariants: new Map(),
    forwardClosureCaptureNames: new Set(),
    functions: new Map(
      [
        ...(module.semanticFunctions ?? []),
        ...module.declarations.filter(
          (declaration): declaration is IrFunctionDeclaration => declaration.kind === 'function',
        ),
        ...module.declarations.flatMap((declaration): IrFunctionDeclaration[] => {
          if (
            declaration.kind !== 'variable' ||
            declaration.initializer?.kind !== 'function' ||
            !declaration.initializer.returns
          ) {
            return [];
          }
          return [
            {
              body: declaration.initializer.body,
              execution: declaration.initializer.execution,
              exported: declaration.exported,
              kind: 'function',
              name: declaration.name,
              origin: declaration.origin,
              parameters: declaration.initializer.parameters,
              returns: declaration.initializer.returns,
              typeParameters: [],
            },
          ];
        }),
      ].map((declaration) => [declaration.name, declaration]),
    ),
    inheritedAnonymousTypeKeys: new Set(),
    importedModules: new Map(
      (module.imports ?? []).flatMap((group) => group.names.map((item) => [item.local, group.module] as const)),
    ),
    importedTypeNames: new Set(module.typeImports),
    inlineFunctions,
    knownNullNames: new Set(),
    lexicalTypeParameters: new Set(),
    lazyScalarNames: new Set(),
    localFunctionNames: new Set(
      module.declarations.flatMap((declaration) =>
        declaration.kind === 'function' ||
        (declaration.kind === 'variable' && declaration.initializer?.kind === 'function')
          ? [declaration.name]
          : [],
      ),
    ),
    localTypeNames: new Set(
      module.declarations.filter((declaration) => declaration.kind === 'type').map((declaration) => declaration.name),
    ),
    mutexCollectionNames: new Set(
      module.declarations.flatMap((declaration) =>
        declaration.kind === 'variable' &&
        declaration.initializer?.kind === 'array' &&
        moduleMutatedNames.has(declaration.name)
          ? [declaration.name]
          : [],
      ),
    ),
    mutexValueNames: new Set([
      ...module.declarations.flatMap((declaration) =>
        declaration.kind === 'variable' &&
        declaration.initializer &&
        declaration.initializer.kind !== 'array' &&
        !(declaration.initializer.kind === 'literal' && typeof declaration.initializer.value === 'boolean') &&
        moduleMutatedNames.has(declaration.name)
          ? [declaration.name]
          : [],
      ),
      ...(module.imports ?? []).flatMap((group) =>
        group.names.flatMap((item) => (item.kind === 'mutable' ? [item.local] : [])),
      ),
    ]),
    mutatedNames: new Set(),
    mutatingFunctions,
    namedTypeParameters: new Map([
      ...Object.entries(module.semanticTypeParameters ?? {}),
      ...module.declarations
        .filter((declaration) => declaration.kind === 'type')
        .map((declaration) => [declaration.name, declaration.typeParameters] as const),
    ]),
    namedTypes: new Map([
      ...Object.entries(module.semanticTypes ?? {}),
      ...module.declarations
        .filter((declaration) => declaration.kind === 'type')
        .map((declaration) => [declaration.name, declaration.type] as const),
    ]),
    nonNullableNames: new Set(),
    nullCheckedNames: new Set(),
    numericNamespaceNames: new Set(
      module.declarations.flatMap((declaration) =>
        declaration.kind === 'variable' &&
        declaration.exported &&
        (!declaration.type || declaration.type.kind === 'dynamic') &&
        !moduleMutatedNames.has(declaration.name) &&
        isNumericNamespaceInitializer(declaration.initializer)
          ? [declaration.name]
          : [],
      ),
    ),
    openInterfaceFields: new Map(),
    placeAliases: new Map(),
    preservedNames: new Set(),
    rawClosureNames: new Set(),
    recursiveClosureSlots: new Map(),
    sharedCaptureNames: new Set(),
    symbolTypes: new Map(
      module.declarations.flatMap((declaration) => {
        if (declaration.kind !== 'variable') return [];
        const type =
          declaration.type ??
          (declaration.initializer ? inferStaticExpressionType(declaration.initializer) : undefined);
        return type ? [[declaration.name, type] as const] : [];
      }),
    ),
    timerHandleNames: new Set(),
    unionNarrowings: new Map(),
    utf16ViewNames: new Map(),
  };
  registerEntityRuntimeFamilies(context);
  registerOpenInterfaceFamilies(context);
  for (const declaration of module.declarations) {
    if (declaration.kind !== 'variable' || !declaration.initializer) continue;
    const object = unwrapCasts(declaration.initializer);
    const contextualObject =
      object.kind === 'object' ? inferContextualObjectType(object, context, declaration.type) : undefined;
    const inferred =
      (declaration.type?.kind === 'dynamic' ? contextualObject : declaration.type) ??
      contextualObject ??
      inferStaticExpressionType(declaration.initializer) ??
      inferIrExpressionType(declaration.initializer, context);
    if (
      declaration.initializer.kind === 'object' &&
      inferred?.kind === 'anonymous' &&
      !context.numericNamespaceNames.has(declaration.name)
    ) {
      const recordName = topLevelStructuralRecordName(declaration.name, context);
      (context.namedTypes as Map<string, IrType>).set(recordName, inferred);
      context.symbolTypes.set(declaration.name, {
        arguments: [],
        kind: 'named',
        name: recordName,
      });
    } else if (inferred) {
      context.symbolTypes.set(declaration.name, inferred);
    }
  }
  for (const declaration of module.declarations) {
    if (
      declaration.kind !== 'variable' ||
      !declaration.initializer ||
      declaration.initializer.kind === 'function' ||
      moduleMutatedNames.has(declaration.name) ||
      constantValues.has(declaration.name) ||
      isRustConstExpression(declaration.initializer)
    ) {
      continue;
    }
    const type = resolveSemanticType(context.symbolTypes.get(declaration.name), context);
    if (type?.kind === 'primitive' && isCopyType(type, context)) {
      (context.lazyScalarNames as Set<string>).add(declaration.name);
    }
  }
  registerSharedModuleAnonymousTypes(module.declarations, context);
  registerGlobalResolvedAnonymousTypes(module.declarations, context);
  registerImportedFunctionAnonymousTypes(module.semanticFunctions ?? [], context);
  registerTypeDeclarationAnonymousTypes(module.declarations, context);
  registerImportedTypeAnonymousTypes(context);
  registerNestedAnonymousTypes(context);
  const declarationBodies = module.declarations
    .map((declaration) => {
      try {
        return emitDeclaration(declaration, context);
      } catch (error) {
        if (error instanceof RustEmissionError) {
          throw new RustEmissionError(`${declaration.name}: ${error.message}`);
        }
        throw error;
      }
    })
    .join('\n\n');
  const declarations = [emitAnonymousDefinitions(context, true), declarationBodies].filter(Boolean).join('\n\n');
  const existingImportNames = new Set([
    ...module.typeImports,
    ...(module.imports ?? []).flatMap((group) => group.names.map((item) => item.local)),
  ]);
  const synthesizedTypeImports = new Set<string>();
  for (const name of context.localTypeNames) {
    const type = context.namedTypes.get(name);
    if (type?.kind !== 'anonymous') continue;
    for (const field of flattenStructFields(type, context)) {
      for (const referenced of collectReferencedNamedTypes(field.type)) {
        if (
          referenced !== name &&
          context.namedTypes.has(referenced) &&
          !context.localTypeNames.has(referenced) &&
          !existingImportNames.has(referenced)
        ) {
          synthesizedTypeImports.add(referenced);
        }
      }
    }
  }
  const importGroups: RustImport[] = [
    ...(synthesizedTypeImports.size > 0
      ? [
          {
            module: 'crate',
            names: [...synthesizedTypeImports].map((name) => ({
              imported: name,
              kind: 'type' as const,
              local: name,
            })),
          },
        ]
      : []),
    ...(module.typeImports.length > 0
      ? [
          {
            module: 'crate',
            names: [...new Set(module.typeImports)].map((name) => ({
              imported: name,
              kind: 'type' as const,
              local: name,
            })),
          },
        ]
      : []),
    ...(module.imports ?? []),
  ];
  const imports = importGroups
    .flatMap((group) => {
      const names = [
        ...group.names
          .flatMap((item) => {
            const resolved = resolveRustImport(item, declarations);
            return resolved ? [resolved] : [];
          })
          .reduce((bindings, item) => {
            const previous = bindings.get(item.local);
            if (!previous || item.public) bindings.set(item.local, item);
            return bindings;
          }, new Map<string, RustImport['names'][number]>())
          .values(),
      ].sort((left, right) => left.local.localeCompare(right.local));
      return [false, true].flatMap((publicBinding) => {
        const selected = names.filter((item) => Boolean(item.public) === publicBinding);
        if (selected.length === 0) return [];
        const bindings = selected
          .map(({ imported, local }) => (imported === local ? imported : `${imported} as ${local}`))
          .join(', ');
        return [`${publicBinding ? 'pub ' : ''}use ${group.module}::{${bindings}};`];
      });
    })
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
  const stringHelpers = [
    ...(declarations.includes('__flight_number_to_string(')
      ? [
          '#[inline]',
          'fn __flight_number_to_string(value: f64, radix: f64) -> String {',
          indent(
            [
              'let radix = radix.trunc().clamp(2.0_f64, 36.0_f64) as u32;',
              'let mut value = value.trunc().rem_euclid(4294967296.0_f64) as u32;',
              'if value == 0 { return "0".to_owned(); }',
              'let mut digits = Vec::new();',
              'while value > 0 {',
              indent(
                'let digit = value % radix;\ndigits.push(char::from_digit(digit, radix).unwrap());\nvalue /= radix;',
              ),
              '}',
              'digits.iter().rev().collect()',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_number_to_fixed(')
      ? [
          '#[inline]',
          'fn __flight_number_to_fixed(value: f64, digits: f64) -> String {',
          indent(
            [
              'assert!(digits.is_finite() && digits.fract() == 0.0_f64 && (0.0_f64..=100.0_f64).contains(&digits), "Number.toFixed digits must be between 0 and 100");',
              'if value.is_nan() { return "NaN".to_owned(); }',
              'if value == f64::INFINITY { return "Infinity".to_owned(); }',
              'if value == f64::NEG_INFINITY { return "-Infinity".to_owned(); }',
              'let value = if value == 0.0_f64 { 0.0_f64 } else { value };',
              'format!("{:.*}", digits as usize, value)',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_pad_start(')
      ? [
          '#[inline]',
          'fn __flight_pad_start(value: String, width: f64, pad: String) -> String {',
          indent(
            [
              'let length = value.chars().count();',
              'let width = width.max(0.0_f64).trunc() as usize;',
              'if length >= width || pad.is_empty() { return value; }',
              'let prefix: String = pad.chars().cycle().take(width - length).collect();',
              'prefix + &value',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_string_index_of(')
      ? [
          '#[inline]',
          'fn __flight_string_index_of(value: &str, search: &str, position: f64) -> f64 {',
          indent(
            [
              'let value: Vec<u16> = value.encode_utf16().collect();',
              'let search: Vec<u16> = search.encode_utf16().collect();',
              'let start = if position.is_nan() || position <= 0.0_f64 { 0_usize } else if position >= value.len() as f64 { value.len() } else { position.trunc() as usize };',
              'if search.is_empty() { return start as f64; }',
              'value[start..].windows(search.len()).position(|window| window == search).map_or(-1.0_f64, |index| (start + index) as f64)',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_string_slice(')
      ? [
          '#[inline]',
          'fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {',
          indent(
            [
              'let value: Vec<u16> = value.encode_utf16().collect();',
              'let length = value.len();',
              'let relative = |index: f64| -> usize {',
              indent(
                'if index.is_nan() { 0 } else if index < 0.0_f64 { length.saturating_sub((-index.trunc()) as usize) } else { (index.trunc() as usize).min(length) }',
              ),
              '};',
              'let start = relative(start);',
              'let end = end.map_or(length, relative);',
              'String::from_utf16_lossy(&value[start..end.max(start)])',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_string_repeat(')
      ? [
          '#[inline]',
          'fn __flight_string_repeat(value: &str, count: f64) -> String {',
          indent(
            [
              'assert!(count.is_finite() && count >= 0.0_f64, "String.repeat count must be finite and non-negative");',
              'value.repeat(count.trunc() as usize)',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_string_from_code_point(')
      ? [
          '#[inline]',
          'fn __flight_string_from_code_point(value: f64) -> String {',
          indent(
            [
              'assert!(value.is_finite() && value.fract() == 0.0_f64 && (0.0_f64..=0x10FFFF_u32 as f64).contains(&value), "String.fromCodePoint received an invalid code point");',
              'char::from_u32(value as u32).expect("Rust strings cannot represent surrogate code points").to_string()',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_encode_uri_component(')
      ? [
          '#[inline]',
          'fn __flight_encode_uri_component(value: &str) -> String {',
          indent(
            [
              'const HEX: &[u8; 16] = b"0123456789ABCDEF";',
              'let mut encoded = String::with_capacity(value.len());',
              'for byte in value.bytes() {',
              indent(
                [
                  'if byte.is_ascii_alphanumeric() || b"-_.!~*\'()".contains(&byte) {',
                  indent('encoded.push(char::from(byte));'),
                  '} else {',
                  indent(
                    [
                      "encoded.push('%');",
                      'encoded.push(char::from(HEX[(byte >> 4) as usize]));',
                      'encoded.push(char::from(HEX[(byte & 0x0F) as usize]));',
                    ].join('\n'),
                  ),
                  '}',
                ].join('\n'),
              ),
              '}',
              'encoded',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_decode_uri_component(')
      ? [
          '#[inline]',
          'fn __flight_decode_uri_component(value: &str) -> String {',
          indent(
            [
              'let bytes = value.as_bytes();',
              'let mut decoded = Vec::with_capacity(bytes.len());',
              'let mut index = 0_usize;',
              'while index < bytes.len() {',
              indent(
                [
                  "if bytes[index] != b'%' { decoded.push(bytes[index]); index += 1; continue; }",
                  'assert!(index + 2 < bytes.len(), "decodeURIComponent received an incomplete escape");',
                  "let digit = |byte: u8| -> Option<u8> { match byte { b'0'..=b'9' => Some(byte - b'0'), b'a'..=b'f' => Some(byte - b'a' + 10), b'A'..=b'F' => Some(byte - b'A' + 10), _ => None } };",
                  'let high = digit(bytes[index + 1]).expect("decodeURIComponent received a malformed escape");',
                  'let low = digit(bytes[index + 2]).expect("decodeURIComponent received a malformed escape");',
                  'decoded.push((high << 4) | low);',
                  'index += 3;',
                ].join('\n'),
              ),
              '}',
              'String::from_utf8(decoded).expect("decodeURIComponent received invalid UTF-8")',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_number_from_string(')
      ? [
          '#[inline]',
          'fn __flight_number_from_string(value: &str) -> f64 {',
          indent(
            [
              'let value = value.trim();',
              'if value.is_empty() { return 0.0_f64; }',
              'match value { "Infinity" | "+Infinity" => return f64::INFINITY, "-Infinity" => return f64::NEG_INFINITY, _ => {} }',
              'let prefixed = if let Some(digits) = value.strip_prefix("0x").or_else(|| value.strip_prefix("0X")) { Some((digits, 16_u32)) } else if let Some(digits) = value.strip_prefix("0b").or_else(|| value.strip_prefix("0B")) { Some((digits, 2_u32)) } else { value.strip_prefix("0o").or_else(|| value.strip_prefix("0O")).map(|digits| (digits, 8_u32)) };',
              'if let Some((digits, radix)) = prefixed { return u64::from_str_radix(digits, radix).map_or(f64::NAN, |number| number as f64); }',
              'value.parse::<f64>().unwrap_or(f64::NAN)',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
    ...(declarations.includes('__flight_parse_float(')
      ? [
          '#[inline]',
          'fn __flight_parse_float(value: &str) -> f64 {',
          indent(
            [
              'let value = value.trim_start();',
              'if value.starts_with("Infinity") || value.starts_with("+Infinity") { return f64::INFINITY; }',
              'if value.starts_with("-Infinity") { return f64::NEG_INFINITY; }',
              'let bytes = value.as_bytes();',
              "let mut index = usize::from(matches!(bytes.first(), Some(b'+' | b'-')));",
              'let mut digits = 0_usize;',
              "while matches!(bytes.get(index), Some(b'0'..=b'9')) { index += 1; digits += 1; }",
              "if bytes.get(index) == Some(&b'.') { index += 1; while matches!(bytes.get(index), Some(b'0'..=b'9')) { index += 1; digits += 1; } }",
              'if digits == 0 { return f64::NAN; }',
              'let mantissa_end = index;',
              "if matches!(bytes.get(index), Some(b'e' | b'E')) {",
              indent(
                [
                  'let exponent_start = index;',
                  'index += 1;',
                  "if matches!(bytes.get(index), Some(b'+' | b'-')) { index += 1; }",
                  'let exponent_digits = index;',
                  "while matches!(bytes.get(index), Some(b'0'..=b'9')) { index += 1; }",
                  'if index == exponent_digits { index = exponent_start; }',
                ].join('\n'),
              ),
              '}',
              'value[..if index > mantissa_end { index } else { mantissa_end }].parse::<f64>().unwrap_or(f64::NAN)',
            ].join('\n'),
          ),
          '}',
        ]
      : []),
  ].join('\n\n');
  return [
    `// @generated from ${module.source}; do not edit.`,
    '#![allow(clippy::excessive_precision)]',
    '#![allow(non_upper_case_globals)]',
    '#![allow(unused_braces)]',
    '#![allow(unused_imports)]',
    '#![allow(unused_mut)]',
    '#![allow(unused_parens)]',
    '',
    `${imports.length > 0 ? `${imports}\n\n` : ''}${numericCoercions.length > 0 ? `${numericCoercions}\n\n` : ''}${stringHelpers.length > 0 ? `${stringHelpers}\n\n` : ''}${declarations}`,
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
      if (context.numericNamespaceNames.has(declaration.name)) {
        return `${provenance}\n// TypeScript numeric namespace ${declaration.name} is represented by its generated Rust constants.`;
      }
      return `${provenance}\n${emitTypeDeclaration(
        declaration.name,
        declaration.exported,
        context.openInterfaceFields.has(declaration.name)
          ? (context.namedTypes.get(declaration.name) ?? declaration.type)
          : declaration.type,
        context,
        declaration.typeParameters,
      )}`;
    case 'enum':
      return `${provenance}\n${emitEnumDeclaration(declaration, context)}`;
    case 'class':
      return `${provenance}\n${emitClassDeclaration(declaration, context)}`;
  }
}

function emitClassDeclaration(declaration: IrClassDeclaration, context: EmitContext): string {
  if (declaration.methods.length > 0 || declaration.fields.some((field) => field.static)) {
    throw new RustEmissionError(
      `${declaration.origin.source}:${String(declaration.origin.line)}: class methods and static fields are not implemented for ${declaration.name}`,
    );
  }
  const extendsPortError = declaration.extends?.kind === 'named' && declaration.extends.name === 'PortError';
  if (declaration.extends && !extendsPortError) {
    throw new RustEmissionError(
      `${declaration.origin.source}:${String(declaration.origin.line)}: class inheritance is not implemented for ${declaration.name}`,
    );
  }
  const constructorContext = functionContext(context, declaration.name, declaration, {
    arguments: [],
    kind: 'named',
    name: declaration.name,
  });
  registerParameters(declaration.constructorParameters, constructorContext);
  const parameters = declaration.constructorParameters.map((parameter) =>
    emitParameter(parameter, constructorContext, undefined, declaration),
  );
  const assignments = new Map<string, IrExpression>();
  let message: IrExpression | undefined;
  for (const statement of declaration.constructorBody) {
    if (
      statement.kind === 'expression' &&
      statement.expression.kind === 'call' &&
      statement.expression.callee.kind === 'identifier' &&
      statement.expression.callee.name === 'super' &&
      statement.expression.arguments[0]
    ) {
      message = statement.expression.arguments[0];
      continue;
    }
    if (
      statement.kind === 'expression' &&
      statement.expression.kind === 'assignment' &&
      statement.expression.operator === '=' &&
      statement.expression.left.kind === 'property' &&
      statement.expression.left.object.kind === 'identifier' &&
      statement.expression.left.object.name === 'this'
    ) {
      assignments.set(statement.expression.left.name, statement.expression.right);
      continue;
    }
    throw new RustEmissionError(
      `${declaration.origin.source}:${String(declaration.origin.line)}: constructor statement lowering is not implemented for ${declaration.name}`,
    );
  }
  if (extendsPortError && !message) {
    throw new RustEmissionError(
      `${declaration.origin.source}: Error subclass ${declaration.name} is missing super(message)`,
    );
  }
  const visibility = declaration.exported ? 'pub ' : '';
  const fields = declaration.fields.map((field) => {
    const initializer = assignments.get(field.name) ?? field.initializer;
    if (!initializer) {
      throw new RustEmissionError(
        `${declaration.origin.source}: class field ${declaration.name}.${field.name} is uninitialized`,
      );
    }
    return {
      declaration: `${field.public ? 'pub ' : ''}${safeName(field.name)}: ${emitType(field.type, constructorContext)},`,
      initializer: `${safeName(field.name)}: ${emitExpression(initializer, constructorContext, field.type)},`,
    };
  });
  const messageField = extendsPortError ? ['pub message: String,'] : [];
  const messageInitializer =
    extendsPortError && message
      ? [`message: ${emitExpression(message, constructorContext, primitive('String'))},`]
      : [];
  return [
    '#[derive(Clone, Debug)]',
    `${visibility}struct ${declaration.name} {`,
    indent([...messageField, ...fields.map((field) => field.declaration)].join('\n')),
    '}',
    '',
    `impl ${declaration.name} {`,
    indent(
      `${visibility}fn new(${parameters.join(', ')}) -> Self {\n${indent(`Self {\n${indent([...messageInitializer, ...fields.map((field) => field.initializer)].join('\n'))}\n}`)}\n}`,
    ),
    '}',
    ...(extendsPortError
      ? [
          '',
          `impl std::fmt::Display for ${declaration.name} {`,
          indent(
            `fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n${indent('formatter.write_str(&self.message)')}\n}`,
          ),
          '}',
          `impl std::error::Error for ${declaration.name} {}`,
        ]
      : []),
  ].join('\n');
}

function emitEnumDeclaration(declaration: IrEnumDeclaration, context: EmitContext): string {
  const visibility = declaration.exported ? 'pub ' : '';
  const values = new Map<string, number>();
  let previous = -1;
  const members = declaration.members.map((member) => {
    const value = member.initializer ? evaluateConstant(member.initializer, values) : previous + 1;
    if (value === undefined || !Number.isSafeInteger(value) || value < -0x80_00_00_00 || value > 0xff_ff_ff_ff) {
      throw new RustEmissionError(
        `${declaration.origin.source}: enum member ${declaration.name}.${member.name} requires a non-negative u32 constant`,
      );
    }
    previous = value;
    values.set(member.name, value);
    return `#[allow(non_upper_case_globals)]\n${visibility}const ${member.name}: Self = Self(${String(value >>> 0)}_u32);`;
  });
  const methods = declaration.methods.map((method) => emitFunctionDeclaration(method, context));
  const implementation = [...members, ...methods].length
    ? `\n\nimpl ${declaration.name} {\n${indent([...members, ...methods].join('\n\n'))}\n}`
    : '';
  return [
    '#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]',
    '#[repr(transparent)]',
    `${visibility}struct ${declaration.name}(pub u32);`,
    implementation,
    '',
    `impl std::ops::BitAnd for ${declaration.name} {`,
    indent(`type Output = Self;\nfn bitand(self, rhs: Self) -> Self { Self(self.0 & rhs.0) }`),
    '}',
    `impl std::ops::BitOr for ${declaration.name} {`,
    indent(`type Output = Self;\nfn bitor(self, rhs: Self) -> Self { Self(self.0 | rhs.0) }`),
    '}',
    `impl std::ops::BitXor for ${declaration.name} {`,
    indent(`type Output = Self;\nfn bitxor(self, rhs: Self) -> Self { Self(self.0 ^ rhs.0) }`),
    '}',
    `impl std::ops::Not for ${declaration.name} {`,
    indent(`type Output = Self;\nfn not(self) -> Self { Self(!self.0) }`),
    '}',
    `impl PartialEq<f64> for ${declaration.name} {`,
    indent(`fn eq(&self, rhs: &f64) -> bool { self.0 as f64 == *rhs }`),
    '}',
  ].join('\n');
}

function emitTopLevelVariable(declaration: IrVariableDeclaration, context: EmitContext): string {
  if (context.numericNamespaceNames.has(declaration.name) && declaration.initializer?.kind === 'object') {
    const members = declaration.initializer.properties.flatMap((property) => {
      if (property.kind !== 'property') return [];
      const value = evaluateConstant(property.value, context.constantValues);
      if (value === undefined) return [];
      return [`pub const ${property.name}: f64 = ${emitLiteral(value)};`];
    });
    return [
      `${declaration.exported ? 'pub ' : ''}struct ${declaration.name};`,
      `impl ${declaration.name} {\n${indent(members.join('\n'))}\n}`,
    ].join('\n');
  }
  if (context.erasedValueNames.has(declaration.name) && !isScalarNamespaceInitializer(declaration.initializer)) {
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
  const objectInitializer = declaration.initializer.kind === 'object' ? declaration.initializer : undefined;
  const registeredObjectName = topLevelStructuralRecordName(declaration.name, context);
  const registeredObject = context.symbolTypes.get(declaration.name);
  const registeredObjectType =
    registeredObject?.kind === 'named' && registeredObject.name === registeredObjectName
      ? context.namedTypes.get(registeredObjectName)
      : undefined;
  const inferredObject = objectInitializer
    ? (registeredObjectType ??
      inferContextualObjectType(objectInitializer, context, declaration.type) ??
      inferStaticExpressionType(objectInitializer))
    : undefined;
  if (
    context.atomicBoolNames.has(declaration.name) &&
    declaration.initializer.kind === 'literal' &&
    typeof declaration.initializer.value === 'boolean'
  ) {
    return `${visibility}static ${name}: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(${String(declaration.initializer.value)});`;
  }
  if (context.mutexCollectionNames.has(declaration.name)) {
    const type = declaration.type
      ? emitType(declaration.type, context)
      : emitType(
          inferStaticExpressionType(declaration.initializer) ?? {
            element: { kind: 'dynamic' },
            kind: 'array',
          },
          context,
        );
    return `${visibility}static ${name}: std::sync::LazyLock<std::sync::Mutex<${type}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(${emitExpression(declaration.initializer, context, declaration.type)}));`;
  }
  if (context.mutexValueNames.has(declaration.name)) {
    const structuralType =
      declaration.type?.kind === 'anonymous'
        ? declaration.type
        : (!declaration.type || declaration.type.kind === 'dynamic') && inferredObject?.kind === 'anonymous'
          ? inferredObject
          : undefined;
    if (objectInitializer && structuralType) {
      const recordName = pascalCase(declaration.name);
      const structuralContext = typeDeclarationContext(context, recordName, structuralType);
      const objectContext = {
        ...structuralContext,
        anonymousTypes: new Map([...structuralContext.anonymousTypes, [typeKey(structuralType), recordName]]),
      };
      return [
        emitTypeDeclaration(recordName, declaration.exported, structuralType, context),
        `${visibility}static ${name}: std::sync::LazyLock<std::sync::Mutex<${recordName}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(${emitObject(objectInitializer, objectContext, structuralType)}));`,
      ].join('\n\n');
    }
    const inferred =
      (declaration.type?.kind === 'dynamic' ? inferredObject : declaration.type) ??
      inferredObject ??
      inferIrExpressionType(declaration.initializer, context);
    if (!inferred) throw new RustEmissionError(`cannot infer mutable top-level value ${declaration.name}`);
    return `${visibility}static ${name}: std::sync::LazyLock<std::sync::Mutex<${emitType(inferred, context)}>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(${emitExpression(declaration.initializer, context, inferred)}));`;
  }
  const symbol = symbolDescription(declaration.initializer);
  if (symbol !== undefined) {
    return `${visibility}static ${name}: std::sync::LazyLock<crate::FlightSymbol> = std::sync::LazyLock::new(|| ${emitExpression(declaration.initializer, context)});`;
  }
  const structuralType =
    declaration.type?.kind === 'anonymous'
      ? declaration.type
      : (!declaration.type || declaration.type.kind === 'dynamic') && inferredObject?.kind === 'anonymous'
        ? inferredObject
        : undefined;
  if (objectInitializer && structuralType) {
    const recordName = topLevelStructuralRecordName(declaration.name, context);
    const structuralContext = typeDeclarationContext(context, recordName, structuralType);
    const objectContext = {
      ...structuralContext,
      anonymousTypes: new Map([...structuralContext.anonymousTypes, [typeKey(structuralType), recordName]]),
    };
    return [
      emitTypeDeclaration(recordName, declaration.exported, structuralType, context),
      `${visibility}static ${name}: std::sync::LazyLock<${recordName}> = std::sync::LazyLock::new(|| ${emitObject(objectInitializer, objectContext, structuralType)});`,
    ].join('\n\n');
  }
  if (objectInitializer && inferredObject && inferredObject.kind !== 'anonymous') {
    const type = emitType(inferredObject, context);
    return `${visibility}static ${name}: std::sync::LazyLock<${type}> = std::sync::LazyLock::new(|| ${emitObject(objectInitializer, context, inferredObject)});`;
  }
  if (declaration.type && objectInitializer) {
    const type = emitType(declaration.type, context);
    return `${visibility}static ${name}: std::sync::LazyLock<${type}> = std::sync::LazyLock::new(|| ${emitObject(objectInitializer, context, declaration.type)});`;
  }
  if (declaration.initializer.kind === 'literal' && typeof declaration.initializer.value === 'string') {
    return `${visibility}const ${name}: &'static str = ${emitLiteral(declaration.initializer.value)};`;
  }
  const type = declaration.type ? emitType(declaration.type, context) : inferExpressionType(declaration.initializer);
  const folded = context.constantValues.get(declaration.name);
  const initializer =
    folded === undefined ? emitExpression(declaration.initializer, context, declaration.type) : emitLiteral(folded);
  const lazyType =
    declaration.type ??
    inferStaticExpressionType(declaration.initializer) ??
    inferIrExpressionType(declaration.initializer, context);
  if (folded === undefined && !isRustConstExpression(declaration.initializer) && lazyType) {
    return `${visibility}static ${name}: std::sync::LazyLock<${emitType(lazyType, context)}> = std::sync::LazyLock::new(|| ${initializer});`;
  }
  return `${visibility}const ${name}: ${type} = ${initializer};`;
}

function topLevelStructuralRecordName(name: string, context: EmitContext): string {
  return context.erasedValueNames.has(name) ? `${pascalCase(name)}Values` : pascalCase(name);
}

export function isNumericNamespaceInitializer(expression: IrExpression | undefined): boolean {
  return (
    expression?.kind === 'object' &&
    expression.properties.length > 0 &&
    expression.properties.every(
      (property) => property.kind === 'property' && evaluateConstant(property.value, new Map()) !== undefined,
    )
  );
}

function isScalarNamespaceInitializer(expression: IrExpression | undefined): boolean {
  return (
    (expression?.kind === 'literal' && ['boolean', 'number', 'string'].includes(typeof expression.value)) ||
    (expression?.kind === 'object' &&
      expression.properties.length > 0 &&
      expression.properties.every(
        (property) =>
          property.kind === 'property' &&
          property.value.kind === 'literal' &&
          ['boolean', 'number', 'string'].includes(typeof property.value.value),
      ))
  );
}

function isRustConstExpression(expression: IrExpression): boolean {
  switch (expression.kind) {
    case 'literal':
      return true;
    case 'identifier':
      return true;
    case 'unary':
      return ['+', '-', '!', '~'].includes(expression.operator) && isRustConstExpression(expression.operand);
    case 'binary':
      return isRustConstExpression(expression.left) && isRustConstExpression(expression.right);
    case 'property':
      return expression.object.kind === 'identifier' && expression.object.name === 'Math' && expression.name === 'PI';
    case 'cast':
      return isRustConstExpression(expression.expression);
    default:
      return false;
  }
}

function symbolDescription(expression: IrExpression): string | undefined {
  if (expression.kind !== 'call' || expression.arguments[0]?.kind !== 'literal') return undefined;
  const description = expression.arguments[0].value;
  if (typeof description !== 'string') return undefined;
  if (expression.callee.kind === 'identifier' && expression.callee.name === 'Symbol') return description;
  return expression.callee.kind === 'property' &&
    expression.callee.object.kind === 'identifier' &&
    expression.callee.object.name === 'Symbol' &&
    expression.callee.name === 'for'
    ? description
    : undefined;
}

function inferStaticExpressionType(expression: IrExpression): IrType | undefined {
  switch (expression.kind) {
    case 'array': {
      const elements = expression.elements.flatMap((item) => {
        const type = inferStaticExpressionType(item);
        return type ? [type] : [];
      });
      const first = elements[0];
      return first && elements.every((item) => typeKey(item) === typeKey(first))
        ? { element: first, kind: 'array' }
        : undefined;
    }
    case 'literal':
      if (typeof expression.value === 'boolean') return primitive('Bool');
      if (typeof expression.value === 'number') return primitive('Float');
      if (typeof expression.value === 'string') return primitive('String');
      return undefined;
    case 'template':
      return primitive('String');
    case 'call':
      return isSymbolConstruction(expression) ? { arguments: [], kind: 'named', name: 'FlightSymbol' } : undefined;
    case 'hostConstruct':
      return { arguments: [], kind: 'named', name: expression.resultType };
    case 'taskAll':
    case 'taskReady':
    case 'taskReject':
      return { kind: 'task', output: expression.output };
    case 'new': {
      const name = runtimeConstructorType(expression.callee);
      if (name === 'Map' || name === 'WeakMap') {
        return {
          arguments: expression.typeArguments,
          kind: 'named',
          name: 'RustMap',
        };
      }
      if (name === 'Set' || name === 'WeakSet') {
        const source = expression.arguments[0] ? inferStaticExpressionType(expression.arguments[0]) : undefined;
        return {
          arguments:
            expression.typeArguments.length > 0
              ? expression.typeArguments
              : source?.kind === 'array'
                ? [source.element]
                : [],
          kind: 'named',
          name: 'RustSet',
        };
      }
      if (name === 'Array') {
        return {
          element: expression.typeArguments[0] ?? { kind: 'dynamic' },
          kind: 'array',
        };
      }
      return undefined;
    }
    case 'cast':
      return expression.type;
    case 'unary':
      if (expression.operator === '!') return primitive('Bool');
      if (expression.operator === 'typeof') return primitive('String');
      if (['+', '-', '~', '++', '--'].includes(expression.operator)) return primitive('Float');
      return undefined;
    case 'object': {
      const fields = expression.properties.flatMap((property): IrTypeField[] => {
        if (property.kind !== 'property') return [];
        const type = inferStaticExpressionType(property.value);
        return type ? [{ name: property.name, optional: false, type }] : [];
      });
      return fields.length === expression.properties.length ? { extends: [], fields, kind: 'anonymous' } : undefined;
    }
    default:
      return undefined;
  }
}

function emitLiftedFunction(
  declaration: IrVariableDeclaration,
  expression: Extract<IrExpression, { kind: 'function' }>,
  context: EmitContext,
): string {
  rejectPortableTaskExecution(expression.execution);
  const callback = declaration.type?.kind === 'named' && context.callbackTypes.has(declaration.type.name);
  const returns = expression.returns ?? (callback ? primitive('Float') : undefined);
  if (!returns) {
    throw new RustEmissionError(`${declaration.origin.source}: cannot infer return type for ${declaration.name}`);
  }
  const nextContext = functionContext(context, declaration.name, expression, returns);
  registerParameters(expression.parameters, nextContext, callback ? [primitive('Float')] : []);
  registerLocalTypes(expression.body, nextContext);
  const forwardClosureSlots = emitForwardClosureSlotDeclarations(nextContext);
  const utf16Views = prepareParameterUtf16Views(expression.parameters, expression.body, nextContext);
  const parameters = expression.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, callback ? primitive('Float') : undefined, expression),
  );
  const body = expression.expression
    ? `{\n${indent([...forwardClosureSlots, ...utf16Views, `return ${emitExpression(expression.expression, nextContext, returns)};`].join('\n'))}\n}`
    : emitStatementsAsBlock(expression.body, nextContext, [...forwardClosureSlots, ...utf16Views]);
  return `${emitAnonymousDefinitions(nextContext)}${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> ${emitType(returns, nextContext)} ${body}`;
}

function emitFunctionDeclaration(declaration: IrFunctionDeclaration, context: EmitContext): string {
  if (declaration.execution.kind === 'portableTask') {
    return emitPortableTaskFunctionDeclaration(declaration, context);
  }
  rejectPortableTaskExecution(declaration.execution);
  const callbackTypeParameters = inferCallbackTypeParameters(declaration);
  const reachableBody = staticallyReachableStatements(declaration.body, context);
  const contextOwner = { ...declaration, body: reachableBody };
  const nextContext = {
    ...functionContext(context, declaration.name, contextOwner, declaration.returns),
    callbackArgumentStorage: inferCallbackArgumentStorage(declaration, callbackTypeParameters),
    callbackTypeParameters,
    lexicalTypeParameters: new Set(declaration.typeParameters),
  };
  for (const parameter of declaration.parameters) {
    const resolved = resolveSemanticType(parameter.type, nextContext);
    if (
      resolved?.kind === 'anonymous' &&
      (parameter.type.kind === 'anonymous' || isStructuralUtilityType(parameter.type))
    ) {
      registerContextualAnonymousTypes(resolved, nextContext, declaration.name);
    }
  }
  registerParameters(declaration.parameters, nextContext);
  nextContext.entityTypeParameters = inferEntityTypeParameters(reachableBody, declaration.typeParameters, nextContext);
  registerLocalTypes(reachableBody, nextContext);
  const forwardClosureSlots = emitForwardClosureSlotDeclarations(nextContext);
  nextContext.entityTypeParameters = new Set([
    ...nextContext.entityTypeParameters,
    ...inferEntityTypeParameters(reachableBody, declaration.typeParameters, nextContext),
  ]);
  const parameters = declaration.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, undefined, declaration),
  );
  const defaults = declaration.parameters.flatMap((parameter) => {
    if (!parameter.initializer) return [];
    if (parameter.type.kind === 'nullable' && isNullishExpression(parameter.initializer)) return [];
    const name = safeName(parameter.name);
    return [`let ${name} = ${name}.unwrap_or(${emitExpression(parameter.initializer, nextContext, parameter.type)});`];
  });
  const utf16Views = prepareParameterUtf16Views(declaration.parameters, reachableBody, nextContext);
  const body = emitStatementsAsBlock(
    defaults.length > 0
      ? [{ declarations: [], kind: 'variable' } as IrStatement, ...declaration.body]
      : declaration.body,
    nextContext,
    [...forwardClosureSlots, ...defaults, ...utf16Views],
  );
  const signature = [...parameters, emitType(declaration.returns, nextContext)].join(' ');
  const effectiveTypeParameters = declaration.typeParameters.filter((parameter) =>
    new RegExp(`\\b${parameter}\\b`, 'u').test(signature),
  );
  const generics =
    effectiveTypeParameters.length > 0
      ? `<${effectiveTypeParameters
          .map((parameter) =>
            callbackTypeParameters.has(parameter)
              ? `${parameter}: crate::FlightCallback`
              : nextContext.entityTypeParameters.has(parameter)
                ? `${parameter}: Clone + ${entityTraitTypePath(nextContext)}`
                : [...nextContext.entityRuntimeGenericSlotTypes].some((slot) =>
                      new RegExp(`\\b${slot}<[^>]*\\b${parameter}\\b`, 'u').test(signature),
                    )
                  ? `${parameter}: Clone + Send + 'static`
                  : `${parameter}: Clone`,
          )
          .join(', ')}>`
      : '';
  return `${emitAnonymousDefinitions(nextContext)}${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}${generics}(${parameters.join(', ')}) -> ${emitType(declaration.returns, nextContext)} ${body}`;
}

function emitPortableTaskFunctionDeclaration(declaration: IrFunctionDeclaration, context: EmitContext): string {
  const execution = declaration.execution;
  if (execution.kind !== 'portableTask') {
    throw new RustEmissionError(`${declaration.name}: expected portable task execution`);
  }
  if (declaration.returns.kind !== 'task') {
    throw new RustEmissionError(`${declaration.name}: portable task execution requires a task return type`);
  }
  if (typeContainsDynamic(declaration.returns.output)) {
    const { column, lexicalPath, line, source } = execution.origin;
    throw new RustEmissionError(
      `${source}:${String(line)}:${String(column)}: portableTask ${lexicalPath}: async output type is not recovered`,
    );
  }
  if (declaration.typeParameters.length > 0) {
    throw new RustEmissionError(`${declaration.name}: generic portable task lowering is not implemented`);
  }
  const output = declaration.returns.output;
  const reachableBody = staticallyReachableStatements(declaration.body, context);
  const contextOwner = { ...declaration, body: reachableBody };
  const nextContext: EmitContext = {
    ...functionContext(context, declaration.name, contextOwner, output),
    lexicalTypeParameters: new Set(),
    taskOutputType: output,
  };
  registerParameters(declaration.parameters, nextContext);
  registerLocalTypes(reachableBody, nextContext);
  const forwardClosureSlots = emitForwardClosureSlotDeclarations(nextContext);
  const parameters = declaration.parameters.map((parameter) =>
    emitParameter(parameter, nextContext, undefined, declaration, false),
  );
  const defaults = declaration.parameters.flatMap((parameter) => {
    if (!parameter.initializer) return [];
    if (parameter.type.kind === 'nullable' && isNullishExpression(parameter.initializer)) return [];
    const name = safeName(parameter.name);
    return [`let ${name} = ${name}.unwrap_or(${emitExpression(parameter.initializer, nextContext, parameter.type)});`];
  });
  const utf16Views = prepareParameterUtf16Views(declaration.parameters, reachableBody, nextContext);
  if (output.kind !== 'primitive' || output.name !== 'Void') {
    if (!reachableBody.some((statement) => statementAlwaysReturns(statement, nextContext))) {
      throw new RustEmissionError(
        `${declaration.name}: portable task has a non-void output without a guaranteed return`,
      );
    }
  }
  const prefix = [...forwardClosureSlots, ...defaults, ...utf16Views];
  const bodyStatements = emitStatementsAsBlock(reachableBody, nextContext, prefix);
  const body =
    output.kind === 'primitive' && output.name === 'Void'
      ? bodyStatements.replace(/\n\}$/u, '\n  Ok(())\n}')
      : bodyStatements;
  const origin = emitTaskOrigin(execution.origin);
  return `${emitAnonymousDefinitions(nextContext)}${declaration.exported ? 'pub ' : ''}fn ${snakeCase(declaration.name)}(${parameters.join(', ')}) -> crate::FlightTask<${emitType(output, nextContext)}> {\n${indent(`crate::FlightTask::start(async move ${body}, ${origin})`)}\n}`;
}

function emitParameter(
  parameter: IrParameter,
  context: EmitContext,
  fallbackType: IrType | undefined,
  owner: IrClassDeclaration | IrExpression | IrFunctionDeclaration,
  borrowRecords = true,
): string {
  const type = contextualParameterType(parameter.type, fallbackType, context);
  const emitted = emitType(type, context);
  const optional = parameter.optional || parameter.initializer;
  const resolved = resolveSemanticType(type, context);
  const resolvedReference =
    resolved?.kind === 'nullable' ? (resolveSemanticType(resolved.inner, context) ?? resolved.inner) : resolved;
  const sharedHandle = isSharedHandleType(type, context);
  const referenceLike =
    resolvedReference?.kind === 'anonymous' ||
    resolvedReference?.kind === 'array' ||
    resolvedReference?.kind === 'function' ||
    resolvedReference?.kind === 'union' ||
    (resolvedReference?.kind === 'named' &&
      (resolvedReference.name === 'RustMap' ||
        resolvedReference.name === 'RustSet' ||
        resolvedReference.name === 'FlightRegex')) ||
    (resolvedReference?.kind === 'named' && Boolean(typedArrayType(resolvedReference.name)));
  const captured = capturesParameterInReturnedClosure(owner, parameter.name);
  const stored = resolved?.kind === 'function' && storesParameter(owner, parameter.name);
  const concreteCallbackHandle = isConcreteCallbackHandleType(type, context);
  if (
    resolved?.kind === 'function' &&
    !concreteCallbackHandle &&
    !optional &&
    !parameter.rest &&
    borrowRecords &&
    !captured &&
    !stored
  ) {
    context.rawClosureNames.add(parameter.name);
    context.borrowedNames.add(parameter.name);
    return `${safeName(parameter.name)}: &mut impl FnMut(${resolved.parameters
      .map((item) => emitType(item, context))
      .join(', ')}) -> ${emitType(resolved.returns, context)}`;
  }
  const assigned = referenceLike ? context.mutatedNames.has(parameter.name) : assignsName(owner, parameter.name);
  const ownedReadHandle =
    sharedHandle || concreteCallbackHandle || (type.kind === 'named' && type.name === 'Signal' && !assigned);
  const borrowed =
    referenceLike && !ownedReadHandle && !optional && !parameter.rest && borrowRecords && !captured && !stored;
  if (borrowed) context.borrowedNames.add(parameter.name);
  const name = `${assigned && !borrowed ? 'mut ' : ''}${safeName(parameter.name)}`;
  const storage = optional && type.kind !== 'nullable' ? `Option<${emitted}>` : emitted;
  return `${name}: ${borrowed ? `${assigned ? '&mut ' : '&'}${storage}` : storage}`;
}

function emitStatementsAsBlock(
  statements: IrStatement[],
  context: EmitContext,
  prefix: string[] = [],
  suffix: string[] = [],
): string {
  const lines = [...prefix];
  const namesUsedLater = identifierNamesUsedLater(statements);
  const nullCheckedNames = new Set(context.nullCheckedNames);
  collectNullCheckedIdentifierNames(statements, nullCheckedNames);
  let activeContext: EmitContext = { ...context, nullCheckedNames };
  for (const [index, statement] of statements.entries()) {
    const statementContext = contextPreservingReferencedNames(
      statement,
      namesUsedLater[index] ?? new Set(),
      activeContext,
    );
    lines.push(...emitStatement(statement, statementContext));
    if (statementAlwaysReturns(statement, statementContext)) break;
    if (
      statement.kind === 'if' &&
      !statement.otherwise &&
      statementAlwaysReturns(statement.consequent, statementContext)
    ) {
      activeContext = narrowTypeofContexts(statement.condition, activeContext).whenFalse;
    }
  }
  lines.push(...suffix);
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
      const loopContext = contextPreservingLoopReads(statement, context);
      const condition = emitCondition(statement.condition, loopContext);
      const conditionCheck = `if !(${condition}) { break; }`;
      const bodyContext = {
        ...loopContext,
        continueEpilogue: [conditionCheck],
      };
      return ['loop {', indent(emitStatement(statement.body, bodyContext).join('\n')), indent(conditionCheck), '}'];
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
      const collectionType = iterableType?.kind === 'nullable' ? iterableType.inner : iterableType;
      const elementType =
        collectionType?.kind === 'array'
          ? collectionType.element
          : collectionType?.kind === 'named' && collectionType.name === 'RustMap'
            ? ({
                arguments: collectionType.arguments,
                kind: 'named',
                name: 'RustTuple2',
              } as const)
            : collectionType?.kind === 'named' && collectionType.name === 'RustSet'
              ? collectionType.arguments[0]
              : undefined;
      const iterable =
        statement.iterable.kind === 'identifier' && context.mutexCollectionNames.has(statement.iterable.name)
          ? emitCollectionPlace(statement.iterable, context)
          : emitExpression(statement.iterable, context);
      const iterablePlace =
        iterableType?.kind === 'nullable'
          ? `${parenthesize(iterable)}.as_ref().expect("TypeScript nullable iterable was not narrowed")`
          : parenthesize(iterable);
      const loopContext: EmitContext = {
        ...contextPreservingLoopReads(statement.body, context),
        continueEpilogue: [],
        placeAliases: new Map(context.placeAliases),
        symbolTypes: new Map(context.symbolTypes),
      };
      if (elementType) loopContext.symbolTypes.set(statement.variable, elementType);
      const bindings = statement.bindings.flatMap((binding) => emitLocalVariable(binding, loopContext));
      const bodyStatements = statement.body.kind === 'block' ? statement.body.statements : [statement.body];
      const body = `{\n${indent(
        [...bindings, ...bodyStatements.flatMap((item) => emitStatement(item, loopContext))].join('\n'),
      )}\n}`;
      const mutable = collectMutatedNames(statement.body, context.mutatingFunctions).has(statement.variable);
      return [
        `for ${mutable ? 'mut ' : ''}${safeName(statement.variable)} in ${iterablePlace}.iter().cloned() ${body}`,
      ];
    }
    case 'forIn': {
      if (statement.enumeration !== 'direct-record') {
        throw new RustEmissionError('dynamic for-in Rust enumeration is not implemented');
      }
      const objectType = inferIrExpressionType(statement.object, context);
      const resolved = objectType ? (resolveSemanticType(objectType, context) ?? objectType) : undefined;
      if (resolved?.kind !== 'named' || resolved.name !== 'RustMap') {
        throw new RustEmissionError('direct for-in Rust enumeration requires a string-keyed record');
      }
      const object = emitExpression(statement.object, context);
      const loopContext: EmitContext = {
        ...contextPreservingLoopReads(statement.body, context),
        continueEpilogue: [],
        placeAliases: new Map(context.placeAliases),
        symbolTypes: new Map(context.symbolTypes),
      };
      loopContext.symbolTypes.set(statement.variable, {
        kind: 'primitive',
        name: 'String',
      });
      const body = emitStatementAsBlock(statement.body, loopContext);
      return [
        '{',
        indent(
          [
            `let __flight_keys: Vec<String> = ${parenthesize(object)}.iter().map(|(key, _)| key.clone()).collect();`,
            `for ${safeName(statement.variable)} in __flight_keys ${body}`,
          ].join('\n'),
        ),
        '}',
      ];
    }
    case 'if': {
      const constant = evaluateStaticBoolean(statement.condition, context);
      if (constant === true) return emitStatement(statement.consequent, context);
      if (constant === false) return statement.otherwise ? emitStatement(statement.otherwise, context) : [];
      const narrowed = narrowTypeofContexts(statement.condition, context);
      const lines = [
        `if ${emitCondition(statement.condition, context)} ${emitStatementAsBlock(statement.consequent, narrowed.whenTrue)}`,
      ];
      if (statement.otherwise) {
        lines[0] += ` else ${emitStatementAsBlock(statement.otherwise, narrowed.whenFalse)}`;
      }
      return lines;
    }
    case 'return':
      if (context.taskOutputType) {
        if (!statement.expression) return [context.captureReturns ? 'return Ok(Some(()));' : 'return Ok(());'];
        const actual = inferIrExpressionType(statement.expression, context);
        const resolved = promiseType(actual, context);
        if (resolved) {
          const task = emitExpression(statement.expression, context, resolved);
          return [context.captureReturns ? `return ${task}.await.map(Some);` : `return ${task}.await;`];
        }
        const value = emitExpression(statement.expression, context, context.taskOutputType);
        return [context.captureReturns ? `return Ok(Some(${value}));` : `return Ok(${value});`];
      }
      return context.captureReturns
        ? [
            statement.expression
              ? `return Some(${emitReturnExpression(statement.expression, context)});`
              : 'return Some(());',
          ]
        : [statement.expression ? `return ${emitReturnExpression(statement.expression, context)};` : 'return;'];
    case 'switch':
      return emitSwitchStatement(statement, context);
    case 'throw':
      if (context.taskOutputType) {
        return [`return Err(crate::FlightTaskError::Rejection(${emitTaskRejection(statement.expression, context)}));`];
      }
      return [`panic!("{}", ${emitThrowMessage(statement.expression, context)});`];
    case 'try': {
      if (statement.execution === 'portableTask') {
        if (!context.taskOutputType) {
          throw portableTaskTryError(statement, 'portable task try/catch requires portable task execution');
        }
        return emitPortableTaskTryStatement(statement, context);
      }
      if (statement.catchBody) {
        if (
          ['break', 'continue'].some(
            (kind) =>
              containsStatementKind(statement.tryBody, kind as IrStatement['kind']) ||
              containsStatementKind(statement.catchBody, kind as IrStatement['kind']),
          )
        ) {
          throw new RustEmissionError('try/catch with escaping loop control is not implemented');
        }
        const capturesReturn =
          containsStatementKind(statement.tryBody, 'return') || containsStatementKind(statement.catchBody, 'return');
        const catchContext: EmitContext = {
          ...context,
          captureReturns: capturesReturn,
          symbolTypes: new Map(context.symbolTypes),
        };
        if (statement.catchName) {
          catchContext.symbolTypes.set(statement.catchName, {
            kind: 'dynamic',
          });
        }
        const catchLines = [
          ...(statement.catchName ? [`let ${safeName(statement.catchName)} = crate::OpaqueHostValue::Object;`] : []),
          ...emitStatement(statement.catchBody, catchContext),
        ];
        const returnType = context.currentReturnType;
        if (capturesReturn && !returnType) {
          throw new RustEmissionError('try/catch return capture requires an inferred function return type');
        }
        const tryContext: EmitContext = {
          ...context,
          captureReturns: capturesReturn,
        };
        const caught = capturesReturn
          ? [
              `let __flight_try_return: Option<${emitType(returnType!, context)}> = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Option<${emitType(returnType!, context)}> {`,
              indent(emitStatement(statement.tryBody, tryContext).join('\n')),
              indent('None'),
              '})) {',
              indent('Ok(value) => value,'),
              indent(
                `Err(_) => (|| -> Option<${emitType(returnType!, context)}> {\n${indent(
                  [...catchLines, 'None'].join('\n'),
                )}\n})(),`,
              ),
              '};',
            ].join('\n')
          : [
              'match std::panic::catch_unwind(std::panic::AssertUnwindSafe(||',
              emitStatementAsBlock(statement.tryBody, context),
              ')) {',
              indent('Ok(_) => {}'),
              indent(`Err(_) => {\n${indent(catchLines.join('\n'))}\n}`),
              '}',
            ].join(' ');
        return [
          caught,
          ...(statement.finallyBody ? emitStatement(statement.finallyBody, context) : []),
          ...(capturesReturn
            ? statementAlwaysReturns(statement, context)
              ? ['return __flight_try_return.expect("TypeScript try/catch completed without returning");']
              : ['if let Some(__flight_return) = __flight_try_return { return __flight_return; }']
            : []),
        ];
      }
      return [
        ...emitStatement(statement.tryBody, context),
        ...(statement.finallyBody ? emitStatement(statement.finallyBody, context) : []),
      ];
    }
    case 'variable':
      return statement.declarations.flatMap((variable) => emitLocalVariable(variable, context));
    case 'while': {
      const loopContext = contextPreservingLoopReads(statement, context);
      return [
        `while ${emitCondition(statement.condition, loopContext)} ${emitStatementAsBlock(statement.body, {
          ...loopContext,
          continueEpilogue: [],
        })}`,
      ];
    }
  }
}

function emitPortableTaskTryStatement(
  statement: Extract<IrStatement, { kind: 'try' }>,
  context: EmitContext,
): string[] {
  if (!statement.catchBody) {
    throw portableTaskTryError(statement, 'portable task try/finally lowering is not implemented');
  }
  if (statement.catchName) {
    throw portableTaskTryError(statement, 'portable task catch bindings are not implemented');
  }
  if (statement.finallyBody) {
    throw portableTaskTryError(statement, 'portable task try/catch/finally lowering is not implemented');
  }
  if (
    ['break', 'continue'].some(
      (kind) =>
        containsRegionStatementKind(statement.tryBody, kind as IrStatement['kind']) ||
        containsRegionStatementKind(statement.catchBody!, kind as IrStatement['kind']),
    )
  ) {
    throw portableTaskTryError(statement, 'portable task try/catch with escaping loop control is not implemented');
  }

  const tryAlwaysReturns = statementAlwaysReturns(statement.tryBody, context);
  const catchAlwaysReturns = statementAlwaysReturns(statement.catchBody, context);
  if (tryAlwaysReturns && catchAlwaysReturns) {
    const tryBody = emitStatement(statement.tryBody, {
      ...context,
      captureReturns: false,
    }).join('\n');
    const catchBody = emitStatement(statement.catchBody, {
      ...context,
      captureReturns: false,
    }).join('\n');
    return [
      [
        'return match (async {',
        indent(tryBody),
        '}).await {',
        indent('Ok(__flight_value) => Ok(__flight_value),'),
        indent(`Err(crate::FlightTaskError::Rejection(_)) => (async {\n${indent(catchBody)}\n}).await,`),
        indent('Err(__flight_error) => Err(__flight_error),'),
        '};',
      ].join('\n'),
    ];
  }

  const capturesReturn =
    containsRegionStatementKind(statement.tryBody, 'return') ||
    containsRegionStatementKind(statement.catchBody, 'return');
  const tryContext: EmitContext = {
    ...context,
    captureReturns: capturesReturn,
  };
  const catchContext: EmitContext = {
    ...context,
    captureReturns: capturesReturn,
  };
  const tryBody = emitStatement(statement.tryBody, tryContext).join('\n');
  const catchBody = emitStatement(statement.catchBody, catchContext).join('\n');
  const returnType = context.currentReturnType;
  if (capturesReturn && !returnType) {
    throw portableTaskTryError(statement, 'portable task try/catch return capture requires an inferred output type');
  }
  const regionType = capturesReturn ? `Option<${emitType(returnType!, context)}>` : '()';
  const fallthrough = `Ok::<${regionType}, crate::FlightTaskError>(${capturesReturn ? 'None' : '()'})`;
  const caught = [
    capturesReturn ? `let __flight_try_return: ${regionType} = match (async {` : 'match (async {',
    indent([tryBody, fallthrough].join('\n')),
    '}).await {',
    indent(`Ok(__flight_value) => __flight_value,`),
    indent(
      `Err(crate::FlightTaskError::Rejection(_)) => (async {\n${indent(
        [catchBody, fallthrough].join('\n'),
      )}\n}).await?,`,
    ),
    indent('Err(__flight_error) => return Err(__flight_error),'),
    '};',
  ].join('\n');
  return capturesReturn
    ? [caught, 'if let Some(__flight_return) = __flight_try_return { return Ok(__flight_return); }']
    : [caught];
}

function portableTaskTryError(statement: Extract<IrStatement, { kind: 'try' }>, reason: string): RustEmissionError {
  const { column, line, source } = statement.origin;
  return new RustEmissionError(`${source}:${String(line)}:${String(column)}: ${reason}`);
}

function emitReturnExpression(expression: IrExpression, context: EmitContext): string {
  const expected = context.currentReturnType;
  if (!expected || expected.kind !== 'nullable') {
    const root = expressionRootIdentifier(expression);
    const actual = root ? context.symbolTypes.get(root) : undefined;
    const actualExpressionType = inferIrExpressionType(expression, context);
    const resolvedActualExpression = resolveSemanticType(actualExpressionType, context);
    const resolvedExpected = resolveSemanticType(expected, context);
    if (
      expected &&
      expression.kind !== 'object' &&
      actualExpressionType &&
      resolvedActualExpression?.kind === 'anonymous' &&
      resolvedExpected?.kind === 'anonymous'
    ) {
      const projected = emitStructuralProjectionArgument(
        emitPlaceExpression(expression, context),
        actualExpressionType,
        expected,
        context,
      );
      if (projected) return projected;
    }
    if (
      expected &&
      root &&
      ((actual?.kind === 'nullable' && semanticTypesEqual(actual.inner, expected, context)) ||
        (actual && context.nonNullableNames.has(root) && semanticTypesEqual(actual, expected, context))) &&
      !isCopyType(expected, context)
    ) {
      return `${parenthesize(emitExpression(expression, context, expected))}.clone()`;
    }
    if (
      expected &&
      expression.kind === 'identifier' &&
      context.borrowedNames.has(expression.name) &&
      !isCopyType(expected, context)
    ) {
      return `${emitIdentifier(expression.name, context)}.clone()`;
    }
    if (
      expected &&
      expression.kind === 'identifier' &&
      !context.borrowedNames.has(expression.name) &&
      !context.constantNames.has(expression.name) &&
      !context.sharedCaptureNames.has(expression.name) &&
      !context.mutexCollectionNames.has(expression.name) &&
      !context.mutexValueNames.has(expression.name) &&
      typeKey(context.symbolTypes.get(expression.name) ?? { kind: 'dynamic' }) === typeKey(expected)
    ) {
      return emitIdentifier(expression.name, context);
    }
    return emitExpression(expression, context, expected);
  }
  return emitNullableValue(expression, context, expected);
}

function emitNullableValue(
  expression: IrExpression,
  context: EmitContext,
  expected: Extract<IrType, { kind: 'nullable' }>,
): string {
  if (expression.kind === 'conditional') {
    return `if ${emitCondition(expression.condition, context)} { ${emitNullableValue(expression.whenTrue, context, expected)} } else { ${emitNullableValue(expression.whenFalse, context, expected)} }`;
  }
  if (isNullishExpression(expression)) return emitExpression(expression, context, expected);
  const actual = inferIrExpressionType(expression, context);
  return actual?.kind === 'nullable'
    ? emitExpression(expression, context, expected)
    : `Some(${emitExpression(expression, context, expected.inner)})`;
}

function emitSwitchStatement(statement: Extract<IrStatement, { kind: 'switch' }>, context: EmitContext): string[] {
  const root = expressionRootIdentifier(statement.expression);
  const reusedInCase = root && statement.cases.some((switchCase) => containsIdentifier(switchCase.statements, root));
  const valueContext = reusedInCase
    ? { ...context, preservedNames: new Set([...context.preservedNames, root]) }
    : context;
  const value = emitExpression(statement.expression, valueContext);
  const defaultIndex = statement.cases.findIndex((switchCase) => !switchCase.expression);
  const selections: string[] = [];
  statement.cases.forEach((switchCase, index) => {
    if (!switchCase.expression) return;
    selections.push(
      `${selections.length === 0 ? 'if' : 'else if'} __switch_value == ${emitExpression(
        switchCase.expression,
        context,
      )} { ${String(index)}_usize }`,
    );
  });
  const fallback = defaultIndex >= 0 ? String(defaultIndex) : String(statement.cases.length);
  const clauses = statement.cases.map((switchCase, index) => {
    const body = switchCase.statements.flatMap((item) => emitSwitchCaseStatement(item, context));
    return `if __flight_case <= ${String(index)}_usize {\n${indent(body.join('\n'))}\n}`;
  });
  const exhaustiveReturn = switchAlwaysReturns(statement, context);
  const exhaustiveExit = switchAlwaysExits(statement, context);
  return [
    '{',
    indent(
      [
        `let __switch_value = ${value};`,
        `let __flight_case = ${selections.join(' ')}${selections.length > 0 ? ' else ' : ''}{ ${fallback}_usize };`,
        "'__flight_switch: {",
        indent(clauses.join('\n')),
        ...(exhaustiveReturn ? ['unreachable!("exhaustive TypeScript switch completed without returning");'] : []),
        ...(!exhaustiveReturn && exhaustiveExit
          ? ['unreachable!("exhaustive TypeScript switch completed without exiting");']
          : []),
        '}',
      ].join('\n'),
    ),
    '}',
  ];
}

function emitSwitchCaseStatement(statement: IrStatement, context: EmitContext): string[] {
  if (statement.kind === 'break') return ["break '__flight_switch;"];
  if (statement.kind === 'block') {
    return [
      `{\n${indent(statement.statements.flatMap((item) => emitSwitchCaseStatement(item, context)).join('\n'))}\n}`,
    ];
  }
  if (statement.kind === 'if') {
    let value = `if ${emitCondition(statement.condition, context)} {\n${indent(
      emitSwitchCaseStatement(statement.consequent, context).join('\n'),
    )}\n}`;
    if (statement.otherwise) {
      value += ` else {\n${indent(emitSwitchCaseStatement(statement.otherwise, context).join('\n'))}\n}`;
    }
    return [value];
  }
  return emitStatement(statement, context);
}

function emitForStatement(statement: Extract<IrStatement, { kind: 'for' }>, context: EmitContext): string[] {
  const loopContext = contextPreservingLoopReads(statement, context);
  const initializer = Array.isArray(statement.initializer)
    ? statement.initializer.flatMap((variable) => emitLocalVariable(variable, context))
    : statement.initializer
      ? [`${emitExpression(statement.initializer, context)};`]
      : [];
  const condition = statement.condition ? emitCondition(statement.condition, loopContext) : 'true';
  const increment = statement.increment ? `${emitExpression(statement.increment, loopContext)};` : undefined;
  const body = emitStatementAsBlock(
    statement.body,
    { ...loopContext, continueEpilogue: increment ? [increment] : [] },
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
  return emitStatementsAsBlock(statements, context, [], increment ? [`${emitExpression(increment, context)};`] : []);
}

function emitForwardClosureSlotDeclarations(context: EmitContext): string[] {
  return [...context.forwardClosureCaptureNames].map((name) => {
    const type = context.symbolTypes.get(name);
    const slot = context.recursiveClosureSlots.get(name);
    if (!type || !slot) throw new RustEmissionError(`cannot recover forward closure capture ${name}`);
    return `let ${slot}: std::sync::Arc<std::sync::Mutex<Option<${emitType(type, context)}>>> = std::sync::Arc::new(std::sync::Mutex::new(None));`;
  });
}

function emitLocalVariable(variable: IrVariable, context: EmitContext): string[] {
  const transferredBindings = collectTransferredObjectBindings(variable.initializer);
  const mutable =
    variable.mutable ||
    variable.initializer?.kind === 'function' ||
    context.mutatedNames.has(variable.name) ||
    transferredBindings.some((binding) => context.mutatedNames.has(binding.source));
  if (!variable.initializer) {
    const inferred = variable.type ?? context.symbolTypes.get(variable.name);
    if (!inferred) throw new RustEmissionError(`cannot infer uninitialized local ${variable.name}`);
    const nullable =
      inferred.kind !== 'nullable' &&
      (isReferenceLike(inferred, context) ||
        (inferred.kind === 'named' && !context.enumNames.has(inferred.name) && !typedArrayType(inferred.name)));
    const type = nullable ? ({ inner: inferred, kind: 'nullable' } as const) : inferred;
    context.symbolTypes.set(variable.name, type);
    if (context.sharedCaptureNames.has(variable.name)) {
      return [
        `let ${safeName(variable.name)}: std::sync::Arc<std::sync::Mutex<${emitType(type, context)}>> = std::sync::Arc::new(std::sync::Mutex::new(None));`,
      ];
    }
    return [
      nullable
        ? `let ${mutable ? 'mut ' : ''}${safeName(variable.name)}: ${emitType(type, context)} = None;`
        : `let ${mutable ? 'mut ' : ''}${safeName(variable.name)}: ${emitType(type, context)};`,
    ];
  }
  if (variable.type?.kind === 'named' && variable.type.name === 'FlightNever') {
    context.symbolTypes.set(variable.name, variable.type);
    return [];
  }
  const callbackStorage = context.callbackArgumentStorage.get(variable.name);
  const expected: IrType | undefined = callbackStorage
    ? {
        inner: {
          arguments: [{ arguments: [], kind: 'named', name: callbackStorage }],
          kind: 'named',
          name: 'FlightCallbackArgs',
        },
        kind: 'nullable',
      }
    : variable.type;
  const entitySpreadType = inferEntitySpreadType(variable.initializer, context);
  const object = unwrapCasts(variable.initializer);
  const contextualObject = object.kind === 'object' ? inferContextualObjectType(object, context, expected) : undefined;
  const inferredValue =
    (expected?.kind === 'dynamic' ? contextualObject : expected) ??
    contextualObject ??
    entitySpreadType ??
    inferIrExpressionType(variable.initializer, context) ??
    (evaluatesToNullish(variable.initializer, context)
      ? ({ inner: { kind: 'dynamic' }, kind: 'nullable' } as const)
      : undefined);
  const nullCheckedMapElement = context.nullCheckedNames.has(variable.name)
    ? inferMapElementLookup(variable.initializer, context)
    : undefined;
  const inferred =
    nullCheckedMapElement && nullCheckedMapElement.type.kind !== 'nullable'
      ? ({ inner: nullCheckedMapElement.type, kind: 'nullable' } as const)
      : inferredValue;
  if (inferred?.kind === 'anonymous') registerInferredObjectType(inferred, context);
  const forwardCaptureSlot = context.forwardClosureCaptureNames.has(variable.name)
    ? context.recursiveClosureSlots.get(variable.name)
    : undefined;
  if (context.timerHandleNames.has(variable.name)) {
    const timerType: IrType = {
      inner: { arguments: [], kind: 'named', name: 'FlightTimeout' },
      kind: 'nullable',
    };
    context.symbolTypes.set(variable.name, timerType);
    const initializer =
      variable.initializer.kind === 'literal' && variable.initializer.value === 0
        ? 'None'
        : emitExpression(variable.initializer, context, timerType);
    const sharedCapture = context.sharedCaptureNames.has(variable.name);
    const declaration = sharedCapture
      ? `let ${safeName(variable.name)}: std::sync::Arc<std::sync::Mutex<${emitType(timerType, context)}>> = std::sync::Arc::new(std::sync::Mutex::new(${initializer}));`
      : `let ${mutable ? 'mut ' : ''}${safeName(variable.name)}: ${emitType(timerType, context)} = ${initializer};`;
    return forwardCaptureSlot
      ? [declaration, `*${forwardCaptureSlot}.lock().unwrap() = Some(${safeName(variable.name)}.clone());`]
      : [declaration];
  }
  const sharedCapture = context.sharedCaptureNames.has(variable.name);
  if (
    inferred?.kind === 'function' &&
    variable.initializer.kind === 'function' &&
    containsIdentifier(variable.initializer, variable.name)
  ) {
    const slot = forwardCaptureSlot ?? `__flight_recursive_${safeName(variable.name)}`;
    const recursiveContext: EmitContext = {
      ...context,
      recursiveClosureSlots: new Map(context.recursiveClosureSlots).set(variable.name, slot),
    };
    const type = emitType(inferred, context);
    const initializer = emitExpression(variable.initializer, recursiveContext, inferred);
    context.symbolTypes.set(variable.name, inferred);
    return [
      ...(forwardCaptureSlot
        ? []
        : [
            `let ${slot}: std::sync::Arc<std::sync::Mutex<Option<${type}>>> = std::sync::Arc::new(std::sync::Mutex::new(None));`,
          ]),
      `let ${mutable ? 'mut ' : ''}${safeName(variable.name)}: ${type} = ${initializer};`,
      `*${slot}.lock().unwrap() = Some(${safeName(variable.name)}.clone());`,
    ];
  }
  if (
    inferred &&
    isReferenceLike(inferred, context) &&
    (variable.initializer.kind === 'property' || variable.initializer.kind === 'identifier')
  ) {
    context.symbolTypes.set(variable.name, inferred);
    context.placeAliases.set(variable.name, variable.initializer);
    return [];
  }
  const initializer = nullCheckedMapElement
    ? emitElement(nullCheckedMapElement.expression, context)
    : emitExpression(
        entitySpreadType ? unwrapCasts(variable.initializer) : variable.initializer,
        context,
        (expected?.kind === 'dynamic' ? inferred : expected) ??
          (inferred?.kind === 'nullable' && variable.initializer.kind === 'call'
            ? inferred
            : mutable && inferred?.kind === 'primitive' && inferred.name === 'String'
              ? inferred
              : variable.initializer.kind === 'function' || containsObjectLiteral(variable.initializer)
                ? inferred
                : undefined),
      );
  if (inferred) context.symbolTypes.set(variable.name, inferred);
  if (!mutable && evaluatesToNullish(variable.initializer, context)) {
    context.knownNullNames.add(variable.name);
  }
  const annotationType =
    (nullCheckedMapElement ? inferred : expected) ??
    (variable.initializer.kind === 'function' ||
    variable.initializer.kind === 'new' ||
    evaluatesToNullish(variable.initializer, context)
      ? inferred
      : undefined);
  const annotation =
    annotationType && annotationType.kind !== 'dynamic' ? `: ${emitType(annotationType, context)}` : '';
  for (const binding of transferredBindings) {
    if (!context.mutatedNames.has(binding.source)) continue;
    context.placeAliases.set(binding.source, {
      kind: 'property',
      name: binding.field,
      object: { kind: 'identifier', name: variable.name },
      optional: false,
    });
  }
  if (sharedCapture && inferred) {
    const declaration = `let ${safeName(variable.name)}: std::sync::Arc<std::sync::Mutex<${emitType(inferred, context)}>> = std::sync::Arc::new(std::sync::Mutex::new(${initializer}));`;
    return forwardCaptureSlot
      ? [declaration, `*${forwardCaptureSlot}.lock().unwrap() = Some(${safeName(variable.name)}.clone());`]
      : [declaration];
  }
  const declaration = `let ${mutable ? 'mut ' : ''}${safeName(variable.name)}${annotation} = ${initializer};`;
  return forwardCaptureSlot
    ? [declaration, `*${forwardCaptureSlot}.lock().unwrap() = Some(${safeName(variable.name)}.clone());`]
    : [declaration];
}

function collectTransferredObjectBindings(
  initializer: IrExpression | undefined,
): Array<{ field: string; source: string }> {
  let expression = initializer;
  while (expression?.kind === 'cast') expression = expression.expression;
  if (expression?.kind === 'call') {
    expression = expression.arguments[0];
    while (expression?.kind === 'cast') expression = expression.expression;
  }
  if (expression?.kind !== 'object') return [];
  return expression.properties.flatMap((property) =>
    property.kind === 'property' && property.value.kind === 'identifier'
      ? [{ field: property.name, source: property.value.name }]
      : [],
  );
}

function containsObjectLiteral(expression: IrExpression): boolean {
  if (expression.kind === 'object') return true;
  if (expression.kind === 'conditional') {
    return containsObjectLiteral(expression.whenTrue) || containsObjectLiteral(expression.whenFalse);
  }
  if (expression.kind === 'cast') return containsObjectLiteral(expression.expression);
  return false;
}

function evaluatesToNullish(expression: IrExpression, context: EmitContext): boolean {
  if (expression.kind === 'cast') return evaluatesToNullish(expression.expression, context);
  if (expression.kind === 'conditional') {
    const constant = evaluateStaticBoolean(expression.condition, context);
    return constant === undefined
      ? false
      : evaluatesToNullish(constant ? expression.whenTrue : expression.whenFalse, context);
  }
  return isNullishExpression(expression);
}

function unwrapCasts(expression: IrExpression): IrExpression {
  let value = expression;
  while (value.kind === 'cast') value = value.expression;
  return value;
}

function emitExpression(expression: IrExpression, context: EmitContext, expectedType?: IrType | undefined): string {
  if (expectedType?.kind === 'nullable' && !isNullishExpression(expression)) {
    const actualType = inferIrExpressionType(expression, context);
    const resolvedActual = resolveSemanticType(actualType, context) ?? actualType;
    if (resolvedActual?.kind !== 'nullable' && resolvedActual?.kind !== 'dynamic') {
      return `Some(${emitExpression(expression, context, expectedType.inner)})`;
    }
  }
  const resolvedExpectedType = resolveSemanticType(expectedType, context);
  const expectedUnion =
    expectedType?.kind === 'union'
      ? expectedType
      : resolvedExpectedType?.kind === 'union'
        ? resolvedExpectedType
        : undefined;
  const expectedUnionName =
    expectedType?.kind === 'named' ? emitNamedUnionConstructor(expectedType, context) : undefined;
  if (expectedUnion) {
    const actualType = inferIrExpressionType(expression, context);
    const resolvedActual = resolveSemanticType(actualType, context) ?? actualType;
    if (resolvedActual?.kind === 'dynamic' && resolvedActual.portable) {
      const converted = emitPortableValueToUnion(
        emitExpression(expression, context, actualType),
        resolvedActual,
        expectedUnion.variants,
        context,
        expectedUnionName,
      );
      if (converted) return converted;
    }
    if (actualType?.kind !== 'union') {
      const variantIndex = actualType
        ? expectedUnion.variants.findIndex((variant) => semanticTypesEqual(variant, actualType, context))
        : -1;
      if (variantIndex >= 0) {
        const variant = expectedUnion.variants[variantIndex]!;
        return wrapUnionValue(
          emitExpression(expression, context, variant),
          expectedUnion.variants,
          variantIndex,
          context,
          expectedUnionName,
        );
      }
    }
    if (expression.kind === 'object') {
      const matches = expectedUnion.variants.flatMap((variant, index) =>
        objectLiteralMatchesType(expression, variant, context) ? [{ index, variant }] : [],
      );
      if (matches.length === 1) {
        const match = matches[0]!;
        const resolvedVariant = resolveSemanticType(match.variant, context) ?? match.variant;
        return wrapUnionValue(
          emitExpression(expression, context, resolvedVariant.kind === 'union' ? resolvedVariant : match.variant),
          expectedUnion.variants,
          match.index,
          context,
          expectedUnionName,
        );
      }
    }
  }
  if (resolvedExpectedType?.kind === 'dynamic') {
    const actualType = inferIrExpressionType(expression, context);
    const resolvedActual = resolveSemanticType(actualType, context) ?? actualType;
    const storesEntityRuntime =
      expression.kind === 'object' &&
      expression.properties.some(
        (property) =>
          property.kind === 'computedProperty' &&
          property.key.kind === 'identifier' &&
          property.key.name === 'EntityRuntimeKey',
      );
    if (
      (expression.kind === 'object' && !storesEntityRuntime) ||
      expression.kind === 'array' ||
      expression.kind === 'function' ||
      isNullishExpression(expression) ||
      (actualType && resolvedActual?.kind !== 'dynamic' && isPortableValueInlineConversionBounded(actualType, context))
    ) {
      return emitPortableValueExpression(expression, context, actualType);
    }
  }
  switch (expression.kind) {
    case 'array': {
      const resolved = resolveSemanticType(expectedType, context);
      if (resolved?.kind === 'named' && resolved.name === 'RustTuple2') {
        if (expression.elements.length !== 2 || expression.elements.some((item) => item.kind === 'spread')) {
          throw new RustEmissionError('tuple array literal requires exactly two non-spread elements');
        }
        return `(${emitExpression(expression.elements[0]!, context, resolved.arguments[0])}, ${emitExpression(
          expression.elements[1]!,
          context,
          resolved.arguments[1],
        )})`;
      }
      const elementType = resolved?.kind === 'array' ? resolved.element : undefined;
      if (expression.elements.some((item) => item.kind === 'spread')) {
        const statements = expression.elements.map((item) =>
          item.kind === 'spread'
            ? `__flight_array.extend(${parenthesize(emitExpression(item.expression, context))}.iter().cloned());`
            : `__flight_array.push(${emitExpression(item, context, elementType)});`,
        );
        return `{ let mut __flight_array = Vec::new(); ${statements.join(' ')} __flight_array }`;
      }
      return `vec![${expression.elements.map((item) => emitExpression(item, context, elementType)).join(', ')}]`;
    }
    case 'assignment':
      return emitAssignment(expression, context);
    case 'await': {
      if (!context.taskOutputType) throw new RustEmissionError('await requires portable task execution');
      const task = promiseType(inferIrExpressionType(expression.expression, context), context);
      if (task) return `${parenthesize(emitExpression(expression.expression, context, task))}.await?`;
      const valueType = inferIrExpressionType(expression.expression, context) ?? expectedType;
      if (!valueType || typeContainsDynamic(valueType)) {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: await value type is not recovered`,
        );
      }
      const value = emitExpression(expression.expression, context, valueType);
      return `${parenthesize(`crate::FlightTask::ready(${value}, ${emitTaskOrigin(expression.origin)})`)}.await?`;
    }
    case 'binary':
      return coerceExpression(emitBinary(expression, context), expectedType);
    case 'call': {
      const call = emitCall(expression, context, expectedType);
      const actualType = inferIrExpressionType(expression, context);
      const genericCallee =
        expression.callee.kind === 'identifier' &&
        (context.functions.get(expression.callee.name)?.typeParameters.length ?? 0) > 0;
      const projected =
        actualType && expectedType && !genericCallee && !semanticTypesEqual(actualType, expectedType, context)
          ? emitStructuralProjectionArgument(call, actualType, expectedType, context)
          : undefined;
      return coerceExpression(projected ?? call, expectedType);
    }
    case 'cast':
      if (expression.type.kind === 'named' && context.callbackTypeParameters.has(expression.type.name)) {
        const value = unwrapCasts(expression.expression);
        if (value.kind === 'identifier' && value.name === 'nullSignalEmit') {
          return `${expression.type.name}::flight_noop()`;
        }
        if (value.kind === 'function') return emitExpression(value, context, expression.type);
      }
      {
        const value = unwrapCasts(expression.expression);
        const actualType = inferIrExpressionType(value, context);
        const actual = resolveSemanticType(actualType, context);
        const target = resolveSemanticType(expression.type, context);
        if (
          actualType?.kind === 'nullable' &&
          target &&
          semanticTypesEqual(actualType.inner, expression.type, context)
        ) {
          return `${emitExpression(value, context, actualType)}.unwrap()`;
        }
        const unionSourceType = actualType?.kind === 'nullable' ? actualType.inner : actualType;
        const unionSource = resolveSemanticType(unionSourceType, context) ?? unionSourceType;
        if (unionSource?.kind === 'union' && !typeContainsAnyDynamic(expression.type)) {
          const variantIndex = unionSource.variants.findIndex((variant) =>
            semanticTypesEqual(variant, expression.type, context),
          );
          if (variantIndex >= 0) {
            const source = emitExpression(value, context, unionSourceType);
            const unionName =
              unionSourceType?.kind === 'named' ? emitNamedUnionConstructor(unionSourceType, context) : undefined;
            return unwrapUnionValue(source, unionSource.variants, variantIndex, unionName);
          }
        }
        const entitySourceType = actualType?.kind === 'nullable' ? actualType.inner : actualType;
        if (
          expression.type.kind === 'named' &&
          context.entityTypeParameters.has(expression.type.name) &&
          entitySourceType &&
          isNativeEntityType(entitySourceType, context)
        ) {
          const source = emitExpression(value, context, entitySourceType);
          return `${entityTraitTypePath(context)}::__flight_downcast::<${expression.type.name}>(&${parenthesize(source)}).expect("TypeScript entity cast lost its concrete Rust snapshot")`;
        }
        if (actual?.kind === 'dynamic' && actual.portable && target) {
          const portable = emitExpression(value, context, actualType);
          const valuePath = dynamicValuePath(actual);
          if (target.kind === 'named' && target.name === 'RustMap') {
            return `match ${portable} { ${valuePath}::Record(entries) => entries, _ => panic!("TypeScript Record cast received a non-record portable value") }`;
          }
          if (target.kind === 'primitive') {
            const variant =
              target.name === 'String'
                ? 'String'
                : target.name === 'Bool'
                  ? 'Bool'
                  : target.name === 'Float' || target.name === 'Int'
                    ? 'Number'
                    : undefined;
            if (variant) {
              return `match ${portable} { ${valuePath}::${variant}(value) => value, _ => panic!("TypeScript ${target.name} cast received an incompatible portable value") }`;
            }
          }
          const portableVector = emitPortableNumericVectorCast(portable, valuePath, target);
          if (portableVector) return portableVector;
        }
        if (actualType && actual?.kind === 'anonymous' && target?.kind === 'anonymous') {
          const projected = emitStructuralProjectionArgument(
            emitExpression(value, context, actualType),
            actualType,
            expression.type,
            context,
          );
          if (projected) return projected;
        }
      }
      if (
        unwrapCasts(expression.expression).kind !== 'call' &&
        (
          resolveSemanticType(inferIrExpressionType(expression.expression, context), context) ??
          inferIrExpressionType(expression.expression, context)
        )?.kind === 'dynamic' &&
        isDynamicHostCastTarget(expression.type, context)
      ) {
        return emitHostValueExpression(expression.type, '"host.cast"', context);
      }
      return emitExpression(
        expression.expression,
        context,
        expression.type.kind === 'dynamic' ? (expectedType ?? expression.type) : expression.type,
      );
    case 'conditional': {
      const constant = evaluateStaticBoolean(expression.condition, context);
      if (constant !== undefined) {
        return emitExpression(constant ? expression.whenTrue : expression.whenFalse, context, expectedType);
      }
      const narrowed = narrowTypeofContexts(expression.condition, context);
      const whenTrueType = inferIrExpressionType(expression.whenTrue, narrowed.whenTrue);
      const whenFalseType = inferIrExpressionType(expression.whenFalse, narrowed.whenFalse);
      const contextualType =
        expectedType ??
        inferIrExpressionType(expression, context) ??
        (whenTrueType && whenFalseType && typeKey(whenTrueType) === typeKey(whenFalseType) ? whenTrueType : undefined);
      return `if ${emitCondition(expression.condition, context)} { ${emitExpression(
        expression.whenTrue,
        narrowed.whenTrue,
        contextualType,
      )} } else { ${emitExpression(expression.whenFalse, narrowed.whenFalse, contextualType)} }`;
    }
    case 'element':
      if (isErasedEntityRuntimeAccess(expression) && !isNativeEntityRuntimeAccess(expression, context)) {
        rejectEntityRuntimeStorage();
      }
      return coerceExpression(emitElementRead(expression, context, expectedType), expectedType);
    case 'function':
      return emitClosure(expression, context, expectedType);
    case 'hostConstruct':
      return emitHostConstruct(expression, context);
    case 'identifier': {
      const resolvedExpected = resolveSemanticType(expectedType, context) ?? expectedType;
      const declaredType = context.symbolTypes.get(expression.name);
      if (declaredType?.kind === 'named' && declaredType.name === 'FlightNever') {
        return 'panic!("TypeScript never value was reached")';
      }
      const functionDeclaration = context.functions.get(expression.name);
      if (resolvedExpected?.kind === 'function' && functionDeclaration) {
        const parameters = resolvedExpected.parameters.map(
          (type, index): IrParameter => ({
            name: `__flight_argument_${String(index)}`,
            optional: false,
            rest: false,
            type,
          }),
        );
        return emitClosure(
          {
            body: [],
            execution: { kind: 'sync' },
            expression: {
              arguments: parameters.map((parameter) => ({
                kind: 'identifier',
                name: parameter.name,
              })),
              callee: expression,
              kind: 'call',
              typeArguments: [],
            },
            kind: 'function',
            parameters,
            returns: resolvedExpected.returns,
          },
          context,
          expectedType,
        );
      }
      if (expression.name.toLowerCase() === 'undefined' && expectedType?.kind === 'nullable') {
        return 'None';
      }
      if (
        expression.name.toLowerCase() === 'undefined' &&
        (resolveSemanticType(expectedType, context) ?? expectedType)?.kind === 'dynamic'
      ) {
        return `${dynamicValuePath(resolveSemanticType(expectedType, context) ?? expectedType)}::Undefined`;
      }
      const actualType = context.symbolTypes.get(expression.name);
      const emitted = emitIdentifier(expression.name, context);
      const flowNarrowing = context.unionNarrowings.get(expression.name);
      if (flowNarrowing) {
        const owned = context.borrowedNames.has(expression.name)
          ? `${parenthesize(`*${emitted}`)}.clone()`
          : `${parenthesize(emitted)}.clone()`;
        return coerceExpression(
          unwrapUnionValue(owned, flowNarrowing.variants, flowNarrowing.index, flowNarrowing.unionName),
          expectedType,
        );
      }
      if (actualType?.kind === 'union' && expectedType && expectedType.kind !== 'union') {
        const variantIndex = actualType.variants.findIndex((variant) => typeKey(variant) === typeKey(expectedType));
        if (variantIndex >= 0) {
          const owned = context.borrowedNames.has(expression.name)
            ? `${parenthesize(`*${emitted}`)}.clone()`
            : `${parenthesize(emitted)}.clone()`;
          return unwrapUnionValue(owned, actualType.variants, variantIndex);
        }
      }
      const value =
        context.constantNames.has(expression.name) &&
        actualType &&
        isReferenceLike(actualType, context) &&
        !context.atomicBoolNames.has(expression.name) &&
        !context.mutexCollectionNames.has(expression.name) &&
        !context.mutexValueNames.has(expression.name)
          ? `${parenthesize(`*${emitted}`)}.clone()`
          : emitted;
      const borrowedOwnedValue =
        actualType &&
        expectedType &&
        context.borrowedNames.has(expression.name) &&
        typeKey(actualType) === typeKey(expectedType)
          ? `${parenthesize(`*${emitted}`)}.clone()`
          : value;
      const resolvedActual = resolveSemanticType(actualType, context) ?? actualType;
      const ownedValue =
        actualType &&
        !context.borrowedNames.has(expression.name) &&
        !context.rawClosureNames.has(expression.name) &&
        ((!isCopyType(actualType, context) &&
          ((expectedType && typeKey(actualType) === typeKey(expectedType)) ||
            context.preservedNames.has(expression.name))) ||
          context.sharedCaptureNames.has(expression.name) ||
          resolvedActual?.kind === 'function' ||
          isSharedHandleType(actualType, context) ||
          (actualType.kind === 'named' && actualType.name === 'Signal') ||
          (actualType.kind === 'named' && actualType.name === 'FlightCallbackArgs') ||
          (actualType.kind === 'named' && context.callbackTypeParameters.has(actualType.name)))
          ? `${parenthesize(borrowedOwnedValue)}.clone()`
          : context.mutexValueNames.has(expression.name) || context.mutexCollectionNames.has(expression.name)
            ? `${parenthesize(borrowedOwnedValue)}.clone()`
            : borrowedOwnedValue;
      const narrowed =
        actualType?.kind === 'nullable' && expectedType && expectedType.kind !== 'nullable'
          ? `${parenthesize(ownedValue)}.clone().unwrap()`
          : ownedValue;
      if (
        context.constantNames.has(expression.name) &&
        resolvedExpected?.kind === 'primitive' &&
        resolvedExpected.name === 'String' &&
        (!actualType || isPlainStringType(actualType, context) || isStringRepresentedType(actualType, context))
      ) {
        return `${parenthesize(narrowed)}.to_owned()`;
      }
      return coerceExpression(narrowed, expectedType);
    }
    case 'literal':
      return emitLiteral(expression.value, expectedType, context);
    case 'new':
      return emitNew(expression, context, expectedType);
    case 'object':
      return emitObject(expression, context, expectedType);
    case 'property': {
      const actualType = inferIrExpressionType(expression, context);
      const value = emitProperty(expression, context, expectedType);
      const narrowed =
        actualType?.kind === 'nullable' && expectedType && expectedType.kind !== 'nullable'
          ? `${parenthesize(value)}.unwrap()`
          : value;
      return coerceExpression(narrowed, expectedType);
    }
    case 'regexp':
      return emitRegexp(expression);
    case 'spread':
      throw new RustEmissionError('spread Rust lowering is not implemented');
    case 'template':
      return emitTemplate(expression, context);
    case 'taskAll': {
      const expectedTask = expectedType?.kind === 'task' ? expectedType : undefined;
      const output = expectedTask?.output ?? expression.output;
      if (typeContainsDynamic(output)) {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: taskAll output type is not recovered`,
        );
      }
      const resolvedOutput = resolveSemanticType(output, context) ?? output;
      if (resolvedOutput.kind !== 'array') {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: taskAll currently requires a homogeneous array output`,
        );
      }
      const collection = resolveSemanticType(inferIrExpressionType(expression.tasks, context), context);
      const inputTask = collection?.kind === 'array' ? promiseType(collection.element, context) : undefined;
      const resolvedInput = inputTask
        ? (resolveSemanticType(inputTask.output, context) ?? inputTask.output)
        : undefined;
      const resolvedElement = resolveSemanticType(resolvedOutput.element, context) ?? resolvedOutput.element;
      const literalInputs =
        expression.tasks.kind === 'array'
          ? expression.tasks.elements.every((task) =>
              Boolean(promiseType(inferIrExpressionType(task, context), context)),
            )
          : true;
      if (!inputTask || !resolvedInput || typeKey(resolvedInput) !== typeKey(resolvedElement) || !literalInputs) {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: taskAll currently requires homogeneous task inputs matching its array output`,
        );
      }
      const tasksType: IrType = {
        element: { kind: 'task', output: resolvedOutput.element },
        kind: 'array',
      };
      return `crate::FlightTask::all(${emitExpression(expression.tasks, context, tasksType)}, ${emitTaskOrigin(expression.origin)})`;
    }
    case 'taskReady': {
      const expectedTask = expectedType?.kind === 'task' ? expectedType : undefined;
      const output = expectedTask?.output ?? expression.output;
      if (typeContainsDynamic(output)) {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: taskReady output type is not recovered`,
        );
      }
      const value = expression.value ? emitExpression(expression.value, context, output) : '()';
      return `crate::FlightTask::ready(${value}, ${emitTaskOrigin(expression.origin)})`;
    }
    case 'taskReject': {
      const expectedTask = expectedType?.kind === 'task' ? expectedType : undefined;
      const output = expectedTask?.output ?? expression.output;
      if (typeContainsDynamic(output)) {
        throw new RustEmissionError(
          `${expression.origin.source}:${String(expression.origin.line)}:${String(expression.origin.column)}: taskReject output type is not recovered`,
        );
      }
      return `crate::FlightTask::<${emitType(output, context)}>::reject(${emitTaskRejection(expression.rejection, context)}, ${emitTaskOrigin(expression.origin)})`;
    }
    case 'unary':
      return coerceExpression(emitUnary(expression, context), expectedType);
  }
}

function emitHostValueExpression(type: IrType, operation: string, context: EmitContext): string {
  const resolved = resolveSemanticType(type, context) ?? type;
  if (resolved.kind === 'task') {
    return `crate::host_task::<${emitType(resolved.output, context)}>(${operation})`;
  }
  return `crate::host_value::<${emitType(type, context)}>(${operation})`;
}

function emitCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  if (expression.optional) return emitOptionalCall(expression, context);
  const promiseCall = emitPromiseCall(expression, context);
  if (promiseCall) return promiseCall;
  const portableGlobal =
    expression.callee.kind === 'identifier' ? expression.callee.name : runtimeGlobalType(expression.callee);
  if (portableGlobal) {
    const value = expression.arguments[0];
    if (portableGlobal === 'isNaN') {
      if (!value) throw new RustEmissionError('isNaN requires a value');
      return `${parenthesize(emitExpression(value, context, primitive('Float')))}.is_nan()`;
    }
    if (portableGlobal === 'parseInt') {
      if (!value) throw new RustEmissionError('parseInt requires a string value');
      const radix = expression.arguments[1]
        ? emitExpression(expression.arguments[1], context, primitive('Float'))
        : '10.0_f64';
      return `{ let __flight_value = ${emitExpression(value, context, primitive('String'))}; let __flight_radix = ${parenthesize(radix)} as u32; i64::from_str_radix(__flight_value.trim(), __flight_radix).map_or(f64::NAN, |value| value as f64) }`;
    }
    if (portableGlobal === 'parseFloat') {
      if (!value) throw new RustEmissionError('parseFloat requires a string value');
      return `__flight_parse_float(&${parenthesize(emitExpression(value, context, primitive('String')))})`;
    }
    if (portableGlobal === 'encodeURIComponent') {
      if (!value) throw new RustEmissionError('encodeURIComponent requires a string value');
      return `__flight_encode_uri_component(&${parenthesize(emitExpression(value, context, primitive('String')))})`;
    }
    if (portableGlobal === 'decodeURIComponent') {
      if (!value) throw new RustEmissionError('decodeURIComponent requires a string value');
      return `__flight_decode_uri_component(&${parenthesize(emitExpression(value, context, primitive('String')))})`;
    }
    if (portableGlobal === 'String') {
      if (!value) return 'String::new()';
      const valueType = inferIrExpressionType(value, context) ?? { kind: 'dynamic', portable: true };
      const portableValue = emitPortableValueExpression(value, context, valueType);
      return `{ let __flight_value = ${portableValue}; crate::flight_value_to_string(&__flight_value) }`;
    }
    if (portableGlobal === 'Number') {
      if (!value) return '0.0_f64';
      const valueType = resolveSemanticType(inferIrExpressionType(value, context), context);
      if (valueType?.kind === 'primitive' && (valueType.name === 'Float' || valueType.name === 'Int')) {
        return emitExpression(value, context, primitive('Float'));
      }
      if (valueType?.kind === 'primitive' && valueType.name === 'Bool') {
        return `if ${emitExpression(value, context, primitive('Bool'))} { 1.0_f64 } else { 0.0_f64 }`;
      }
      return `__flight_number_from_string(&${parenthesize(emitExpression(value, context, primitive('String')))})`;
    }
  }
  const knownHostReturnType = inferKnownHostCallReturnType(expression, context);
  if (knownHostReturnType) {
    return emitHostValueExpression(expectedType ?? knownHostReturnType, '"host.call"', context);
  }
  if (isSymbolConstruction(expression)) {
    if (expression.callee.kind === 'property' && expression.callee.name === 'for' && expression.arguments[0]) {
      return `crate::FlightSymbol::for_name(&${parenthesize(
        emitExpression(expression.arguments[0], context, primitive('String')),
      )})`;
    }
    return 'crate::FlightSymbol::new()';
  }
  if (isArrayFillConstruction(expression)) {
    const length = expression.callee.object.arguments[0];
    const value = expression.arguments[0];
    if (!length || !value) throw new RustEmissionError('new Array(length).fill(value) requires both arguments');
    return `vec![${emitExpression(value, context)}; ${parenthesize(emitExpression(length, context))} as usize]`;
  }
  if (expression.callee.kind === 'identifier') {
    if (expression.callee.name === 'clearTimeout') {
      const timer = expression.arguments[0];
      if (!timer) throw new RustEmissionError('clearTimeout requires a timer handle');
      const timerType = inferIrExpressionType(timer, context);
      const value = emitExpression(timer, context, timerType);
      return timerType?.kind === 'nullable'
        ? `if let Some(__flight_timer) = ${parenthesize(value)}.clone() { crate::clear_timeout(__flight_timer); }`
        : `crate::clear_timeout(${value})`;
    }
    if (expression.callee.name === 'clearInterval') {
      const timer = expression.arguments[0];
      if (!timer) throw new RustEmissionError('clearInterval requires a timer handle');
      const timerType = inferIrExpressionType(timer, context);
      const value = emitExpression(timer, context, timerType);
      return timerType?.kind === 'nullable'
        ? `if let Some(__flight_timer) = ${parenthesize(value)}.clone() { crate::clear_interval(__flight_timer); }`
        : `crate::clear_interval(${value})`;
    }
    if (expression.callee.name === 'setTimeout') {
      const callback = expression.arguments[0];
      const delay = expression.arguments[1];
      if (!callback || !delay) throw new RustEmissionError('setTimeout requires a callback and delay');
      const callbackType: IrType = {
        kind: 'function',
        parameters: [],
        returns: primitive('Void'),
      };
      const emittedCallback =
        callback.kind === 'function'
          ? emitClosure(callback, context, callbackType, false, true)
          : `{ let __flight_callback = ${emitExpression(
              callback,
              context,
              callbackType,
            )}; move || __flight_callback.lock().unwrap()() }`;
      return `crate::set_timeout(${emittedCallback}, ${emitExpression(delay, context, primitive('Float'))})`;
    }
    if (expression.callee.name === 'setInterval') {
      const callback = expression.arguments[0];
      const delay = expression.arguments[1];
      if (!callback || !delay) throw new RustEmissionError('setInterval requires a callback and delay');
      const callbackType: IrType = {
        kind: 'function',
        parameters: [],
        returns: primitive('Void'),
      };
      const emittedCallback =
        callback.kind === 'function'
          ? emitClosure(callback, context, callbackType, false, true)
          : `{ let __flight_callback = ${emitExpression(
              callback,
              context,
              callbackType,
            )}; move || __flight_callback.lock().unwrap()() }`;
      return `crate::set_interval(${emittedCallback}, ${emitExpression(delay, context, primitive('Float'))})`;
    }
    const localType = context.symbolTypes.get(expression.callee.name);
    if (localType?.kind === 'nullable') {
      const callbackType = resolveSemanticType(localType.inner, context);
      if (callbackType?.kind === 'function') {
        const arguments_ = expression.arguments.map((argument, index) =>
          argument.kind === 'spread'
            ? emitExpression(argument.expression, context, callbackType.parameters[index])
            : emitExpression(argument, context, callbackType.parameters[index]),
        );
        return `${emitPlaceExpression(expression.callee, context)}.as_ref().unwrap().lock().unwrap()(${arguments_.join(', ')})`;
      }
    }
    if (context.mutexValueNames.has(expression.callee.name)) {
      const localType = inferIrExpressionType(expression.callee, context);
      const resolvedLocal = resolveSemanticType(localType, context) ?? localType;
      const callbackType =
        resolvedLocal?.kind === 'nullable' ? resolveSemanticType(resolvedLocal.inner, context) : resolvedLocal;
      const arguments_ = expression.arguments.map((argument, index) => {
        const value = argument.kind === 'spread' ? argument.expression : argument;
        return emitFunctionCallArgument(
          value,
          callbackType?.kind === 'function' ? callbackType.parameters[index] : inferIrExpressionType(value, context),
          context,
        );
      });
      const callback = emitPlaceExpression(expression.callee, context);
      const value = context.nonNullableNames.has(expression.callee.name)
        ? `${parenthesize(callback)}.clone()`
        : `${parenthesize(callback)}.clone().unwrap()`;
      return `{ let __flight_callback = ${value}; __flight_callback.lock().unwrap()(${arguments_.join(', ')}) }`;
    }
    const inline = context.inlineFunctions.get(expression.callee.name);
    if (inline) return emitInlineFunctionCall(expression, inline, context);
    const declaration = context.functions.get(expression.callee.name);
    if (declaration) {
      if (
        context.localFunctionNames.has(declaration.name) &&
        requiresMutableStructuralInlining(expression, declaration, context)
      ) {
        return emitStructuralFunctionInlineCall(expression, declaration, context, true);
      }
      if (
        !context.localFunctionNames.has(declaration.name) &&
        ((!declaration.exported && !containsIdentifier(declaration.body, declaration.name)) ||
          (declaration.parameters.some((parameter) => typeContainsAnonymousRecord(parameter.type, context)) &&
            isStructuralInlineHelper(declaration)))
      ) {
        return emitStructuralFunctionInlineCall(expression, declaration, context);
      }
      return emitKnownFunctionCall(expression, declaration, context, expectedType);
    }
  }
  if (
    expression.callee.kind === 'property' &&
    expression.callee.object.kind === 'identifier' &&
    expression.callee.object.name === 'JSON' &&
    expression.callee.name === 'stringify'
  ) {
    const value = expression.arguments[0];
    if (!value) throw new RustEmissionError('JSON.stringify requires a value');
    if (expression.arguments.length > 1) {
      throw new RustEmissionError('JSON.stringify replacer and spacing arguments are not implemented');
    }
    return emitJsonStringify(value, context);
  }
  const runtimeGlobal = runtimeGlobalType(expression);
  if (runtimeGlobal) return 'crate::OpaqueHostValue::Object';
  if (expression.callee.kind === 'property') {
    const owner =
      expression.callee.object.kind === 'identifier'
        ? expression.callee.object.name
        : runtimeGlobalType(expression.callee.object);
    const method = expression.callee.name;
    if (owner === 'Array' && method === 'isArray') {
      const value = expression.arguments[0];
      if (!value) throw new RustEmissionError('Array.isArray requires a value');
      const type = inferIrExpressionType(value, context);
      const resolved = resolveSemanticType(type?.kind === 'nullable' ? type.inner : type, context);
      const represented =
        resolved?.kind === 'array' || (resolved?.kind === 'named' && Boolean(typedArrayType(resolved.name)));
      return type?.kind === 'nullable'
        ? represented
          ? `${parenthesize(emitExpression(value, context))}.is_some()`
          : 'false'
        : String(represented);
    }
    if (owner === 'Array' && method === 'from') {
      const result =
        (resolveSemanticType(expectedType, context) ?? expectedType)?.kind === 'array'
          ? expectedType!
          : ({ element: { kind: 'dynamic' }, kind: 'array' } as const);
      return emitHostValueExpression(result, '"host.Array.from"', context);
    }
    if (owner === 'Math') {
      return emitMathCall(
        method,
        expression.arguments.map((argument) => emitExpression(argument, context)),
      );
    }
    if (owner === 'Number' && method === 'isFinite' && expression.arguments[0]) {
      return `${parenthesize(emitExpression(expression.arguments[0], context))}.is_finite()`;
    }
    if (owner === 'Number' && method === 'isInteger' && expression.arguments[0]) {
      const value = emitExpression(expression.arguments[0], context);
      return `${parenthesize(value)}.is_finite() && ${parenthesize(value)}.fract() == 0.0_f64`;
    }
    if (owner === 'Number' && method === 'parseFloat') {
      const value = expression.arguments[0];
      if (!value) throw new RustEmissionError('Number.parseFloat requires a string value');
      return `__flight_parse_float(&${parenthesize(emitExpression(value, context, primitive('String')))})`;
    }
    if (owner === 'String' && method === 'fromCodePoint') {
      const values = expression.arguments.map((argument) => emitExpression(argument, context, primitive('Float')));
      if (values.length === 0) return 'String::new()';
      if (values.length === 1) return `__flight_string_from_code_point(${values[0]})`;
      return `{ let mut __flight_string = String::new(); for __flight_code_point in [${values.join(', ')}] { __flight_string.push_str(&__flight_string_from_code_point(__flight_code_point)); } __flight_string }`;
    }
    if (owner === 'Object' && method === 'is') {
      const left = expression.arguments[0];
      const right = expression.arguments[1];
      if (!left || !right) throw new RustEmissionError('Object.is requires two values');
      const leftType = resolveSemanticType(inferIrExpressionType(left, context), context);
      const rightType = resolveSemanticType(inferIrExpressionType(right, context), context);
      const numeric = (type: IrType | undefined): boolean =>
        (type?.kind === 'primitive' && (type.name === 'Float' || type.name === 'Int')) ||
        isPortableNumericStorageType(type);
      if (numeric(leftType) && numeric(rightType)) {
        const emittedLeft = emitExpression(left, context, primitive('Float'));
        const emittedRight = emitExpression(right, context, primitive('Float'));
        return `{ let __flight_left = ${emittedLeft}; let __flight_right = ${emittedRight}; __flight_left.to_bits() == __flight_right.to_bits() || (__flight_left.is_nan() && __flight_right.is_nan()) }`;
      }
      if (
        leftType?.kind === 'primitive' &&
        rightType?.kind === 'primitive' &&
        leftType.name === rightType.name &&
        (leftType.name === 'Bool' || leftType.name === 'String')
      ) {
        return `${parenthesize(emitExpression(left, context, leftType))} == ${parenthesize(
          emitExpression(right, context, rightType),
        )}`;
      }
      return emitHostValueExpression(primitive('Bool'), '"host.Object.is"', context);
    }
    if (owner === 'Object' && method === 'keys') {
      const value = expression.arguments[0];
      if (!value) throw new RustEmissionError('Object.keys requires a value');
      const valueType = inferIrExpressionType(value, context);
      const collectionType = valueType?.kind === 'nullable' ? valueType.inner : valueType;
      if (collectionType?.kind === 'named' && collectionType.name === 'RustMap') {
        const collection =
          valueType?.kind === 'nullable'
            ? `${emitPlaceExpression(value, assignmentPlaceContext(value, context))}.as_ref().unwrap()`
            : emitCollectionPlace(value, context);
        return `${collection}.iter().map(|(entry_key, _)| entry_key.clone()).collect::<Vec<_>>()`;
      }
      return emitHostValueExpression({ element: primitive('String'), kind: 'array' }, '"host.Object.keys"', context);
    }
    if (owner === 'Object' && method === 'entries') {
      const value = expression.arguments[0];
      if (!value) throw new RustEmissionError('Object.entries requires a value');
      const valueType = inferIrExpressionType(value, context);
      const collectionType = valueType?.kind === 'nullable' ? valueType.inner : valueType;
      if (collectionType?.kind === 'named' && collectionType.name === 'RustMap') {
        return `${parenthesize(emitExpression(value, context, collectionType))}.clone()`;
      }
      const tuple: IrType = {
        arguments: [primitive('String'), { kind: 'dynamic' }],
        kind: 'named',
        name: 'RustTuple2',
      };
      return emitHostValueExpression({ element: tuple, kind: 'array' }, '"host.Object.entries"', context);
    }
    if (owner === 'Date' && method === 'now') return 'crate::flight_now_millis()';
    if (owner === '_Runtime' && method === 'typeofGlobal') return '"undefined"';
  }
  if (
    expression.callee.kind === 'property' &&
    expression.callee.name === 'now' &&
    runtimeGlobalType(expression.callee.object) === 'Date'
  ) {
    return 'crate::flight_now_millis()';
  }
  if (expression.callee.kind === 'property') {
    const ownerType = inferIrExpressionType(expression.callee.object, context);
    const collectionType = ownerType?.kind === 'nullable' ? ownerType.inner : ownerType;
    const method = expression.callee.name;
    const owner =
      ownerType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.callee.object, context)}.as_ref().unwrap()`
        : emitExpression(expression.callee.object, context);
    const nullishOwner =
      expression.callee.object.kind === 'assignment' && expression.callee.object.operator === '??='
        ? expression.callee.object
        : undefined;
    const ownerPlace = nullishOwner
      ? `${emitPlaceExpression(nullishOwner.left, assignmentPlaceContext(nullishOwner.left, context))}.as_mut().unwrap()`
      : ownerType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.callee.object, context)}.as_mut().unwrap()`
        : emitCollectionPlace(expression.callee.object, context);
    const ownerRead =
      ownerType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.callee.object, context)}.as_ref().unwrap()`
        : emitCollectionPlace(expression.callee.object, context);
    if (collectionType?.kind === 'dynamic' || isNativeHostHandleType(collectionType)) {
      const result = expectedType ?? primitive('Void');
      return emitHostValueExpression(result, emitRustStringLiteral(`host.${method}`), context);
    }
    if (collectionType?.kind === 'named' && collectionType.name === 'RustMap') {
      const keyType = collectionType.arguments[0];
      const valueType = collectionType.arguments[1];
      const keyExpression = expression.arguments[0];
      if (method === 'clear') return `${ownerPlace}.clear()`;
      if (method === 'keys') {
        return `${ownerRead}.iter().map(|(key, _)| key.clone()).collect::<Vec<_>>()`;
      }
      if (method === 'values') {
        return `${ownerRead}.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>()`;
      }
      if (!keyExpression) throw new RustEmissionError(`Map.${method} requires a key argument`);
      const key = emitExpression(keyExpression, context, keyType);
      if (method === 'get') {
        let lookup: string;
        if (
          ownerType?.kind === 'nullable' &&
          expression.callee.object.kind === 'property' &&
          expression.callee.object.optional
        ) {
          const optionalOwner = emitExpression(expression.callee.object, context, ownerType);
          lookup = `${parenthesize(optionalOwner)}.and_then(|entries| entries.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone()))`;
        } else if (expression.callee.optional && ownerType?.kind === 'nullable') {
          const optionalOwner = emitPlaceExpression(expression.callee.object, context);
          lookup = `${optionalOwner}.as_ref().and_then(|entries| entries.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone()))`;
        } else {
          lookup = `${ownerRead}.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone())`;
        }
        return expectedType &&
          expectedType.kind !== 'nullable' &&
          valueType &&
          semanticTypesEqual(valueType, expectedType, context)
          ? `${parenthesize(lookup)}.expect("TypeScript Map.get returned undefined")`
          : lookup;
      }
      if (method === 'has') {
        return `${ownerRead}.iter().any(|(entry_key, _)| entry_key == &${key})`;
      }
      if (method === 'delete') {
        return `{ let __flight_key = ${key}; if let Some(__flight_index) = ${ownerPlace}.iter().position(|(key, _)| key == &__flight_key) { ${ownerPlace}.remove(__flight_index); true } else { false } }`;
      }
      if (method === 'set') {
        const valueExpression = expression.arguments[1];
        if (!valueExpression) throw new RustEmissionError('Map.set requires a value argument');
        const value = emitExpression(valueExpression, context, valueType);
        const mutation = `let __flight_key = ${key}; let __flight_value = ${value}; if let Some((_, value)) = ${ownerPlace}.iter_mut().find(|(key, _)| key == &__flight_key) { *value = __flight_value; } else { ${ownerPlace}.push((__flight_key, __flight_value)); }`;
        const initialize = nullishOwner ? emitNullishAssignment(nullishOwner, context, false) : undefined;
        return `{ ${initialize ? `${initialize}; ` : ''}${mutation} }`;
      }
    }
    if (collectionType?.kind === 'named' && collectionType.name === 'RustSet') {
      const valueType = collectionType.arguments[0];
      const valueExpression = expression.arguments[0];
      if (method === 'clear') return `${ownerPlace}.clear()`;
      if (method === 'keys' || method === 'values') return `${ownerRead}.clone()`;
      if (!valueExpression) throw new RustEmissionError(`Set.${method} requires a value argument`);
      const value = emitExpression(valueExpression, context, valueType);
      if (method === 'has') return `${ownerRead}.iter().any(|item| item == &${value})`;
      if (method === 'add') {
        return `{ let __flight_value = ${value}; if !${ownerPlace}.contains(&__flight_value) { ${ownerPlace}.push(__flight_value); } }`;
      }
      if (method === 'delete') {
        return `{ let __flight_value = ${value}; if let Some(__flight_index) = ${ownerPlace}.iter().position(|item| item == &__flight_value) { ${ownerPlace}.remove(__flight_index); true } else { false } }`;
      }
    }
    if (collectionType?.kind === 'named' && collectionType.name === 'FlightRegex') {
      const argument = expression.arguments[0];
      if (method === 'test') {
        if (!argument) throw new RustEmissionError('RegExp.test requires a string argument');
        return `${parenthesize(owner)}.is_match(&${parenthesize(emitExpression(argument, context))})`;
      }
      if (method === 'exec') {
        if (!argument) throw new RustEmissionError('RegExp.exec requires a string argument');
        return emitRegexCaptures(owner, emitExpression(argument, context, primitive('String')));
      }
    }
    const callableType =
      collectionType?.kind === 'function'
        ? collectionType
        : ownerType?.kind === 'nullable'
          ? resolveSemanticType(ownerType.inner, context)
          : undefined;
    if (method === 'call' && callableType?.kind === 'function') {
      const arguments_ = expression.arguments
        .slice(1)
        .map((argument, index) => emitExpression(argument, context, callableType.parameters[index]));
      const callback =
        ownerType?.kind === 'nullable'
          ? `${emitPlaceExpression(expression.callee.object, context)}.as_ref().unwrap()`
          : owner;
      return `${parenthesize(callback)}.lock().unwrap()(${arguments_.join(', ')})`;
    }
    if (collectionType?.kind === 'primitive' && collectionType.name === 'String') {
      const argument = expression.arguments[0];
      if (method === 'charCodeAt') {
        if (!argument) throw new RustEmissionError('String.charCodeAt requires an index argument');
        const view =
          expression.callee.object.kind === 'identifier'
            ? context.utf16ViewNames.get(expression.callee.object.name)
            : undefined;
        return emitUtf16CharCodeAt(
          view ? `&${view}` : `&${parenthesize(owner)}.encode_utf16().collect::<Vec<u16>>()`,
          emitExpression(argument, context, primitive('Float')),
        );
      }
      if (method === 'codePointAt') {
        if (!argument) throw new RustEmissionError('String.codePointAt requires an index argument');
        const view =
          expression.callee.object.kind === 'identifier'
            ? context.utf16ViewNames.get(expression.callee.object.name)
            : undefined;
        return emitUtf16CodePointAt(
          view ? `&${view}` : `&${parenthesize(owner)}.encode_utf16().collect::<Vec<u16>>()`,
          emitExpression(argument, context, primitive('Float')),
        );
      }
      if (argument?.kind === 'regexp') {
        const regex = emitRegexp(argument);
        if (method === 'match') return emitRegexCaptures(regex, owner);
        if (method === 'split') {
          return `${parenthesize(regex)}.split(&${parenthesize(owner)}).map(|part| part.to_owned()).collect::<Vec<_>>()`;
        }
        if (method === 'search') {
          return `${parenthesize(regex)}.find(&${parenthesize(owner)}).map_or(-1.0_f64, |matched| matched.start() as f64)`;
        }
        if (method === 'replace') {
          const replacement = expression.arguments[1];
          if (!replacement) throw new RustEmissionError('String.replace requires a replacement');
          const replaceMethod = argument.flags.includes('g') ? 'replace_all' : 'replace';
          if (replacement.kind === 'function') {
            const stringType = primitive('String');
            const optionalStringType: IrType = { inner: stringType, kind: 'nullable' };
            const callbackType: IrType = {
              kind: 'function',
              parameters: replacement.parameters.map((_, index) => (index === 0 ? stringType : optionalStringType)),
              returns: stringType,
            };
            const callbackArguments = replacement.parameters.map((parameter, index) => {
              const contextual = parameter.type.kind === 'dynamic' ? callbackType.parameters[index] : parameter.type;
              return index === 0 || contextual?.kind !== 'nullable'
                ? `captures.get(${String(index)}).map_or("", |matched| matched.as_str()).to_owned()`
                : `captures.get(${String(index)}).map(|matched| matched.as_str().to_owned())`;
            });
            return `{ let mut __flight_replace = ${emitClosure(replacement, context, callbackType, false)}; ${parenthesize(regex)}.${replaceMethod}(&${parenthesize(owner)}, |captures: &regex::Captures<'_>| __flight_replace(${callbackArguments.join(', ')})).into_owned() }`;
          }
          return `${parenthesize(regex)}.${replaceMethod}(&${parenthesize(owner)}, ${emitExpression(replacement, context, primitive('String'))}).into_owned()`;
        }
      }
      if (method === 'split' && argument) {
        return `${parenthesize(owner)}.split(${emitExpression(argument, context, primitive('String'))}.as_str()).map(|part| part.to_owned()).collect::<Vec<_>>()`;
      }
      if (method === 'startsWith' || method === 'endsWith' || method === 'includes') {
        if (!argument) throw new RustEmissionError(`String.${method} requires a search argument`);
        const rustMethod = method === 'startsWith' ? 'starts_with' : method === 'endsWith' ? 'ends_with' : 'contains';
        return `${parenthesize(owner)}.${rustMethod}(${parenthesize(emitExpression(argument, context, primitive('String')))}.as_str())`;
      }
      if (method === 'indexOf') {
        if (!argument) throw new RustEmissionError('String.indexOf requires a search argument');
        const position = expression.arguments[1]
          ? emitExpression(expression.arguments[1], context, primitive('Float'))
          : '0.0_f64';
        return `__flight_string_index_of(&${parenthesize(owner)}, &${parenthesize(emitExpression(argument, context, primitive('String')))}, ${position})`;
      }
      if (method === 'slice') {
        const start = expression.arguments[0] ?? ({ kind: 'literal', value: 0 } as const);
        const end = expression.arguments[1];
        const endType = end ? inferIrExpressionType(end, context) : undefined;
        const emittedEnd = !end
          ? 'None'
          : endType?.kind === 'nullable'
            ? emitExpression(end, context, endType)
            : `Some(${emitExpression(end, context, primitive('Float'))})`;
        return `__flight_string_slice(&${parenthesize(owner)}, ${emitExpression(start, context, primitive('Float'))}, ${
          emittedEnd
        })`;
      }
      if (method === 'toLowerCase') return `${parenthesize(owner)}.to_lowercase()`;
      if (method === 'toUpperCase') return `${parenthesize(owner)}.to_uppercase()`;
      if (method === 'trim') return `${parenthesize(owner)}.trim().to_owned()`;
      if (method === 'padStart') {
        const width = expression.arguments[0];
        if (!width) throw new RustEmissionError('String.padStart requires a target width');
        const pad = expression.arguments[1] ?? ({ kind: 'literal', value: ' ' } as const);
        return `__flight_pad_start(${owner}, ${emitExpression(width, context, primitive('Float'))}, ${emitExpression(pad, context, primitive('String'))})`;
      }
      if (method === 'repeat') {
        if (!argument) throw new RustEmissionError('String.repeat requires a count argument');
        return `__flight_string_repeat(&${parenthesize(owner)}, ${emitExpression(argument, context, primitive('Float'))})`;
      }
    }
    if (
      collectionType?.kind === 'primitive' &&
      (collectionType.name === 'Float' || collectionType.name === 'Int') &&
      method === 'toFixed'
    ) {
      const digits = expression.arguments[0] ?? ({ kind: 'literal', value: 0 } as const);
      return `__flight_number_to_fixed(${owner}, ${emitExpression(digits, context, primitive('Float'))})`;
    }
    if (
      collectionType?.kind === 'primitive' &&
      (collectionType.name === 'Float' || collectionType.name === 'Int') &&
      method === 'toString'
    ) {
      const radix = expression.arguments[0];
      return radix
        ? `__flight_number_to_string(${owner}, ${emitExpression(radix, context, primitive('Float'))})`
        : `${parenthesize(owner)}.to_string()`;
    }
    if (
      (collectionType?.kind === 'array' ||
        (collectionType?.kind === 'named' && Boolean(typedArrayType(collectionType.name)))) &&
      method === 'slice' &&
      expression.arguments.length === 0
    ) {
      return ownerType?.kind === 'nullable'
        ? `${parenthesize(owner)}.as_ref().unwrap().clone()`
        : `${parenthesize(owner)}.clone()`;
    }
    if (collectionType?.kind === 'array' && method === 'pop') {
      const popped = `${ownerPlace}.pop()`;
      return expectedType?.kind === 'nullable' ? popped : `${popped}.expect("TypeScript Array.pop returned undefined")`;
    }
    if (collectionType?.kind === 'array' && method === 'concat') {
      const values = emitExpression(expression.callee.object, context, collectionType);
      const operations = expression.arguments.map((argument, index) => {
        if (argument.kind === 'spread') {
          throw new RustEmissionError('Array.concat spread arguments are not implemented');
        }
        const argumentType = inferIrExpressionType(argument, context);
        const resolvedArgument = resolveSemanticType(argumentType, context) ?? argumentType;
        const emittedArgument = emitExpression(argument, context, argumentType);
        if (
          resolvedArgument?.kind === 'array' ||
          (resolvedArgument?.kind === 'named' && Boolean(typedArrayType(resolvedArgument.name)))
        ) {
          return `let __flight_concat_${String(index)} = ${emittedArgument}; __flight_values.extend(__flight_concat_${String(index)}.iter().cloned());`;
        }
        return `__flight_values.push(${emitExpression(argument, context, collectionType.element)});`;
      });
      return `{ let mut __flight_values = ${values}; ${operations.join(' ')} __flight_values }`;
    }
    if (collectionType?.kind === 'array' && method === 'copyWithin') {
      const target = expression.arguments[0];
      const start = expression.arguments[1];
      if (!target || !start) throw new RustEmissionError('Array.copyWithin requires target and start arguments');
      const end = expression.arguments[2]
        ? emitExpression(expression.arguments[2], context)
        : `${ownerPlace}.len() as f64`;
      return `{ let __flight_start = ${parenthesize(emitExpression(start, context))} as usize; let __flight_end = ${parenthesize(end)} as usize; let __flight_target = ${parenthesize(emitExpression(target, context))} as usize; ${ownerPlace}.copy_within(__flight_start..__flight_end, __flight_target); ${ownerPlace}.clone() }`;
    }
    if (collectionType?.kind === 'array' && method === 'splice') {
      const start = expression.arguments[0];
      if (!start) throw new RustEmissionError('Array.splice requires a start argument');
      const startValue = emitExpression(start, context);
      const count = expression.arguments[1]
        ? emitExpression(expression.arguments[1], context)
        : `((${ownerPlace}.len() as f64) - __flight_start)`;
      const inserted = expression.arguments
        .slice(2)
        .map((argument) => emitExpression(argument, context, collectionType.element));
      return `{ let __flight_start = ${parenthesize(startValue)}; let __flight_count = ${parenthesize(count)}; ${ownerPlace}.splice((__flight_start) as usize..(__flight_start + __flight_count) as usize, vec![${inserted.join(', ')}]).collect::<Vec<_>>() }`;
    }
    if (
      (collectionType?.kind === 'array' ||
        (collectionType?.kind === 'named' && Boolean(typedArrayType(collectionType.name)))) &&
      method === 'fill'
    ) {
      const value = expression.arguments[0];
      if (!value) throw new RustEmissionError('Array.fill requires a value');
      const element =
        collectionType.kind === 'array' ? collectionType.element : typedArrayElementType(collectionType.name);
      const emittedValue = emitExpression(value, context, element);
      if (isRustPlaceExpression(expression.callee.object)) {
        const ownerReference =
          expression.callee.object.kind === 'identifier' && context.borrowedNames.has(expression.callee.object.name)
            ? `&mut *${ownerPlace}`
            : `&mut ${ownerPlace}`;
        return `{ let __flight_value = ${emittedValue}; let __flight_collection = ${ownerReference}; __flight_collection.fill(__flight_value); __flight_collection.clone() }`;
      }
      return `{ let mut __flight_collection = ${owner}; let __flight_value = ${emittedValue}; __flight_collection.fill(__flight_value); __flight_collection }`;
    }
    if (collectionType?.kind === 'named' && typedArrayType(collectionType.name) && method === 'set') {
      const sourceExpression = expression.arguments[0];
      if (!sourceExpression) throw new RustEmissionError('TypedArray.set requires a source argument');
      const offsetExpression = expression.arguments[1];
      const target = typedArrayType(collectionType.name)!;
      const source = emitExpression(sourceExpression, context);
      const offset = offsetExpression ? emitExpression(offsetExpression, context) : '0.0_f64';
      return [
        '{',
        `let __flight_offset = ${parenthesize(offset)} as usize;`,
        `let __flight_values: Vec<${target.rust}> = ${parenthesize(source)}.iter().map(|value| (*value) as ${target.rust}).collect();`,
        `${ownerPlace}[__flight_offset..__flight_offset + __flight_values.len()].copy_from_slice(&__flight_values);`,
        '}',
      ].join(' ');
    }
    if (collectionType?.kind === 'array' && method === 'indexOf') {
      const argument = expression.arguments[0];
      if (!argument) throw new RustEmissionError('Array.indexOf requires an argument');
      const collection =
        ownerType?.kind === 'nullable' ? `${parenthesize(owner)}.as_ref().unwrap()` : parenthesize(ownerRead);
      const value = emitExpression(argument, context, collectionType.element);
      const equality =
        collectionType.element.kind === 'named' && context.callbackTypeParameters.has(collectionType.element.name)
          ? 'crate::FlightCallback::flight_same(item, &__flight_value)'
          : resolveSemanticType(collectionType.element, context)?.kind === 'function'
            ? 'std::sync::Arc::ptr_eq(item, &__flight_value)'
            : 'item == &__flight_value';
      return `{ let __flight_value = ${value}; ${collection}.iter().position(|item| ${equality}).map_or(-1.0_f64, |index| index as f64) }`;
    }
    if (collectionType?.kind === 'array' && method === 'includes') {
      const argument = expression.arguments[0];
      if (!argument) throw new RustEmissionError('Array.includes requires an argument');
      const collection =
        ownerType?.kind === 'nullable' ? `${parenthesize(owner)}.as_ref().unwrap()` : parenthesize(ownerRead);
      const value = emitExpression(argument, context, collectionType.element);
      const equality =
        collectionType.element.kind === 'named' && context.callbackTypeParameters.has(collectionType.element.name)
          ? 'crate::FlightCallback::flight_same(item, &__flight_value)'
          : resolveSemanticType(collectionType.element, context)?.kind === 'function'
            ? 'std::sync::Arc::ptr_eq(item, &__flight_value)'
            : 'item == &__flight_value';
      return `{ let __flight_value = ${value}; ${collection}.iter().any(|item| ${equality}) }`;
    }
    if (collectionType?.kind === 'array' && method === 'sort') {
      const callback = expression.arguments[0];
      const comparator =
        callback?.kind === 'function'
          ? emitClosure(
              callback,
              context,
              {
                kind: 'function',
                parameters: [collectionType.element, collectionType.element],
                returns: primitive('Float'),
              },
              false,
            )
          : undefined;
      const ordering = comparator
        ? `{ let __flight_order = ${parenthesize(comparator)}(left.clone(), right.clone()); __flight_order.partial_cmp(&0.0_f64).unwrap_or(std::cmp::Ordering::Equal) }`
        : 'left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)';
      return `{ let mut __flight_values = ${owner}; __flight_values.sort_by(|left, right| ${ordering}); __flight_values }`;
    }
    if (collectionType?.kind === 'array' && method === 'slice') {
      const start = expression.arguments[0] ? emitExpression(expression.arguments[0], context) : '0.0_f64';
      const end = expression.arguments[1]
        ? emitExpression(expression.arguments[1], context)
        : `${parenthesize(owner)}.len() as f64`;
      return `${parenthesize(owner)}[${parenthesize(start)} as usize..${parenthesize(end)} as usize].to_vec()`;
    }
    if (collectionType?.kind === 'array' && method === 'push') {
      if (expression.arguments.length === 0) throw new RustEmissionError('Array.push requires an argument');
      if (expression.arguments.some((argument) => argument.kind === 'spread')) {
        const operations = expression.arguments.map((argument) => {
          if (argument.kind === 'spread') {
            return `${ownerPlace}.extend(${parenthesize(emitExpression(argument.expression, context))}.iter().cloned());`;
          }
          return `${ownerPlace}.push(${emitExpression(argument, context, collectionType.element)});`;
        });
        return `{ ${operations.join(' ')} ${ownerPlace}.len() as f64 }`;
      }
      const values = expression.arguments.map((argument) => {
        const emitted = emitExpression(argument, context, collectionType.element);
        const actualType = inferIrExpressionType(argument, context);
        return argument.kind === 'identifier' && actualType && !isCopyType(actualType, context)
          ? `${parenthesize(emitted)}.clone()`
          : emitted;
      });
      return values.length === 1
        ? `${ownerPlace}.push(${values[0]})`
        : `${ownerPlace}.extend(vec![${values.join(', ')}])`;
    }
    if (collectionType?.kind === 'array' && method === 'reduce') {
      const callback = expression.arguments[0];
      const initial = expression.arguments[1];
      if (callback?.kind !== 'function' || !initial) {
        throw new RustEmissionError('Array.reduce requires a callback and initial value');
      }
      const accumulatorType = inferIrExpressionType(initial, context) ?? primitive('Float');
      const closureType: IrType = {
        kind: 'function',
        parameters: [accumulatorType, collectionType.element],
        returns: accumulatorType,
      };
      return `${parenthesize(owner)}.iter().cloned().fold(${emitExpression(initial, context)}, ${emitClosure(callback, context, closureType, false)})`;
    }
    if (collectionType?.kind === 'array' && method === 'map') {
      const callback = expression.arguments[0];
      if (!callback) throw new RustEmissionError('Array.map requires a callback');
      if (callback.kind === 'function') {
        const contextualCollection = resolveSemanticType(expectedType, context) ?? expectedType;
        const contextualElement = contextualCollection?.kind === 'array' ? contextualCollection.element : undefined;
        const declaredReturns = callback.returns?.kind === 'dynamic' ? undefined : callback.returns;
        const inferredReturns = inferContextualFunctionExpressionReturnType(
          callback,
          [collectionType.element],
          context,
        );
        const returns =
          declaredReturns ??
          contextualElement ??
          (inferredReturns?.kind === 'dynamic' ? undefined : inferredReturns) ??
          ({ kind: 'dynamic' } as const);
        const closureType: IrType = {
          kind: 'function',
          parameters: [collectionType.element],
          returns,
        };
        return `${parenthesize(owner)}.iter().cloned().map(${emitClosure(callback, context, closureType, false)}).collect::<Vec<_>>()`;
      }
      if (callback.kind === 'identifier') {
        const declaration = context.functions.get(callback.name);
        const localType = resolveSemanticType(context.symbolTypes.get(callback.name), context);
        if (declaration || localType?.kind === 'function') {
          const itemName = '__flight_item';
          const callbackContext: EmitContext = {
            ...context,
            symbolTypes: new Map(context.symbolTypes),
          };
          callbackContext.symbolTypes.set(itemName, collectionType.element);
          const call: Extract<IrExpression, { kind: 'call' }> = {
            arguments: [{ kind: 'identifier', name: itemName }],
            callee: callback,
            kind: 'call',
            typeArguments: [],
          };
          return `${parenthesize(owner)}.iter().cloned().map(|${itemName}| ${emitCall(call, callbackContext)}).collect::<Vec<_>>()`;
        }
      }
      throw new RustEmissionError('Array.map requires an inline or inferred named callback');
    }
    if (collectionType?.kind === 'array' && method === 'filter') {
      const callback = expression.arguments[0];
      if (callback?.kind !== 'function') throw new RustEmissionError('Array.filter requires an inline callback');
      const closureType: IrType = {
        kind: 'function',
        parameters: [collectionType.element],
        returns: primitive('Bool'),
      };
      return `{ let mut __flight_filter = ${emitClosure(callback, context, closureType, false)}; ${parenthesize(owner)}.iter().cloned().filter(|value| __flight_filter(value.clone())).collect::<Vec<_>>() }`;
    }
    if (collectionType?.kind === 'array' && method === 'join') {
      const element = resolveSemanticType(collectionType.element, context) ?? collectionType.element;
      const separator = expression.arguments[0];
      const separatorType = separator ? inferIrExpressionType(separator, context) : undefined;
      const emittedSeparator = !separator
        ? '","'
        : separatorType?.kind === 'nullable'
          ? `${parenthesize(emitExpression(separator, context, separatorType))}.as_deref().unwrap_or(",")`
          : `${parenthesize(emitExpression(separator, context, primitive('String')))}.as_str()`;
      const item =
        element.kind === 'nullable'
          ? 'value.as_ref().map_or_else(String::new, |value| value.to_string())'
          : 'value.to_string()';
      return `${parenthesize(owner)}.iter().map(|value| ${item}).collect::<Vec<_>>().join(${emittedSeparator})`;
    }
    if (collectionType?.kind === 'array' && method === 'find') {
      const callback = expression.arguments[0];
      if (callback?.kind !== 'function') throw new RustEmissionError('Array.find requires a callback');
      const closureType: IrType = {
        kind: 'function',
        parameters: [collectionType.element],
        returns: primitive('Bool'),
      };
      return `${parenthesize(owner)}.iter().find(|value| ${parenthesize(emitClosure(callback, context, closureType, false))}((*value).clone())).cloned()`;
    }
    if (collectionType?.kind === 'array' && (method === 'every' || method === 'some')) {
      const callback = expression.arguments[0];
      if (!callback) throw new RustEmissionError(`Array.${method} requires a callback`);
      const closureType: IrType = {
        kind: 'function',
        parameters: [collectionType.element],
        returns: primitive('Bool'),
      };
      let predicate: string;
      if (callback.kind === 'function') {
        predicate = emitClosure(callback, context, closureType, false);
      } else if (
        callback.kind === 'identifier' &&
        (context.functions.has(callback.name) ||
          resolveSemanticType(context.symbolTypes.get(callback.name), context)?.kind === 'function')
      ) {
        const itemName = '__flight_item';
        const callbackContext: EmitContext = {
          ...context,
          symbolTypes: new Map(context.symbolTypes),
        };
        callbackContext.symbolTypes.set(itemName, collectionType.element);
        const call: Extract<IrExpression, { kind: 'call' }> = {
          arguments: [{ kind: 'identifier', name: itemName }],
          callee: callback,
          kind: 'call',
          typeArguments: [],
        };
        predicate = `|${itemName}| ${emitCall(call, callbackContext)}`;
      } else {
        throw new RustEmissionError(`Array.${method} requires an inline or inferred named callback`);
      }
      return `${parenthesize(owner)}.iter().cloned().${method === 'every' ? 'all' : 'any'}(${predicate})`;
    }
    if (collectionType?.kind === 'named' && typedArrayType(collectionType.name) && method === 'subarray') {
      const start = expression.arguments[0] ? emitExpression(expression.arguments[0], context) : '0.0_f64';
      const end = expression.arguments[1] ? emitExpression(expression.arguments[1], context) : `${owner}.len() as f64`;
      return `${owner}[${parenthesize(start)} as usize..${parenthesize(end)} as usize].to_vec()`;
    }
  }
  const callee = emitExpression(expression.callee, context);
  const calleeType = resolveSemanticType(inferIrExpressionType(expression.callee, context), context);
  const callbackTypeParameter = inferCallbackTypeParameter(expression.callee, context);
  if (callbackTypeParameter) {
    const arguments_ = emitCallbackArguments(expression.arguments, callbackTypeParameter, context);
    return `crate::FlightCallback::flight_call(&${parenthesize(callee)}, ${arguments_})`;
  }
  if (calleeType?.kind === 'function') {
    const arguments_ = expression.arguments.map((argument, index) =>
      emitFunctionCallArgument(
        argument.kind === 'spread' ? argument.expression : argument,
        calleeType.parameters[index],
        context,
      ),
    );
    if (expression.callee.kind === 'identifier' && context.rawClosureNames.has(expression.callee.name)) {
      return `${callee}(${arguments_.join(', ')})`;
    }
    const call = emitLockedCallbackCall(callee, arguments_);
    const projected = expectedType
      ? emitStructuralProjectionArgument(call, calleeType.returns, expectedType, context)
      : undefined;
    return projected ?? call;
  }
  if (calleeType?.kind === 'nullable') {
    const callbackType = resolveSemanticType(calleeType.inner, context);
    if (callbackType?.kind === 'function') {
      const arguments_ = expression.arguments.map((argument, index) =>
        emitFunctionCallArgument(
          argument.kind === 'spread' ? argument.expression : argument,
          callbackType.parameters[index],
          context,
        ),
      );
      if (expression.callee.kind === 'property' && expression.callee.optional) {
        const optional = emitExpression(expression.callee, context, calleeType);
        return `{ let __flight_callback = ${optional}; __flight_callback.as_ref().map(|callback| callback.lock().unwrap()(${arguments_.join(', ')})) }`;
      }
      const call = emitLockedCallbackCall(
        `${emitExpression(expression.callee, context)}.as_ref().unwrap().clone()`,
        arguments_,
      );
      const projected = expectedType
        ? emitStructuralProjectionArgument(call, callbackType.returns, expectedType, context)
        : undefined;
      return projected ?? call;
    }
  }
  if (calleeType?.kind === 'dynamic') {
    const result = expectedType ?? primitive('Void');
    return emitHostValueExpression(result, '"host.call"', context);
  }
  const arguments_ = expression.arguments.map((argument) =>
    argument.kind === 'spread' ? emitExpression(argument.expression, context) : emitExpression(argument, context),
  );
  return `${expression.callee.kind === 'property' ? parenthesize(callee) : callee}(${arguments_.join(', ')})`;
}

function emitFunctionCallArgument(
  argument: IrExpression,
  expectedType: IrType | undefined,
  context: EmitContext,
): string {
  if (!expectedType) return emitExpression(argument, context);
  const actualType = inferIrExpressionType(argument, context);
  const actual = resolveSemanticType(actualType, context);
  const expected = resolveSemanticType(expectedType, context);
  if (actualType && actual?.kind === 'anonymous' && expected?.kind === 'anonymous') {
    const projected = emitStructuralProjectionArgument(
      emitExpression(argument, context, actualType),
      actualType,
      expectedType,
      context,
    );
    if (projected) return projected;
  }
  return emitExpression(argument, context, expectedType);
}

function emitPromiseCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  context: EmitContext,
): string | undefined {
  if (
    expression.callee.kind !== 'property' ||
    (expression.callee.name !== 'then' && expression.callee.name !== 'catch')
  ) {
    return undefined;
  }
  const promise = promiseType(inferIrExpressionType(expression.callee.object, context), context);
  if (!promise) return undefined;
  const callbackExpression = expression.arguments[0];
  if (!callbackExpression) return undefined;
  const inferredCallback = resolveSemanticType(inferIrExpressionType(callbackExpression, context), context);
  const callbackReturns = inferredCallback?.kind === 'function' ? inferredCallback.returns : primitive('Void');
  const callbackType: IrType =
    expression.callee.name === 'then'
      ? {
          kind: 'function',
          parameters: [promise.output],
          returns: callbackReturns,
        }
      : {
          kind: 'function',
          parameters: inferredCallback?.kind === 'function' ? inferredCallback.parameters : [],
          returns: callbackReturns,
        };
  const owner = emitExpression(expression.callee.object, context, promise);
  const callback = emitExpression(callbackExpression, context, callbackType);
  void owner;
  void callback;
  throw new RustEmissionError(
    `${expression.callee.name === 'catch' ? 'taskCatch' : 'taskThen'} Rust lowering is reserved for Pass 27 Stage 4`,
  );
}

function emitTaskOrigin(origin: IrTaskOrigin): string {
  return [
    'crate::FlightTaskOrigin {',
    `package: ${emitRustStringLiteral(origin.packageName)},`,
    `source: ${emitRustStringLiteral(origin.source)},`,
    `line: ${String(origin.line)}_u32,`,
    `column: ${String(origin.column)}_u32,`,
    `lexical_path: ${emitRustStringLiteral(origin.lexicalPath)},`,
    `fingerprint: ${emitRustStringLiteral(origin.fingerprint)},`,
    '}',
  ].join(' ');
}

function emitTaskRejection(expression: IrExpression, context: EmitContext): string {
  if (expression.kind === 'literal') {
    if (expression.value === null) return 'crate::FlightRejection::Null';
    if (typeof expression.value === 'boolean') return `crate::FlightRejection::Bool(${String(expression.value)})`;
    if (typeof expression.value === 'number') return `crate::FlightRejection::Number(${emitLiteral(expression.value)})`;
    if (typeof expression.value === 'string') {
      return `crate::FlightRejection::String(${emitRustStringLiteral(expression.value)}.to_owned())`;
    }
  }
  if (
    expression.kind === 'new' &&
    expression.callee.kind === 'identifier' &&
    expression.callee.name === 'Error' &&
    expression.arguments[0]
  ) {
    return `crate::FlightRejection::Error { name: "Error".to_owned(), message: ${emitExpression(expression.arguments[0], context, { kind: 'primitive', name: 'String' })} }`;
  }
  throw new RustEmissionError('taskReject requires a typed null, boolean, number, string, or Error rejection');
}

function typeContainsDynamic(type: IrType): boolean {
  switch (type.kind) {
    case 'dynamic':
      return !type.portable;
    case 'anonymous':
      return type.extends.some(typeContainsDynamic) || type.fields.some((field) => typeContainsDynamic(field.type));
    case 'array':
      return typeContainsDynamic(type.element);
    case 'function':
      return type.parameters.some(typeContainsDynamic) || typeContainsDynamic(type.returns);
    case 'named':
      return type.arguments.some(typeContainsDynamic);
    case 'nullable':
      return typeContainsDynamic(type.inner);
    case 'task':
      return typeContainsDynamic(type.output);
    case 'union':
      return type.variants.some(typeContainsDynamic);
    case 'primitive':
      return false;
  }
}

function typeContainsAnyDynamic(type: IrType): boolean {
  switch (type.kind) {
    case 'dynamic':
      return true;
    case 'anonymous':
      return (
        type.extends.some(typeContainsAnyDynamic) || type.fields.some((field) => typeContainsAnyDynamic(field.type))
      );
    case 'array':
      return typeContainsAnyDynamic(type.element);
    case 'function':
      return type.parameters.some(typeContainsAnyDynamic) || typeContainsAnyDynamic(type.returns);
    case 'named':
      return type.arguments.some(typeContainsAnyDynamic);
    case 'nullable':
      return typeContainsAnyDynamic(type.inner);
    case 'task':
      return typeContainsAnyDynamic(type.output);
    case 'union':
      return type.variants.some(typeContainsAnyDynamic);
    case 'primitive':
      return false;
  }
}

function promiseType(type: IrType | undefined, context: EmitContext): Extract<IrType, { kind: 'task' }> | undefined {
  const resolved = resolveSemanticType(type?.kind === 'nullable' ? type.inner : type, context);
  return resolved?.kind === 'task' ? resolved : undefined;
}

function emitLockedCallbackCall(callback: string, arguments_: readonly string[]): string {
  return `{ let __flight_callback = ${callback}; let __flight_result = __flight_callback.lock().unwrap()(${arguments_.join(', ')}); __flight_result }`;
}

function emitOptionalCall(expression: Extract<IrExpression, { kind: 'call' }>, context: EmitContext): string {
  if (
    expression.callee.kind === 'call' &&
    expression.callee.callee.kind === 'property' &&
    expression.callee.callee.name === 'get'
  ) {
    const callee = emitCall(expression.callee, context);
    const arguments_ = expression.arguments.map((argument) =>
      argument.kind === 'spread' ? emitExpression(argument.expression, context) : emitExpression(argument, context),
    );
    return `{ let __flight_callback = ${callee}; __flight_callback.as_ref().map(|callback| callback.lock().unwrap()(${arguments_.join(', ')})) }`;
  }
  const hostReturnType = inferOptionalHostCallReturnType(expression, context);
  if (hostReturnType) {
    if (hostReturnType.kind === 'primitive' && hostReturnType.name === 'Bool') return 'Some(false)';
    if (hostReturnType.kind === 'primitive' && hostReturnType.name === 'Void') return 'Some(())';
    return `None::<${emitType(hostReturnType, context)}>`;
  }
  const calleeType = resolveSemanticType(inferIrExpressionType(expression.callee, context), context);
  const callbackType = calleeType?.kind === 'nullable' ? resolveSemanticType(calleeType.inner, context) : calleeType;
  if (callbackType?.kind !== 'function') {
    throw new RustEmissionError(
      `optional call requires an inferred nullable function: ${JSON.stringify(expression.callee)}`,
    );
  }
  const callee = emitExpression(expression.callee, context);
  const arguments_ = expression.arguments.map((argument, index) =>
    argument.kind === 'spread'
      ? emitExpression(argument.expression, context, callbackType.parameters[index])
      : emitExpression(argument, context, callbackType.parameters[index]),
  );
  return `{ let __flight_callback = ${callee}; __flight_callback.as_ref().map(|callback| callback.lock().unwrap()(${arguments_.join(', ')})) }`;
}

function inferOptionalHostCallReturnType(
  expression: Extract<IrExpression, { kind: 'call' }>,
  context: EmitContext,
): IrType | undefined {
  if (!expression.optional || expression.callee.kind !== 'property') return undefined;
  const receiver = resolveSemanticType(inferIrExpressionType(expression.callee.object, context), context);
  if (receiver?.kind !== 'dynamic' && !expression.callee.binding) return undefined;
  if (expression.callee.name === 'getModifierState') return primitive('Bool');
  if (['addEventListener', 'removeEventListener'].includes(expression.callee.name)) return primitive('Void');
  if (['exitFullscreen', 'requestFullscreen', 'requestPointerLock'].includes(expression.callee.name)) {
    return { kind: 'dynamic' };
  }
  return undefined;
}

function inferKnownHostCallReturnType(
  expression: Extract<IrExpression, { kind: 'call' }>,
  context: EmitContext,
): IrType | undefined {
  if (expression.callee.kind !== 'property') return undefined;
  const receiver = resolveSemanticType(inferIrExpressionType(expression.callee.object, context), context);
  const globalReceiver =
    expression.callee.object.kind === 'identifier' &&
    ['navigator', 'performance'].includes(expression.callee.object.name);
  if (
    receiver?.kind !== 'dynamic' &&
    !expression.callee.binding &&
    !globalReceiver &&
    !isDynamicHostTree(expression, context)
  ) {
    return undefined;
  }
  if (['charCodeAt', 'now'].includes(expression.callee.name)) return primitive('Float');
  if (expression.callee.name === 'contains') return primitive('Bool');
  if (['toLowerCase', 'toUpperCase', 'trim'].includes(expression.callee.name)) return primitive('String');
  if (
    expression.callee.binding !== 'DomDocumentBackend' &&
    ['createElement', 'createTextNode'].includes(expression.callee.name)
  ) {
    return { kind: 'dynamic' };
  }
  if (['getCoalescedEvents', 'segment'].includes(expression.callee.name)) {
    return { element: { kind: 'dynamic' }, kind: 'array' };
  }
  if (expression.callee.name === 'getGamepads') {
    return {
      element: { inner: { kind: 'dynamic' }, kind: 'nullable' },
      kind: 'array',
    };
  }
  if (['createBufferSource', 'createGain', 'createStereoPanner', 'resume'].includes(expression.callee.name)) {
    return { kind: 'dynamic' };
  }
  return undefined;
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

function emitStructuralFunctionInlineCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
  allowMutations = false,
): string {
  if (
    declaration.parameters.some(
      (parameter) => parameter.rest || (!allowMutations && collectMutatedNames(declaration).has(parameter.name)),
    )
  ) {
    return emitKnownFunctionCall(expression, declaration, context);
  }
  const bindings = new Map<string, IrExpression>();
  const localParameters: IrVariable[] = [];
  const mutated = collectMutatedNames(declaration, context.mutatingFunctions);
  const structuralMutationIndexes = allowMutations
    ? mutableStructuralParameterIndexes(expression, declaration, context)
    : new Set<number>();
  declaration.parameters.forEach((parameter, index) => {
    const argument = expression.arguments[index];
    const initializer = argument ?? parameter.initializer;
    if (!initializer) return;
    if (allowMutations && mutated.has(parameter.name) && !structuralMutationIndexes.has(index)) {
      const name = `__flight_inline_${safeName(parameter.name)}_${String(index)}`;
      const actualType = argument
        ? inferIrExpressionType(argument, context)
        : parameter.optional
          ? ({ inner: parameter.type, kind: 'nullable' } as const)
          : parameter.type;
      localParameters.push({
        initializer,
        mutable: true,
        name,
        type: actualType ?? parameter.type,
      });
      bindings.set(parameter.name, { kind: 'identifier', name });
      return;
    }
    bindings.set(parameter.name, initializer);
  });
  const missing = declaration.parameters.filter((parameter) => !bindings.has(parameter.name) && !parameter.optional);
  if (missing.length > 0) {
    throw new RustEmissionError(
      `${declaration.name} call is missing required argument ${missing.map((parameter) => parameter.name).join(', ')}`,
    );
  }
  const body = substituteIdentifiers(declaration.body, bindings);
  const statements: IrStatement[] = [
    ...(localParameters.length > 0 ? [{ declarations: localParameters, kind: 'variable' } as const] : []),
    ...body,
  ];
  if (allowMutations && !declaration.body.some((statement) => containsStatementKind(statement, 'return'))) {
    return emitStatementsAsBlock(statements, context);
  }
  const closure: Extract<IrExpression, { kind: 'function' }> = {
    body: statements,
    execution: { kind: 'sync' },
    kind: 'function',
    parameters: [],
    returns: declaration.returns,
  };
  const constantNames = new Map(context.constantNames);
  for (const name of collectScreamingSnakeIdentifiers(declaration.body)) {
    if (!constantNames.has(name)) constantNames.set(name, `crate::${name}`);
  }
  return `${parenthesize(
    emitClosure(
      closure,
      { ...context, constantNames },
      { kind: 'function', parameters: [], returns: declaration.returns },
      false,
      false,
    ),
  )}()`;
}

function requiresMutableStructuralInlining(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
): boolean {
  return mutableStructuralParameterIndexes(expression, declaration, context).size > 0;
}

function mutableStructuralParameterIndexes(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
): ReadonlySet<number> {
  const mutated = collectMutatedNames(declaration, context.mutatingFunctions);
  return new Set(
    declaration.parameters.flatMap((parameter, index) => {
      if (!mutated.has(parameter.name)) return [];
      const argument = expression.arguments[index];
      if (!argument || !isRustPlaceExpression(argument)) return [];
      const actualType = inferIrExpressionType(argument, context);
      if (!actualType || emitType(actualType, context) === emitType(parameter.type, context)) return [];
      return emitStructuralProjectionArgument('__flight_inline_probe', actualType, parameter.type, context)
        ? [index]
        : [];
    }),
  );
}

function typeContainsAnonymousRecord(type: IrType, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context) ?? type;
  switch (resolved.kind) {
    case 'anonymous':
      return true;
    case 'array':
      return typeContainsAnonymousRecord(resolved.element, context);
    case 'function':
      return (
        resolved.parameters.some((parameter) => typeContainsAnonymousRecord(parameter, context)) ||
        typeContainsAnonymousRecord(resolved.returns, context)
      );
    case 'named':
      return resolved.arguments.some((argument) => typeContainsAnonymousRecord(argument, context));
    case 'nullable':
      return typeContainsAnonymousRecord(resolved.inner, context);
    case 'task':
      return typeContainsAnonymousRecord(resolved.output, context);
    case 'union':
      return resolved.variants.some((variant) => typeContainsAnonymousRecord(variant, context));
    case 'dynamic':
    case 'primitive':
      return false;
  }
}

function isStructuralInlineHelper(declaration: IrFunctionDeclaration): boolean {
  return containsStructuralUtilityCast(declaration.body) || containsJsonStringifyCall(declaration.body);
}

function containsStructuralUtilityCast(value: unknown): boolean {
  if (!value || typeof value !== 'object') return false;
  if ('kind' in value && value.kind === 'cast' && 'type' in value && isStructuralUtilityType(value.type as IrType)) {
    return true;
  }
  return Object.values(value).some((item) =>
    Array.isArray(item)
      ? item.some((child) => containsStructuralUtilityCast(child))
      : containsStructuralUtilityCast(item),
  );
}

function containsJsonStringifyCall(value: unknown): boolean {
  if (!value || typeof value !== 'object') return false;
  if (
    'kind' in value &&
    value.kind === 'call' &&
    'callee' in value &&
    value.callee &&
    typeof value.callee === 'object' &&
    'kind' in value.callee &&
    value.callee.kind === 'property' &&
    'name' in value.callee &&
    value.callee.name === 'stringify' &&
    'object' in value.callee &&
    value.callee.object &&
    typeof value.callee.object === 'object' &&
    'kind' in value.callee.object &&
    value.callee.object.kind === 'identifier' &&
    'name' in value.callee.object &&
    value.callee.object.name === 'JSON'
  ) {
    return true;
  }
  return Object.values(value).some((item) =>
    Array.isArray(item) ? item.some((child) => containsJsonStringifyCall(child)) : containsJsonStringifyCall(item),
  );
}

function collectScreamingSnakeIdentifiers(value: unknown): ReadonlySet<string> {
  const found = new Set<string>();
  const visit = (item: unknown): void => {
    if (!item || typeof item !== 'object') return;
    if (
      'kind' in item &&
      item.kind === 'identifier' &&
      'name' in item &&
      typeof item.name === 'string' &&
      /^[A-Z][A-Z0-9_]*$/u.test(item.name)
    ) {
      found.add(item.name);
    }
    for (const child of Object.values(item)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
  return found;
}

function emitKnownFunctionCall(
  expression: Extract<IrExpression, { kind: 'call' }>,
  declaration: IrFunctionDeclaration,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const substitutions = inferFunctionTypeSubstitutions(declaration, expression, context, expectedType);
  const restIndex = declaration.parameters.findIndex((parameter) => parameter.rest);
  if (restIndex < 0) {
    const directlyMutated = collectMutatedNames(declaration);
    const mutableIndexes = new Set(
      declaration.parameters.flatMap((parameter, index) =>
        (context.mutatingFunctions.get(declaration.name)?.has(index) ?? false) || directlyMutated.has(parameter.name)
          ? [index]
          : [],
      ),
    );
    const mutableRoots = new Set(
      [...mutableIndexes].flatMap((index) => {
        const argument = expression.arguments[index];
        const root = argument ? expressionRootIdentifier(argument) : undefined;
        return root ? [root] : [];
      }),
    );
    const mutableRootCounts = new Map<string, number>();
    for (const index of mutableIndexes) {
      const argument = expression.arguments[index];
      const root = argument ? expressionRootIdentifier(argument) : undefined;
      if (root) mutableRootCounts.set(root, (mutableRootCounts.get(root) ?? 0) + 1);
    }
    const prefix: string[] = [];
    const suffix: string[] = [];
    const arguments_ = declaration.parameters.map((parameter, index) => {
      const argument = expression.arguments[index];
      if (!argument) {
        if (parameter.optional || parameter.initializer) return 'None';
        throw new RustEmissionError(`${declaration.name} call is missing required argument ${parameter.name}`);
      }
      const parameterType = substituteIrType(parameter.type, substitutions);
      const root = expressionRootIdentifier(argument);
      const genericValueParameter =
        parameter.type.kind === 'named' &&
        parameter.type.arguments.length === 0 &&
        declaration.typeParameters.includes(parameter.type.name);
      const owned =
        genericValueParameter ||
        isConcreteCallbackHandleType(parameterType, context) ||
        capturesParameterInReturnedClosure(declaration, parameter.name) ||
        ((resolveSemanticType(parameterType, context) ?? parameterType).kind === 'function' &&
          storesParameter(declaration, parameter.name));
      const recursiveStorage =
        mutableIndexes.has(index) && root && (mutableRootCounts.get(root) ?? 0) > 1 && argument.kind === 'property'
          ? recursiveStructPropertyStorage(inferIrExpressionType(argument.object, context), argument.name, context)
          : undefined;
      const aliasedOwnerIndex =
        root && recursiveStorage === 'nullable'
          ? [...mutableIndexes].find(
              (candidate) => candidate !== index && expressionRootIdentifier(expression.arguments[candidate]!) === root,
            )
          : undefined;
      const aliasedOwnerParameter =
        aliasedOwnerIndex === undefined ? undefined : declaration.parameters[aliasedOwnerIndex];
      if (
        recursiveStorage === 'nullable' &&
        argument.kind === 'property' &&
        aliasedOwnerParameter &&
        clearsPropertyWithoutReading(declaration, aliasedOwnerParameter.name, argument.name)
      ) {
        const temporary = `__flight_argument_${String(index)}`;
        const place = emitPropertyPlace(argument, context);
        prefix.push(
          `let mut ${temporary} = ${place}.replace(Box::new(Default::default())).expect("narrowed recursive field was absent");`,
        );
        suffix.push(`if ${place}.is_some() { ${place} = Some(${temporary}); }`);
        return `&mut *${temporary}`;
      }
      if (
        !mutableIndexes.has(index) &&
        ((root && mutableRoots.has(root)) || referencesAnyIdentifier(argument, mutableRoots))
      ) {
        const temporary = `__flight_argument_${String(index)}`;
        const valueType = parameterType.kind === 'nullable' ? parameterType.inner : parameterType;
        const referenceLike = isReferenceLike(valueType, context);
        const value = referenceLike
          ? `${parenthesize(emitPlaceExpression(argument, context))}.clone()`
          : emitKnownFunctionArgument(argument, { ...parameter, type: parameterType }, context, false, owned);
        prefix.push(`let ${temporary} = ${value};`);
        return referenceLike ? `&${temporary}` : temporary;
      }
      return emitKnownFunctionArgument(
        argument,
        { ...parameter, type: parameterType },
        context,
        mutableIndexes.has(index),
        owned,
      );
    });
    const call = `${snakeCase(declaration.name)}(${arguments_.join(', ')})`;
    return prefix.length > 0 || suffix.length > 0
      ? `{ ${prefix.join(' ')} let __flight_result = ${call}; ${suffix.join(' ')} __flight_result }`
      : call;
  }
  const fixed = expression.arguments.slice(0, restIndex).map((argument, index) => {
    const parameter = declaration.parameters[index];
    if (!parameter) return emitExpression(argument, context);
    const parameterType = substituteIrType(parameter.type, substitutions);
    const mutable =
      (context.mutatingFunctions.get(declaration.name)?.has(index) ?? false) ||
      collectMutatedNames(declaration).has(parameter.name);
    return emitKnownFunctionArgument(argument, { ...parameter, type: parameterType }, context, mutable);
  });
  const rest = declaration.parameters[restIndex]!;
  const restType = substituteIrType(rest.type, substitutions);
  const element = restType.kind === 'array' ? restType.element : undefined;
  const restArguments = expression.arguments.slice(restIndex);
  if (restArguments.length === 1 && restArguments[0]?.kind === 'spread') {
    return `${snakeCase(declaration.name)}(${[
      ...fixed,
      emitExpression(restArguments[0].expression, context, restType),
    ].join(', ')})`;
  }
  if (restType.kind === 'named' && restType.name === 'FlightCallbackArgs') {
    const signalType = expression.arguments[0] ? inferIrExpressionType(expression.arguments[0], context) : undefined;
    const signal = signalType?.kind === 'nullable' ? signalType.inner : signalType;
    const signalCallback =
      signal?.kind === 'named' && signal.name === 'Signal'
        ? resolveSemanticType(signal.arguments[0], context)
        : undefined;
    const callback =
      signalCallback?.kind === 'function' ? signalCallback : resolveSemanticType(restType.arguments[0], context);
    const values = restArguments.map((argument, index) =>
      emitExpression(argument, context, callback?.kind === 'function' ? callback.parameters[index] : undefined),
    );
    const tuple = values.length === 0 ? '()' : `(${values.join(', ')}${values.length === 1 ? ',' : ''})`;
    return `${snakeCase(declaration.name)}(${[...fixed, tuple].join(', ')})`;
  }
  const values = expression.arguments.slice(restIndex).map((argument) => emitExpression(argument, context, element));
  return `${snakeCase(declaration.name)}(${[...fixed, `vec![${values.join(', ')}]`].join(', ')})`;
}

function emitKnownFunctionArgument(
  argument: IrExpression,
  parameter: IrParameter,
  context: EmitContext,
  mutable = false,
  owned = false,
): string {
  owned ||= isConcreteCallbackHandleType(parameter.type, context);
  const argumentType = inferIrExpressionType(argument, context);
  const resolvedParameterType = resolveSemanticType(parameter.type, context) ?? parameter.type;
  const nullableParameter = resolvedParameterType.kind === 'nullable';
  const expectedType = nullableParameter ? resolvedParameterType.inner : parameter.type;
  const resolvedExpectedType = resolveSemanticType(expectedType, context);
  if (mutable) {
    const portableRecord = emitMutablePortableRecordCastArgument(argument, resolvedExpectedType, context);
    if (portableRecord) return portableRecord;
  }
  if (
    resolvedExpectedType?.kind === 'anonymous' &&
    (expectedType.kind === 'anonymous' || isStructuralUtilityType(expectedType))
  ) {
    registerContextualAnonymousTypes(resolvedExpectedType, context, parameter.name);
  }
  const optionalParameter = Boolean(parameter.optional || parameter.initializer) && !nullableParameter;
  const borrowedNullableReference =
    nullableParameter &&
    !owned &&
    !isSharedHandleType(expectedType, context) &&
    !(expectedType.kind === 'named' && expectedType.name === 'Signal' && !mutable) &&
    isReferenceLike(expectedType, context);
  if (argument.kind === 'literal' && argument.value === null && !borrowedNullableReference) return 'None';
  if (borrowedNullableReference) {
    const projected =
      !mutable && argumentType
        ? emitCollectionProjectionExpression(argument, argumentType, parameter.type, context)
        : undefined;
    if (projected) return `&${parenthesize(projected)}`;
    const root = expressionRootIdentifier(argument);
    const value =
      argumentType?.kind === 'nullable' && isRustPlaceExpression(argument)
        ? emitPlaceExpression(argument, context)
        : emitExpression(argument, context, parameter.type);
    return argumentType?.kind === 'nullable' &&
      argument.kind === 'identifier' &&
      root &&
      context.borrowedNames.has(root)
      ? value
      : `${mutable ? '&mut ' : '&'}${parenthesize(value)}`;
  }
  if (optionalParameter && argumentType?.kind === 'nullable') {
    const value = emitExpression(argument, context, argumentType);
    const projected = emitStructuralProjectionArgument('__flight_value', argumentType.inner, expectedType, context);
    if (projected) {
      return `${parenthesize(value)}.as_ref().map(|__flight_value| ${projected})`;
    }
    if (semanticTypesEqual(argumentType.inner, expectedType, context)) return `${parenthesize(value)}.clone()`;
  }
  if (optionalParameter && argument.kind !== 'object' && argumentType) {
    const value = emitExpression(argument, context, argumentType);
    const projected = emitStructuralProjectionArgument(value, argumentType, expectedType, context);
    if (projected) return `Some(${projected})`;
  }
  if (!nullableParameter && !optionalParameter && expectedType.kind === 'union') {
    return `&${parenthesize(emitExpression(argument, context, expectedType))}`;
  }

  if (
    !nullableParameter &&
    !optionalParameter &&
    !owned &&
    expectedType.kind !== 'union' &&
    !isSharedHandleType(expectedType, context) &&
    !(expectedType.kind === 'named' && expectedType.name === 'Signal' && !mutable) &&
    isReferenceLike(expectedType, context)
  ) {
    const root = expressionRootIdentifier(argument);
    const resolvedExpected = resolvedExpectedType ?? expectedType;
    if (resolvedExpected.kind === 'function') {
      const resolvedArgument = resolveSemanticType(argumentType, context) ?? argumentType;
      if (
        argument.kind !== 'function' &&
        resolvedArgument?.kind === 'function' &&
        !(argument.kind === 'identifier' && context.rawClosureNames.has(argument.name))
      ) {
        const callback = emitExpression(argument, context, argumentType);
        const parameters = resolvedExpected.parameters.map(
          (type, index) => `__flight_callback_argument_${String(index)}: ${emitType(type, context)}`,
        );
        const values = resolvedExpected.parameters.map((_type, index) => `__flight_callback_argument_${String(index)}`);
        return `&mut |${parameters.join(', ')}| -> ${emitType(resolvedExpected.returns, context)} { let __flight_callback = ${callback}; __flight_callback.lock().unwrap()(${values.join(', ')}) }`;
      }
      const value =
        argument.kind === 'function'
          ? emitClosure(argument, context, expectedType, false)
          : emitPlaceExpression(argument, context);
      return argument.kind === 'identifier' && context.borrowedNames.has(argument.name) ? value : `&mut ${value}`;
    }
    const value =
      argument.kind === 'function'
        ? emitExpression(argument, context, expectedType)
        : argument.kind === 'object'
          ? emitExpression(argument, context, expectedType)
          : isRustPlaceExpression(argument)
            ? emitPlaceExpression(argument, context)
            : emitExpression(argument, context, expectedType);
    const collectionProjection =
      !mutable && argumentType
        ? emitCollectionProjectionExpression(argument, argumentType, expectedType, context)
        : undefined;
    if (collectionProjection) return `&${parenthesize(collectionProjection)}`;
    if (argumentType?.kind === 'nullable' && semanticTypesEqual(argumentType.inner, expectedType, context)) {
      return `${value}.${mutable ? 'as_mut' : 'as_ref'}().unwrap()`;
    }
    const resolvedArgument = resolveSemanticType(argumentType, context) ?? argumentType;
    let structuralProjection: string | undefined;
    if (!mutable && argument.kind !== 'object' && argumentType) {
      if (resolvedArgument?.kind === 'union') {
        const probe = emitStructuralProjectionArgument(
          '__flight_union_projection',
          argumentType,
          expectedType,
          context,
        );
        structuralProjection = probe
          ? emitStructuralProjectionArgument(
              emitExpression(argument, context, argumentType),
              argumentType,
              expectedType,
              context,
            )
          : undefined;
      } else {
        structuralProjection = emitStructuralProjectionArgument(value, argumentType, expectedType, context);
      }
    }
    if (structuralProjection) return `&${structuralProjection}`;
    return argument.kind === 'identifier' && root && context.borrowedNames.has(root)
      ? value
      : `${mutable ? '&mut ' : '&'}${value}`;
  }

  let emitted = emitExpression(
    argument,
    context,
    nullableParameter && argumentType?.kind === 'nullable' ? parameter.type : expectedType,
  );
  if (
    argument.kind === 'identifier' &&
    argumentType &&
    !isCopyType(argumentType, context) &&
    !nullableParameter &&
    !optionalParameter &&
    !context.unionNarrowings.has(argument.name)
  ) {
    const value = argumentType.kind === 'nullable' ? emitted : emitPlaceExpression(argument, context);
    emitted = `${parenthesize(value)}.clone()`;
  }
  if (
    argument.kind !== 'object' &&
    (nullableParameter || optionalParameter) &&
    argumentType &&
    !isCopyType(argumentType, context)
  ) {
    emitted = `${parenthesize(emitted)}.clone()`;
  }
  if (nullableParameter && argumentType?.kind !== 'nullable') emitted = `Some(${emitted})`;
  if (optionalParameter) emitted = `Some(${emitted})`;
  return emitted;
}

function inferFunctionTypeSubstitutions(
  declaration: IrFunctionDeclaration,
  expression: Extract<IrExpression, { kind: 'call' }>,
  context: EmitContext,
  expectedType?: IrType,
): ReadonlyMap<string, IrType> {
  const substitutions = new Map<string, IrType>();
  declaration.typeParameters.forEach((parameter, index) => {
    const explicit = expression.typeArguments[index];
    if (explicit) substitutions.set(parameter, explicit);
  });
  const bind = (pattern: IrType, actual: IrType | undefined): void => {
    if (!actual) return;
    if (pattern.kind !== 'nullable' && actual.kind === 'nullable') {
      bind(pattern, actual.inner);
      return;
    }
    if (
      pattern.kind === 'named' &&
      pattern.arguments.length === 0 &&
      declaration.typeParameters.includes(pattern.name)
    ) {
      if (!substitutions.has(pattern.name)) substitutions.set(pattern.name, actual);
      return;
    }
    if (pattern.kind === 'array' && actual.kind === 'array') {
      bind(pattern.element, actual.element);
      return;
    }
    if (pattern.kind === 'nullable') {
      bind(pattern.inner, actual.kind === 'nullable' ? actual.inner : actual);
      return;
    }
    if (pattern.kind === 'named' && actual.kind === 'named' && pattern.name === actual.name) {
      pattern.arguments.forEach((argument, index) => bind(argument, actual.arguments[index]));
    }
  };
  bind(declaration.returns, expectedType);
  declaration.parameters.forEach((parameter, index) => {
    const argument = expression.arguments[index];
    if (argument) {
      const staticType = inferStaticExpressionType(argument);
      bind(
        parameter.type,
        staticType ?? (isRustPlaceExpression(argument) ? inferIrExpressionType(argument, context) : undefined),
      );
    }
  });
  return substitutions;
}

function inferFunctionExpressionReturnType(
  expression: Extract<IrExpression, { kind: 'function' }>,
): IrType | undefined {
  if (expression.returns) return expression.returns;
  if (expression.expression) return inferStaticExpressionType(expression.expression);
  for (const statement of expression.body) {
    if (statement.kind === 'return' && statement.expression) {
      return inferStaticExpressionType(statement.expression);
    }
  }
  return undefined;
}

function inferContextualFunctionExpressionReturnType(
  expression: Extract<IrExpression, { kind: 'function' }>,
  fallbackParameters: IrType[],
  context: EmitContext,
): IrType | undefined {
  if (expression.returns && expression.returns.kind !== 'dynamic') return expression.returns;
  const nextContext: EmitContext = {
    ...context,
    symbolTypes: new Map(context.symbolTypes),
  };
  registerParameters(expression.parameters, nextContext, fallbackParameters);
  registerLocalTypes(expression.body, nextContext);
  if (expression.expression) {
    return (
      inferIrExpressionType(expression.expression, nextContext) ?? inferStaticExpressionType(expression.expression)
    );
  }
  for (const statement of expression.body) {
    if (statement.kind === 'return' && statement.expression) {
      return (
        inferIrExpressionType(statement.expression, nextContext) ?? inferStaticExpressionType(statement.expression)
      );
    }
  }
  return undefined;
}

function emitJsonStringify(expression: IrExpression, context: EmitContext): string {
  const type = inferIrExpressionType(expression, context);
  if (!type) throw new RustEmissionError('JSON.stringify requires a statically recoverable value');
  const value = emitPortableValueExpression(expression, context, type);
  return `crate::flight_json_stringify(&${parenthesize(value)}).expect("JSON.stringify encountered an opaque host object").expect("JSON.stringify returned undefined where Rust requires String")`;
}

function emitPortableValueExpression(expression: IrExpression, context: EmitContext, type?: IrType): string {
  if (expression.kind === 'literal' && expression.value === null) return 'crate::FlightValue::Null';
  if (expression.kind === 'identifier' && expression.name.toLowerCase() === 'undefined') {
    return 'crate::FlightValue::Undefined';
  }
  if (expression.kind === 'object') return emitPortableObjectLiteral(expression, context);
  if (expression.kind === 'array') return emitPortableArrayLiteral(expression, context, type);
  if (expression.kind === 'function') return 'crate::FlightValue::Function';
  if (isSymbolConstruction(expression)) return 'crate::FlightValue::Symbol';
  if (expression.kind === 'conditional') {
    const narrowed = narrowTypeofContexts(expression.condition, context);
    return `if ${emitCondition(expression.condition, context)} { ${emitPortableValueExpression(
      expression.whenTrue,
      narrowed.whenTrue,
    )} } else { ${emitPortableValueExpression(expression.whenFalse, narrowed.whenFalse)} }`;
  }

  let actualType = type ?? inferIrExpressionType(expression, context);
  if (!actualType) {
    throw new RustEmissionError('portable value conversion requires a statically recoverable source type');
  }
  let resolved = resolveSemanticType(actualType, context) ?? actualType;
  if (resolved.kind === 'dynamic') {
    const inferred = inferIrExpressionType(expression, context);
    const inferredResolved = resolveSemanticType(inferred, context) ?? inferred;
    if (inferred && inferredResolved && inferredResolved.kind !== 'dynamic') {
      actualType = inferred;
      resolved = inferredResolved;
    }
  }
  if (resolved.kind === 'dynamic') return emitExpression(expression, context, actualType);
  const source = emitExpression(expression, context, actualType);
  return `{ let __flight_portable_source = ${source}; ${emitPortableValueFromReference(
    '&__flight_portable_source',
    actualType,
    context,
  )} }`;
}

function isPortableValueInlineConversionBounded(type: IrType, context: EmitContext): boolean {
  const budget = { remaining: 64 };
  const visit = (candidate: IrType, ancestors: ReadonlySet<string>): boolean => {
    budget.remaining -= 1;
    if (budget.remaining < 0) return false;
    if (candidate.kind === 'named' && context.enumNames.has(candidate.name)) return true;
    const candidateKey = typeKey(candidate);
    if (ancestors.has(candidateKey)) return false;
    const resolved = resolveSemanticType(candidate, context) ?? candidate;
    const resolvedKey = typeKey(resolved);
    if (ancestors.has(resolvedKey)) return false;
    const next = new Set([...ancestors, candidateKey, resolvedKey]);
    switch (resolved.kind) {
      case 'anonymous':
        return flattenStructFields(resolved, context).every(
          (field) => !(field.optional && field.type.kind === 'nullable') && visit(field.type, next),
        );
      case 'array':
        return visit(resolved.element, next);
      case 'nullable':
        return visit(resolved.inner, next);
      case 'union':
        return resolved.variants.every((variant) => visit(variant, next));
      case 'task':
        return false;
      case 'named':
        if (resolved.name === 'RustMap') {
          return visit(resolved.arguments[1] ?? { kind: 'dynamic' }, next);
        }
        return true;
      case 'dynamic':
      case 'function':
      case 'primitive':
        return true;
    }
  };
  return visit(type, new Set());
}

function emitPortableArrayLiteral(
  expression: Extract<IrExpression, { kind: 'array' }>,
  context: EmitContext,
  type?: IrType,
): string {
  const resolved = resolveSemanticType(type, context) ?? type;
  const elementType = resolved?.kind === 'array' ? resolved.element : undefined;
  const operations = expression.elements.map((item, index) => {
    if (item.kind !== 'spread') {
      const itemType = elementType ?? inferIrExpressionType(item, context);
      return `__flight_array.push(${emitPortableValueExpression(item, context, itemType)});`;
    }
    const spreadType = inferIrExpressionType(item.expression, context);
    const spreadResolved = resolveSemanticType(spreadType, context) ?? spreadType;
    if (
      !spreadType ||
      !(
        spreadResolved?.kind === 'array' ||
        spreadResolved?.kind === 'dynamic' ||
        (spreadResolved?.kind === 'named' && Boolean(typedArrayType(spreadResolved.name)))
      )
    ) {
      throw new RustEmissionError(`portable array spread ${String(index)} requires an array source`);
    }
    const spread = `__flight_spread_${String(index)}`;
    return `let ${spread} = ${emitPortableValueExpression(item.expression, context, spreadType)}; match ${spread} { crate::FlightValue::Array(values) => __flight_array.extend(values), _ => panic!("portable array spread requires an array value") }`;
  });
  return `crate::FlightValue::Array({ let mut __flight_array = Vec::new(); ${operations.join(' ')} __flight_array })`;
}

function emitPortableObjectLiteral(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
): string {
  const propertyContexts = contextsPreservingNamesUsedLater(expression.properties, context);
  const operations = expression.properties.map((property, index) => {
    const propertyContext = propertyContexts[index] ?? context;
    if (property.kind === 'spread') {
      const spreadType = inferIrExpressionType(property.expression, propertyContext);
      const spreadResolved = resolveSemanticType(spreadType, propertyContext) ?? spreadType;
      if (
        !spreadType ||
        !(
          spreadResolved?.kind === 'anonymous' ||
          spreadResolved?.kind === 'array' ||
          spreadResolved?.kind === 'dynamic' ||
          (spreadResolved?.kind === 'named' &&
            (spreadResolved.name === 'RustMap' || Boolean(typedArrayType(spreadResolved.name))))
        )
      ) {
        throw new RustEmissionError(`portable object spread ${String(index)} requires a structural source`);
      }
      const spread = `__flight_spread_${String(index)}`;
      return `let ${spread} = ${emitPortableValueExpression(property.expression, propertyContext, spreadType)}; match ${spread} { crate::FlightValue::Record(entries) => { for (__flight_key, __flight_value) in entries { ${emitPortableRecordUpdate(
        '__flight_key',
        '__flight_value',
      )} } }, crate::FlightValue::Array(values) => { for (__flight_index, __flight_value) in values.into_iter().enumerate() { let __flight_key = __flight_index.to_string(); ${emitPortableRecordUpdate(
        '__flight_key',
        '__flight_value',
      )} } }, crate::FlightValue::Undefined | crate::FlightValue::Null | crate::FlightValue::Bool(_) | crate::FlightValue::Number(_) | crate::FlightValue::Function | crate::FlightValue::Symbol => {}, crate::FlightValue::String(_) => panic!("portable object spread of strings requires UTF-16 property lowering"), crate::FlightValue::Error { .. } | crate::FlightValue::Object => panic!("portable object spread cannot inspect an opaque host object") }`;
    }
    const key =
      property.kind === 'property'
        ? `${emitRustStringLiteral(property.name)}.to_owned()`
        : emitPortablePropertyKey(property.key, propertyContext);
    const valueType = inferIrExpressionType(property.value, propertyContext);
    const keyName = `__flight_key_${String(index)}`;
    const valueName = `__flight_value_${String(index)}`;
    return `let ${keyName} = ${key}; let ${valueName} = ${emitPortableValueExpression(
      property.value,
      propertyContext,
      valueType,
    )}; ${emitPortableRecordUpdate(keyName, valueName)}`;
  });
  return `crate::FlightValue::Record({ let mut __flight_record = Vec::new(); ${operations.join(
    ' ',
  )} __flight_record })`;
}

function emitPortablePropertyKey(expression: IrExpression, context: EmitContext): string {
  const keyType = resolveSemanticType(inferIrExpressionType(expression, context), context);
  if (keyType?.kind !== 'primitive' || keyType.name !== 'String') {
    throw new RustEmissionError('portable computed object keys require a statically typed string');
  }
  return emitExpression(expression, context, primitive('String'));
}

function emitPortableRecordUpdate(key: string, value: string): string {
  return `if let Some((_, __flight_existing)) = __flight_record.iter_mut().find(|(existing, _)| existing == &${key}) { *__flight_existing = ${value}; } else { __flight_record.push((${key}, ${value})); }`;
}

function emitPortableValueFromReference(reference: string, type: IrType, context: EmitContext): string {
  if (type.kind === 'named' && context.enumNames.has(type.name)) {
    return `crate::FlightValue::Number(${parenthesize(reference)}.0 as f64)`;
  }
  const resolved = resolveSemanticType(type, context) ?? type;
  switch (resolved.kind) {
    case 'anonymous': {
      const fields = flattenStructFields(resolved, context).flatMap((field) => {
        if (field.name === '__flight_identity') return [];
        if (field.optional && field.type.kind === 'nullable') {
          throw new RustEmissionError(
            `portable field ${field.name} cannot distinguish an omitted property from explicit null`,
          );
        }
        const key = emitRustStringLiteral(field.name);
        const place = `${parenthesize(reference)}.${safeName(field.name)}`;
        if (field.optional && field.type.kind !== 'nullable') {
          return [
            `if let Some(value) = ${place}.as_ref() { __flight_record.push((${key}.to_owned(), ${emitPortableValueFromReference(
              'value',
              field.type,
              context,
            )})); }`,
          ];
        }
        return [
          `__flight_record.push((${key}.to_owned(), ${emitPortableValueFromReference(
            `&${parenthesize(place)}`,
            field.type,
            context,
          )}));`,
        ];
      });
      return `crate::FlightValue::Record({ let mut __flight_record = Vec::new(); ${fields.join(
        ' ',
      )} __flight_record })`;
    }
    case 'array':
      return `crate::FlightValue::Array(${parenthesize(reference)}.iter().map(|value| ${emitPortableValueFromReference(
        'value',
        resolved.element,
        context,
      )}).collect())`;
    case 'dynamic':
      return `${parenthesize(reference)}.clone()`;
    case 'function':
      return 'crate::FlightValue::Function';
    case 'nullable':
      return `match ${parenthesize(reference)}.as_ref() { Some(value) => ${emitPortableValueFromReference(
        'value',
        resolved.inner,
        context,
      )}, None => crate::FlightValue::Null }`;
    case 'primitive':
      switch (resolved.name) {
        case 'Bool':
          return `crate::FlightValue::Bool(*${parenthesize(reference)})`;
        case 'Float':
        case 'Int':
          return `crate::FlightValue::Number(*${parenthesize(reference)} as f64)`;
        case 'String':
          return `crate::FlightValue::String(${parenthesize(reference)}.clone())`;
        case 'Void':
          return 'crate::FlightValue::Undefined';
      }
    case 'task':
      throw new RustEmissionError('portable value conversion cannot observe an unsettled task');
    case 'union':
      return emitPortableUnionFromReference(reference, resolved.variants, context);
    case 'named': {
      if (resolved.name === 'FlightSymbol') return 'crate::FlightValue::Symbol';
      if (isPortableNumericStorageType(resolved)) {
        return `crate::FlightValue::Number(*${parenthesize(reference)} as f64)`;
      }
      if (resolved.name === 'RustMap') {
        const key = resolveSemanticType(resolved.arguments[0], context) ?? resolved.arguments[0];
        if (key?.kind !== 'primitive' || key.name !== 'String') {
          throw new RustEmissionError('portable record conversion requires string map keys');
        }
        const value = resolved.arguments[1] ?? { kind: 'dynamic' as const };
        return `crate::FlightValue::Record(${parenthesize(reference)}.iter().map(|(key, value)| (key.clone(), ${emitPortableValueFromReference(
          'value',
          value,
          context,
        )})).collect())`;
      }
      if (resolved.name === 'RustSet') return 'crate::FlightValue::Record(Vec::new())';
      if (
        resolved.name === 'ByteBuffer' ||
        resolved.name === 'ArrayBufferView' ||
        Boolean(typedArrayType(resolved.name))
      ) {
        return `crate::FlightValue::Array(${parenthesize(reference)}.iter().map(|value| crate::FlightValue::Number((*value) as f64)).collect())`;
      }
      return 'crate::FlightValue::Object';
    }
  }
}

function emitPortableUnionFromReference(reference: string, variants: readonly IrType[], context: EmitContext): string {
  const [first, ...rest] = variants;
  if (!first) return 'crate::FlightValue::Undefined';
  if (rest.length === 0) return emitPortableValueFromReference(reference, first, context);
  return `match ${reference} { crate::FlightUnion2::A(value) => ${emitPortableValueFromReference(
    'value',
    first,
    context,
  )}, crate::FlightUnion2::B(value) => ${emitPortableUnionFromReference('value', rest, context)} }`;
}

function emitPortableValueToUnion(
  value: string,
  dynamic: Extract<IrType, { kind: 'dynamic' }>,
  variants: readonly IrType[],
  context: EmitContext,
  unionName?: string,
): string | undefined {
  const valuePath = dynamicValuePath(dynamic);
  const arms = variants.map((variant, index) => {
    const resolved = resolveSemanticType(variant, context) ?? variant;
    let pattern: string;
    let converted: string;
    if (resolved.kind === 'primitive' && resolved.name === 'String') {
      pattern = `${valuePath}::String(value)`;
      converted = 'value';
    } else if (resolved.kind === 'primitive' && resolved.name === 'Bool') {
      pattern = `${valuePath}::Bool(value)`;
      converted = 'value';
    } else if (resolved.kind === 'primitive' && (resolved.name === 'Float' || resolved.name === 'Int')) {
      pattern = `${valuePath}::Number(value)`;
      converted = resolved.name === 'Int' ? '(value as i64)' : 'value';
    } else if (portableNumericVectorElement(resolved)) {
      pattern = `${valuePath}::Array(values)`;
      converted = emitPortableNumericVectorValues('values', valuePath, portableNumericVectorElement(resolved)!);
    } else {
      return undefined;
    }
    return `${pattern} => ${wrapUnionValue(converted, variants, index, context, unionName)}`;
  });
  if (arms.some((arm) => arm === undefined)) return undefined;
  return `match ${value} { ${arms.join(', ')}, _ => panic!("TypeScript union cast received an incompatible portable value") }`;
}

function emitPortableNumericVectorCast(value: string, valuePath: string, target: IrType): string | undefined {
  const element = portableNumericVectorElement(target);
  if (!element) return undefined;
  return `match ${value} { ${valuePath}::Array(values) => ${emitPortableNumericVectorValues(
    'values',
    valuePath,
    element,
  )}, _ => panic!("TypeScript typed-array cast received a non-array portable value") }`;
}

function portableNumericVectorElement(type: IrType): string | undefined {
  if (type.kind !== 'named') return undefined;
  if (type.name === 'ByteBuffer' || type.name === 'ArrayBufferView') return 'u8';
  return typedArrayType(type.name)?.rust;
}

function emitPortableNumericVectorValues(values: string, valuePath: string, element: string): string {
  return `${values}.into_iter().map(|value| match value { ${valuePath}::Number(value) => value as ${element}, _ => panic!("TypeScript typed-array cast received a non-numeric element") }).collect::<Vec<_>>()`;
}

function emitMathCall(method: string, arguments_: string[]): string {
  const first = arguments_[0];
  if (!first) throw new RustEmissionError(`Math.${method} requires an argument`);
  switch (method) {
    case 'abs':
    case 'acos':
    case 'asin':
    case 'cbrt':
    case 'ceil':
    case 'cos':
    case 'exp':
    case 'floor':
    case 'ln':
    case 'log':
    case 'log2':
    case 'round':
    case 'sin':
    case 'sqrt':
    case 'tan':
    case 'trunc':
      return `${parenthesize(first)}.${method === 'log' ? 'ln' : method}()`;
    case 'atan2':
      if (!arguments_[1]) throw new RustEmissionError('Math.atan2 requires two arguments');
      return `${parenthesize(first)}.atan2(${arguments_[1]})`;
    case 'hypot':
      return `${parenthesize(arguments_.map((value) => `${parenthesize(value)}.powi(2)`).join(' + '))}.sqrt()`;
    case 'sign':
      return `{ let __flight_value = ${first}; if __flight_value.is_nan() || __flight_value == 0.0 { __flight_value } else { __flight_value.signum() } }`;
    case 'max':
    case 'min':
      return arguments_.slice(1).reduce((value, item) => `${parenthesize(value)}.${method}(${item})`, first);
    case 'imul':
      if (!arguments_[1]) throw new RustEmissionError('Math.imul requires two arguments');
      return `__flight_js_to_i32(${first}).wrapping_mul(__flight_js_to_i32(${arguments_[1]})) as f64`;
    case 'fround':
      return `(${parenthesize(first)} as f32) as f64`;
    case 'pow':
      if (!arguments_[1]) throw new RustEmissionError('Math.pow requires two arguments');
      return `${parenthesize(first)}.powf(${arguments_[1]})`;
    default:
      throw new RustEmissionError(`Math.${method} Rust lowering is not implemented`);
  }
}

function emitProperty(
  expression: Extract<IrExpression, { kind: 'property' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  if (expression.optional) return emitOptionalProperty(expression, context, expectedType);
  const structuralCast = emitStructuralCastProperty(expression, context);
  if (structuralCast !== undefined) return structuralCast;
  const errorProperty = emitNarrowedErrorProperty(expression, context, expectedType);
  if (errorProperty !== undefined) return errorProperty;
  if (expression.object.kind === 'identifier') {
    if (expression.object.name === 'Math' && expression.name === 'PI') return 'std::f64::consts::PI';
    if (expression.object.name === 'Number' && expression.name === 'POSITIVE_INFINITY') return 'f64::INFINITY';
    if (expression.object.name === 'Number' && expression.name === 'NEGATIVE_INFINITY') return 'f64::NEG_INFINITY';
    if (expression.object.name === 'Number' && expression.name === 'NaN') return 'f64::NAN';
    if (expression.object.name === 'Number' && expression.name === 'EPSILON') return 'f64::EPSILON';
    if (expression.object.name === 'Number' && expression.name === 'MAX_VALUE') return 'f64::MAX';
    if (expression.object.name === 'Number' && expression.name === 'MAX_SAFE_INTEGER') {
      return '9007199254740991.0_f64';
    }
    if (expression.object.name === 'Float' && expression.name === 'INFINITY') return 'f64::INFINITY';
    if (expression.object.name === 'Float' && expression.name === 'NAN') return 'f64::NAN';
    if (context.enumNames.has(expression.object.name)) {
      return `${expression.object.name}::${expression.name}`;
    }
  }
  const objectType = inferIrExpressionType(expression.object, context);
  const resolvedObject = resolveSemanticType(objectType, context) ?? objectType;
  const resolvedReceiver = resolvedObject?.kind === 'nullable' ? resolvedObject.inner : resolvedObject;
  const unionProperty = emitUnionPropertyRead(expression, context);
  if (unionProperty !== undefined) return unionProperty;
  const unionOwner =
    expression.object.kind === 'property' ? emitUnionPropertyRead(expression.object, context) : undefined;
  if (unionOwner !== undefined) {
    const type = inferIrExpressionType(expression, context) ?? expectedType;
    const value = `${parenthesize(unionOwner)}.${safeName(expression.name)}`;
    return type && !isCopyType(type, context) ? `${parenthesize(value)}.clone()` : value;
  }
  if (resolvedReceiver?.kind === 'dynamic' || isNativeHostHandleType(resolvedReceiver)) {
    const result = expectedType ?? inferDynamicHostPropertyType(expression.name) ?? { kind: 'dynamic' };
    return emitHostValueExpression(result, emitRustStringLiteral(`host.${expression.name}`), context);
  }
  if (objectType?.kind === 'array' && expression.name === 'length') {
    return `(${emitCollectionPlace(expression.object, context)}.len() as f64)`;
  }
  if (objectType?.kind === 'named' && typedArrayType(objectType.name) && expression.name === 'length') {
    return `(${emitPlaceExpression(expression.object, context)}.len() as f64)`;
  }
  if (resolvedReceiver?.kind === 'primitive' && resolvedReceiver.name === 'String' && expression.name === 'length') {
    const view =
      expression.object.kind === 'identifier' ? context.utf16ViewNames.get(expression.object.name) : undefined;
    if (view) return `(${view}.len() as f64)`;
    const receiver =
      resolvedObject?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
        : isDynamicHostTree(expression.object, context)
          ? emitExpression(expression.object, context, primitive('String'))
          : emitPlaceExpression(expression.object, context);
    return `(${receiver}.encode_utf16().count() as f64)`;
  }
  if (
    objectType?.kind === 'nullable' &&
    (objectType.inner.kind === 'array' ||
      (objectType.inner.kind === 'named' && Boolean(typedArrayType(objectType.inner.name)))) &&
    expression.name === 'length'
  ) {
    return `(${emitPlaceExpression(expression.object, context)}.as_ref().unwrap().len() as f64)`;
  }
  const collectionType = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (
    collectionType?.kind === 'named' &&
    (collectionType.name === 'RustMap' || collectionType.name === 'RustSet') &&
    expression.name === 'size'
  ) {
    const owner =
      objectType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
        : emitCollectionPlace(expression.object, context);
    return `(${owner}.len() as f64)`;
  }
  if (collectionType?.kind === 'named' && collectionType.name === 'RustMap') {
    const keyType = collectionType.arguments[0] ?? primitive('String');
    const key = emitExpression({ kind: 'literal', value: expression.name }, context, keyType);
    const owner =
      objectType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
        : emitPlaceExpression(expression.object, context);
    return `${owner}.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone()).expect("TypeScript Record key was absent")`;
  }
  const type = inferIrExpressionType(expression, context) ?? expectedType;
  const runtimeObjectType = inferIrExpressionType(expression.object, context);
  const runtime = runtimeObjectType?.kind === 'nullable' ? runtimeObjectType.inner : runtimeObjectType;
  if (runtime?.kind === 'named' && context.entityRuntimeTypes.has(runtime.name)) {
    const slot = entityRuntimeFieldSlot(runtime.name, expression.name, context);
    const genericStorage = entityRuntimeGenericSlotStorageType(runtime, slot, context);
    if (genericStorage) {
      const owner =
        runtimeObjectType?.kind === 'nullable'
          ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
          : emitPlaceExpression(expression.object, context);
      const stored = `__flight_storage.${safeName(expression.name)}`;
      const value = context.entityRuntimeLateFields.has(`${slot}\0${expression.name}`)
        ? `${stored}.as_ref().expect("entity runtime field ${expression.name} was read before initialization")`
        : stored;
      const result = type && !isCopyType(type, context) ? `${parenthesize(value)}.clone()` : value;
      return `{ let __flight_slot = ${owner}.__flight_generic_slot::<${genericStorage}>(); let __flight_storage = __flight_slot.lock().unwrap(); ${result} }`;
    }
  }
  const recursiveStorage = recursiveStructPropertyStorage(objectType, expression.name, context);
  if (recursiveStorage) {
    const place = emitPropertyPlace(expression, context);
    return recursiveStorage === 'nullable'
      ? `${parenthesize(place)}.as_deref().cloned()`
      : `${parenthesize(`*${place}`)}.clone()`;
  }
  const place = emitPropertyPlace(expression, context);
  return type && !isCopyType(type, context) ? `${parenthesize(place)}.clone()` : place;
}

function emitUnionPropertyRead(
  expression: Extract<IrExpression, { kind: 'property' }>,
  context: EmitContext,
): string | undefined {
  if (expression.object.kind === 'identifier' && context.unionNarrowings.has(expression.object.name)) {
    return undefined;
  }
  const name = expression.object.kind === 'identifier' ? expression.object.name : undefined;
  const sourceType = inferIrExpressionType(expression.object, context);
  const candidate = sourceType?.kind === 'nullable' ? sourceType.inner : sourceType;
  const union = resolveSemanticType(candidate, context);
  if (union?.kind !== 'union') return undefined;
  const excluded = (name ? context.excludedUnionVariants.get(name) : undefined) ?? new Set<number>();
  const active = union.variants.flatMap((variant, index) => (excluded.has(index) ? [] : [{ index, variant }]));
  const fields = active.map(({ variant }) => {
    const concrete = resolveSemanticType(variant, context);
    return concrete?.kind === 'anonymous'
      ? flattenStructFields(concrete, context).find((field) => field.name === expression.name)
      : undefined;
  });
  const first = fields[0];
  if (!first || fields.some((field) => !field || !semanticTypesEqual(field.type, first.type, context))) {
    return undefined;
  }
  const allowed = new Set(active.map(({ index }) => index));
  const read = (value: string, variants: readonly IrType[], offset: number): string => {
    if (variants.length === 1) {
      return allowed.has(offset)
        ? `${parenthesize(value)}.${safeName(expression.name)}.clone()`
        : 'unreachable!("excluded TypeScript union variant was observed")';
    }
    return `match ${value} { crate::FlightUnion2::A(value) => ${
      allowed.has(offset)
        ? `${parenthesize('value')}.${safeName(expression.name)}.clone()`
        : 'unreachable!("excluded TypeScript union variant was observed")'
    }, crate::FlightUnion2::B(value) => ${read('value', variants.slice(1), offset + 1)} }`;
  };
  const value = emitExpression(expression.object, context, sourceType);
  const reference =
    sourceType?.kind === 'nullable'
      ? `${parenthesize(value)}.as_ref().expect("TypeScript nullable union property was not narrowed")`
      : `&${parenthesize(value)}`;
  return read(reference, union.variants, 0);
}

function emitNarrowedErrorProperty(
  expression: Extract<IrExpression, { kind: 'property' }>,
  context: EmitContext,
  expectedType?: IrType,
): string | undefined {
  if (expression.object.kind !== 'identifier' || !context.errorValueNames.has(expression.object.name)) {
    return undefined;
  }
  if (!errorValuePropertyType(expression.name)) return undefined;
  const owner = emitIdentifier(expression.object.name, context);
  const ownerType = resolveSemanticType(context.symbolTypes.get(expression.object.name), context);
  const valuePath = dynamicValuePath(ownerType);
  const prefix = `match &${parenthesize(owner)} { ${valuePath}::Error {`;
  const suffix = `_ => unreachable!("instanceof Error narrowing must contain an Error value") }`;
  const dynamic = (resolveSemanticType(expectedType, context) ?? expectedType)?.kind === 'dynamic';
  if (expression.name === 'name' || expression.name === 'message') {
    const value = `${prefix} ${expression.name}, .. } => ${expression.name}.clone(), ${suffix}`;
    return dynamic
      ? `${dynamicValuePath(resolveSemanticType(expectedType, context) ?? expectedType)}::String(${value})`
      : value;
  }
  if (expression.name === 'stack') {
    const value = `${prefix} stack, .. } => stack.clone(), ${suffix}`;
    const resultPath = dynamicValuePath(resolveSemanticType(expectedType, context) ?? expectedType);
    return dynamic ? `${parenthesize(value)}.map(${resultPath}::String).unwrap_or(${resultPath}::Undefined)` : value;
  }
  const value = `${prefix} cause, .. } => cause.as_deref().cloned(), ${suffix}`;
  return (resolveSemanticType(expectedType, context) ?? expectedType)?.kind === 'nullable'
    ? value
    : `${parenthesize(value)}.unwrap_or(crate::FlightValue::Undefined)`;
}

function errorValuePropertyType(name: string): IrType | undefined {
  if (name === 'name' || name === 'message') return primitive('String');
  if (name === 'stack') return { inner: primitive('String'), kind: 'nullable' };
  if (name === 'cause') return { kind: 'dynamic', portable: true };
  return undefined;
}

function emitStructuralCastProperty(
  expression: Extract<IrExpression, { kind: 'property' }>,
  context: EmitContext,
): string | undefined {
  if (expression.object.kind !== 'cast') return undefined;
  const target = resolveSemanticType(expression.object.type, context);
  if (target?.kind !== 'anonymous') return undefined;
  const targetField = flattenStructFields(target, context).find((field) => field.name === expression.name);
  if (!targetField?.optional) return undefined;
  const actualType = inferIrExpressionType(expression.object.expression, context);
  const actual = resolveSemanticType(actualType?.kind === 'nullable' ? actualType.inner : actualType, context);
  if (actual?.kind !== 'anonymous') return undefined;
  const actualField = flattenStructFields(actual, context).find((field) => field.name === expression.name);
  if (!actualField) return `None::<${emitType(targetField.type, context)}>`;
  const projected: Extract<IrExpression, { kind: 'property' }> = {
    kind: 'property',
    name: expression.name,
    object: expression.object.expression,
    optional: false,
  };
  const value = emitExpression(projected, context, actualField.type);
  return actualField.optional && actualField.type.kind !== 'nullable' ? value : `Some(${value})`;
}

function emitCollectionPlace(expression: IrExpression, context: EmitContext): string {
  if (expression.kind === 'identifier' && context.mutexCollectionNames.has(expression.name)) {
    const name = context.constantNames.get(expression.name) ?? screamingSnakeCase(expression.name);
    return `${name}.lock().unwrap()`;
  }
  return emitPlaceExpression(expression, context);
}

function emitPropertyPlace(expression: Extract<IrExpression, { kind: 'property' }>, context: EmitContext): string {
  if (expression.object.kind === 'identifier' && context.enumNames.has(expression.object.name)) {
    return `${expression.object.name}::${expression.name}`;
  }
  if (expression.object.kind === 'identifier') {
    const narrowing = context.unionNarrowings.get(expression.object.name);
    if (narrowing) {
      const identifier = emitIdentifier(expression.object.name, context);
      const owned = context.borrowedNames.has(expression.object.name)
        ? `${parenthesize(`*${identifier}`)}.clone()`
        : `${parenthesize(identifier)}.clone()`;
      return `${parenthesize(
        unwrapUnionValue(owned, narrowing.variants, narrowing.index, narrowing.unionName),
      )}.${safeName(expression.name)}`;
    }
  }
  const objectType = inferIrExpressionType(expression.object, context);
  if (objectType && isSharedHandleType(objectType.kind === 'nullable' ? objectType.inner : objectType, context)) {
    const owner =
      objectType.kind === 'nullable'
        ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
        : emitPlaceExpression(expression.object, context);
    const runtime = objectType.kind === 'nullable' ? objectType.inner : objectType;
    if (runtime.kind === 'named' && context.entityRuntimeTypes.has(runtime.name)) {
      const slot = entityRuntimeFieldSlot(runtime.name, expression.name, context);
      const genericStorage = entityRuntimeGenericSlotStorageType(runtime, slot, context);
      if (genericStorage) {
        const fieldType = inferPropertyType(runtime, expression.name, context);
        if (!fieldType) {
          throw new RustEmissionError(`generic entity runtime property ${expression.name} has no inferred field type`);
        }
        const stored = `__flight_storage.${safeName(expression.name)}`;
        const value = context.entityRuntimeLateFields.has(`${slot}\0${expression.name}`)
          ? `${stored}.as_ref().expect("entity runtime field ${expression.name} was read before initialization")`
          : stored;
        const result = isCopyType(fieldType, context) ? value : `${parenthesize(value)}.clone()`;
        return `{ let __flight_slot = ${owner}.__flight_generic_slot::<${genericStorage}>(); let __flight_storage = __flight_slot.lock().unwrap(); ${result} }`;
      }
      const place = entityRuntimeStorageField(`${owner}.inner.lock().unwrap()`, slot, expression.name);
      if (context.entityRuntimeLateFields.has(`${slot}\0${expression.name}`)) {
        return `(*${place}.as_mut().expect("entity runtime field ${expression.name} was read before initialization"))`;
      }
      return place;
    }
    return `${owner}.inner.lock().unwrap().${safeName(expression.name)}`;
  }
  if (objectType?.kind === 'nullable') {
    const root = expressionRootIdentifier(expression.object);
    const accessor = root && context.mutatedNames.has(root) ? 'as_mut' : 'as_ref';
    return `${emitPlaceExpression(expression.object, context)}.${accessor}().unwrap().${safeName(expression.name)}`;
  }
  return `${emitPlaceExpression(expression.object, context)}.${safeName(expression.name)}`;
}

function emitOptionalProperty(
  expression: Extract<IrExpression, { kind: 'property' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const root = expressionRootIdentifier(expression.object);
  if (root && context.knownNullNames.has(root)) {
    const result = expectedType ?? inferIrExpressionType(expression, context);
    const inner = result?.kind === 'nullable' ? result.inner : (result ?? { kind: 'dynamic' });
    return `None::<${emitType(inner, context)}>`;
  }
  const objectType = inferIrExpressionType(expression.object, context);
  if (objectType?.kind !== 'nullable') {
    return emitProperty({ ...expression, optional: false }, context);
  }
  const inner = resolveSemanticType(objectType.inner, context) ?? objectType.inner;
  if (inner.kind === 'dynamic' || isNativeHostHandleType(inner)) {
    const result =
      expectedType?.kind === 'nullable' ? expectedType : ({ inner: expectedType ?? inner, kind: 'nullable' } as const);
    return emitHostValueExpression(result, emitRustStringLiteral(`host.${expression.name}`), context);
  }
  const owner = emitPlaceExpression(expression.object, context);
  if (
    expression.name === 'length' &&
    (inner.kind === 'array' || (inner.kind === 'named' && Boolean(typedArrayType(inner.name))))
  ) {
    return `${owner}.as_ref().map(|value| value.len() as f64)`;
  }
  if (inner.kind === 'named' && inner.name === 'RustMap') {
    const keyType = inner.arguments[0] ?? primitive('String');
    const key = emitExpression({ kind: 'literal', value: expression.name }, context, keyType);
    return `${owner}.as_ref().and_then(|entries| entries.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone()))`;
  }
  if (isSharedHandleType(objectType.inner, context)) {
    const fieldType = inferPropertyType(objectType.inner, expression.name, context);
    if (!fieldType) {
      throw new RustEmissionError(`optional shared property ${expression.name} has no inferred receiver field`);
    }
    const runtimeSlot =
      objectType.inner.kind === 'named' && context.entityRuntimeTypes.has(objectType.inner.name)
        ? entityRuntimeFieldSlot(objectType.inner.name, expression.name, context)
        : undefined;
    const genericStorage =
      runtimeSlot && objectType.inner.kind === 'named'
        ? entityRuntimeGenericSlotStorageType(objectType.inner, runtimeSlot, context)
        : undefined;
    if (runtimeSlot && genericStorage) {
      const stored = `__flight_storage.${safeName(expression.name)}`;
      const value = context.entityRuntimeLateFields.has(`${runtimeSlot}\0${expression.name}`)
        ? `${stored}.as_ref().expect("entity runtime field ${expression.name} was read before initialization")`
        : stored;
      const body = isCopyType(fieldType, context) ? value : `${parenthesize(value)}.clone()`;
      return `${owner}.as_ref().map(|value| { let __flight_slot = value.__flight_generic_slot::<${genericStorage}>(); let __flight_storage = __flight_slot.lock().unwrap(); ${body} })`;
    }
    const storedValue = runtimeSlot
      ? entityRuntimeStorageField('value.inner.lock().unwrap()', runtimeSlot, expression.name)
      : `value.inner.lock().unwrap().${safeName(expression.name)}`;
    const value =
      runtimeSlot && context.entityRuntimeLateFields.has(`${runtimeSlot}\0${expression.name}`)
        ? `${storedValue}.as_ref().expect("entity runtime field ${expression.name} was read before initialization")`
        : storedValue;
    const body = isCopyType(fieldType, context) ? value : `${parenthesize(value)}.clone()`;
    return `${owner}.as_ref().map(|value| ${body})`;
  }
  const fieldType = inferPropertyType(objectType.inner, expression.name, context);
  if (!fieldType) {
    throw new RustEmissionError(`optional property ${expression.name} has no inferred receiver field`);
  }
  const value = `value.${safeName(expression.name)}`;
  const access = fieldType.kind === 'nullable' ? 'and_then' : 'map';
  const body = isCopyType(fieldType.kind === 'nullable' ? fieldType.inner : fieldType, context)
    ? value
    : `${parenthesize(value)}.clone()`;
  return `${owner}.as_ref().${access}(|value| ${body})`;
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

function isRustPlaceExpression(expression: IrExpression): boolean {
  if (expression.kind === 'cast') return isRustPlaceExpression(expression.expression);
  return expression.kind === 'identifier' || expression.kind === 'property' || expression.kind === 'element';
}

function emitUtf16CodePointAt(units: string, index: string): string {
  return [
    '{',
    `let __flight_units: &[u16] = ${units};`,
    `let __flight_raw_index = ${index};`,
    'let __flight_index = if __flight_raw_index.is_nan() { 0_i64 } else if __flight_raw_index.is_finite() { __flight_raw_index.trunc() as i64 } else { -1_i64 };',
    'if __flight_index < 0 { f64::NAN } else if let Some(&__flight_first) = __flight_units.get(__flight_index as usize) {',
    'let __flight_first = u32::from(__flight_first);',
    'if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {',
    'if let Some(&__flight_second) = __flight_units.get(__flight_index as usize + 1) {',
    'let __flight_second = u32::from(__flight_second);',
    'if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) { (((__flight_first - 0xD800_u32) << 10) + (__flight_second - 0xDC00_u32) + 0x10000_u32) as f64 } else { __flight_first as f64 }',
    '} else { __flight_first as f64 }',
    '} else { __flight_first as f64 }',
    '} else { f64::NAN }',
    '}',
  ].join(' ');
}

function emitUtf16CharCodeAt(units: string, index: string): string {
  return [
    '{',
    `let __flight_units: &[u16] = ${units};`,
    `let __flight_raw_index = ${index};`,
    'let __flight_index = if __flight_raw_index.is_nan() { 0_i64 } else if __flight_raw_index.is_finite() { __flight_raw_index.trunc() as i64 } else { -1_i64 };',
    'if __flight_index < 0 { f64::NAN } else { __flight_units.get(__flight_index as usize).map_or(f64::NAN, |unit| f64::from(*unit)) }',
    '}',
  ].join(' ');
}

function emitBinary(expression: Extract<IrExpression, { kind: 'binary' }>, context: EmitContext): string {
  if (
    expression.operator === 'in' &&
    expression.left.kind === 'identifier' &&
    expression.left.name === 'EntityRuntimeKey'
  ) {
    if (isNativeEntityObject(expression.right, context)) {
      return `${entityRuntimeSlot(expression.right, context)}.lock().unwrap().is_some()`;
    }
    rejectEntityRuntimeStorage();
  }
  const runtimeNullComparison =
    isNullishExpression(expression.right) &&
    expression.left.kind === 'element' &&
    isNativeEntityRuntimeAccess(expression.left, context)
      ? expression.left
      : isNullishExpression(expression.left) &&
          expression.right.kind === 'element' &&
          isNativeEntityRuntimeAccess(expression.right, context)
        ? expression.right
        : undefined;
  if (runtimeNullComparison && ['===', '!==', '==', '!='].includes(expression.operator)) {
    const absent = `${entityRuntimeSlot(runtimeNullComparison.object, context)}.lock().unwrap().is_none()`;
    return expression.operator === '===' || expression.operator === '==' ? absent : `!${parenthesize(absent)}`;
  }
  const rightContext =
    expression.operator === '&&'
      ? narrowTypeofContexts(expression.left, context).whenTrue
      : expression.operator === '||'
        ? narrowTypeofContexts(expression.left, context).whenFalse
        : context;
  const leftType = inferIrExpressionType(expression.left, context);
  const rightType = inferIrExpressionType(expression.right, rightContext);
  const resolvedLeft = resolveSemanticType(leftType, context) ?? leftType;
  const resolvedRight = resolveSemanticType(rightType, context) ?? rightType;
  const discriminant = discriminatedUnionComparison(expression, context);
  if (discriminant) {
    const value = emitPlaceExpression({ kind: 'identifier', name: discriminant.name }, context);
    const matches = `matches!(&${parenthesize(value)}, ${unionVariantPattern(
      discriminant.variants,
      discriminant.matchingIndex,
      discriminant.unionName,
    )})`;
    return discriminant.positive ? matches : `!${parenthesize(matches)}`;
  }
  const comparison = ['===', '!==', '==', '!=', '<', '<=', '>', '>='].includes(expression.operator);
  const arithmetic = ['+', '-', '*', '/', '%', '**'].includes(expression.operator);
  const stringOrdering =
    ['<', '<=', '>', '>='].includes(expression.operator) &&
    resolvedLeft?.kind === 'primitive' &&
    resolvedLeft.name === 'String' &&
    resolvedRight?.kind === 'primitive' &&
    resolvedRight.name === 'String';
  const arraySlot =
    isNullishExpression(expression.right) && expression.left.kind === 'element'
      ? expression.left
      : isNullishExpression(expression.left) && expression.right.kind === 'element'
        ? expression.right
        : undefined;
  if (
    arraySlot &&
    ['===', '!==', '==', '!='].includes(expression.operator) &&
    inferIrExpressionType(arraySlot.object, context)?.kind === 'array'
  ) {
    const owner = emitPlaceExpression(arraySlot.object, context);
    const index = emitExpression(arraySlot.index, context);
    const absent = `${owner}.get(${parenthesize(index)} as usize).is_none()`;
    return expression.operator === '===' || expression.operator === '==' ? absent : `!${parenthesize(absent)}`;
  }
  if (arraySlot && ['===', '!==', '==', '!='].includes(expression.operator)) {
    const objectType = inferIrExpressionType(arraySlot.object, context);
    const collectionType = objectType?.kind === 'nullable' ? objectType.inner : objectType;
    const storedType =
      collectionType?.kind === 'named' && collectionType.name === 'RustMap' ? collectionType.arguments[1] : undefined;
    const valueType = resolveSemanticType(storedType, context) ?? storedType;
    if (storedType && valueType?.kind === 'dynamic' && valueType.portable) {
      const lookup = emitExpression(arraySlot, context, {
        inner: storedType,
        kind: 'nullable',
      });
      const nullish = arraySlot === expression.left ? expression.right : expression.left;
      const valuePath = dynamicValuePath(valueType);
      const loose = expression.operator === '==' || expression.operator === '!=';
      const matched = loose
        ? `${parenthesize(lookup)}.as_ref().map_or(true, |value| matches!(value, ${valuePath}::Null | ${valuePath}::Undefined))`
        : nullish.kind === 'literal'
          ? `${parenthesize(lookup)}.as_ref().is_some_and(|value| matches!(value, ${valuePath}::Null))`
          : `${parenthesize(lookup)}.as_ref().map_or(true, |value| matches!(value, ${valuePath}::Undefined))`;
      return expression.operator === '===' || expression.operator === '==' ? matched : `!${parenthesize(matched)}`;
    }
  }
  const nullishContext =
    (expression.operator === '??' || expression.operator === '??undefined') &&
    resolvedLeft?.kind === 'dynamic' &&
    rightType
      ? ({ inner: rightType, kind: 'nullable' } as const)
      : undefined;
  const nullComparisonContext =
    comparison && isNullishExpression(expression.right) && resolvedLeft?.kind === 'dynamic'
      ? ({ inner: { kind: 'dynamic' }, kind: 'nullable' } as const)
      : undefined;
  const left = emitExpression(
    expression.left,
    context,
    nullComparisonContext ??
      nullishContext ??
      (stringOrdering
        ? rightType
        : (comparison || arithmetic) &&
            (resolvedLeft?.kind === 'dynamic' || !resolvedLeft) &&
            resolvedRight?.kind === 'primitive'
          ? rightType
          : undefined),
  );
  const narrowedErrorCause =
    expression.left.kind === 'property' &&
    expression.left.name === 'cause' &&
    expression.left.object.kind === 'identifier' &&
    context.errorValueNames.has(expression.left.object.name);
  if (
    !narrowedErrorCause &&
    isNullishExpression(expression.right) &&
    ['===', '!==', '==', '!='].includes(expression.operator) &&
    resolvedLeft?.kind === 'dynamic' &&
    resolvedLeft.portable
  ) {
    const valuePath = dynamicValuePath(resolvedLeft);
    const loose = expression.operator === '==' || expression.operator === '!=';
    const pattern = loose
      ? `${valuePath}::Null | ${valuePath}::Undefined`
      : expression.right.kind === 'literal'
        ? `${valuePath}::Null`
        : `${valuePath}::Undefined`;
    const matches = `matches!(&${parenthesize(left)}, ${pattern})`;
    return expression.operator === '===' || expression.operator === '==' ? matches : `!${parenthesize(matches)}`;
  }
  if (expression.operator === 'instanceof') {
    const constructor = runtimeConstructorType(expression.right);
    const candidate = leftType?.kind === 'nullable' ? leftType.inner : leftType;
    if (constructor === 'Promise') {
      return candidate?.kind === 'task'
        ? leftType?.kind === 'nullable'
          ? `${parenthesize(left)}.is_some()`
          : 'true'
        : 'false';
    }
    if (constructor && typedArrayType(constructor)) {
      const representedAsConstructor =
        candidate?.kind === 'named' && typedArrayType(candidate.name)?.rust === typedArrayType(constructor)?.rust;
      return representedAsConstructor
        ? leftType?.kind === 'nullable'
          ? `${parenthesize(left)}.is_some()`
          : 'true'
        : 'false';
    }
    if (constructor === 'Error') {
      const valuePath = dynamicValuePath(candidate);
      return candidate?.kind === 'dynamic' ? `matches!(&${parenthesize(left)}, ${valuePath}::Error { .. })` : 'false';
    }
    if (constructor && opaqueHostInstanceConstructors.has(constructor)) return 'false';
    throw new RustEmissionError('instanceof Rust lowering requires a portable typed-array constructor');
  }
  if (expression.operator === 'in') {
    const rightType = resolveSemanticType(inferIrExpressionType(expression.right, rightContext), rightContext);
    const rightReceiver = rightType?.kind === 'nullable' ? rightType.inner : rightType;
    if (rightReceiver?.kind === 'named' && rightReceiver.name === 'RustMap') {
      const keyType = rightReceiver.arguments[0] ?? { kind: 'dynamic' };
      const key = emitExpression(expression.left, context, keyType);
      const place = emitPlaceExpression(expression.right, rightContext);
      const entries = rightType?.kind === 'nullable' ? `${place}.as_ref().unwrap()` : place;
      return `{ let __flight_key = ${key}; ${entries}.iter().any(|(key, _)| key == &__flight_key) }`;
    }
    if (rightReceiver?.kind === 'dynamic' && rightReceiver.portable) {
      const key = emitExpression(expression.left, context, primitive('String'));
      const value = emitExpression(expression.right, rightContext, rightType);
      const valuePath = dynamicValuePath(rightReceiver);
      return `{ let __flight_key = ${key}; matches!(&${parenthesize(value)}, ${valuePath}::Record(entries) if entries.iter().any(|(key, _)| key == &__flight_key)) }`;
    }
    if (expression.left.kind === 'literal' && typeof expression.left.value === 'string') {
      const propertyName = expression.left.value;
      if (rightType?.kind === 'anonymous') {
        return flattenStructFields(rightType, context).some((field) => field.name === propertyName) ? 'true' : 'false';
      }
      if (rightReceiver?.kind === 'dynamic' || !rightType) return 'false';
    }
    throw new RustEmissionError('in-operator requires a static property name or an opaque host receiver');
  }
  if (
    (expression.operator === '??' || expression.operator === '??undefined') &&
    isNullishExpression(expression.right)
  ) {
    return left;
  }
  if (
    ['===', '!==', '==', '!='].includes(expression.operator) &&
    leftType?.kind === 'nullable' &&
    rightType &&
    typeKey(leftType.inner) === typeKey(rightType)
  ) {
    const equality = expression.operator === '===' || expression.operator === '==';
    const comparison = `${parenthesize(left)} == Some(${emitExpression(expression.right, context, leftType.inner)})`;
    return equality ? comparison : `!${parenthesize(comparison)}`;
  }
  const equality =
    expression.operator === '===' || expression.operator === '=='
      ? true
      : expression.operator === '!==' || expression.operator === '!='
        ? false
        : undefined;
  if (
    equality !== undefined &&
    rightType &&
    isStringRepresentedType(leftType, context) &&
    isPlainStringType(rightType, context)
  ) {
    const comparedLeft =
      leftType?.kind === 'nullable'
        ? `${parenthesize(left)}.as_ref().map(|value| value.to_string())`
        : `Some(${parenthesize(left)}.to_string())`;
    const comparison = `${comparedLeft} == Some(${emitExpression(expression.right, context, primitive('String'))})`;
    return equality ? comparison : `!${parenthesize(comparison)}`;
  }
  if (
    equality !== undefined &&
    leftType &&
    isPlainStringType(leftType, context) &&
    isStringRepresentedType(rightType, context)
  ) {
    const emittedRight = emitExpression(expression.right, context);
    const comparedRight =
      rightType?.kind === 'nullable'
        ? `${parenthesize(emittedRight)}.as_ref().map(|value| value.to_string())`
        : `Some(${parenthesize(emittedRight)}.to_string())`;
    const comparison = `Some(${emitExpression(expression.left, context, primitive('String'))}) == ${comparedRight}`;
    return equality ? comparison : `!${parenthesize(comparison)}`;
  }
  const right = emitExpression(
    expression.right,
    rightContext,
    expression.operator === '??' || expression.operator === '??undefined'
      ? leftType?.kind === 'nullable'
        ? rightType?.kind === 'nullable'
          ? rightType
          : leftType.inner
        : resolvedLeft?.kind === 'dynamic'
          ? rightType
          : leftType
      : comparison && resolvedRight?.kind === 'dynamic' && resolvedLeft?.kind !== 'dynamic'
        ? leftType
        : stringOrdering
          ? leftType
          : arithmetic && (resolvedRight?.kind === 'dynamic' || !resolvedRight) && resolvedLeft?.kind === 'primitive'
            ? leftType
            : undefined,
  );
  if (
    expression.operator === '+' &&
    resolvedLeft?.kind === 'primitive' &&
    resolvedLeft.name === 'String' &&
    resolvedRight?.kind === 'primitive' &&
    resolvedRight.name === 'String'
  ) {
    return `format!("{}{}", ${left}, ${right})`;
  }
  const nullableNumericLeft =
    leftType?.kind === 'nullable' ? (resolveSemanticType(leftType.inner, context) ?? leftType.inner) : undefined;
  if (
    ['<', '<=', '>', '>='].includes(expression.operator) &&
    nullableNumericLeft?.kind === 'primitive' &&
    (nullableNumericLeft.name === 'Float' || nullableNumericLeft.name === 'Int')
  ) {
    return `${parenthesize(left)}.as_ref().is_some_and(|value| *value ${expression.operator} ${right})`;
  }
  const nullableCopyRight =
    rightType?.kind === 'nullable' ? (resolveSemanticType(rightType.inner, context) ?? rightType.inner) : undefined;
  if (
    ['<', '<=', '>', '>='].includes(expression.operator) &&
    leftType &&
    rightType?.kind === 'nullable' &&
    nullableCopyRight &&
    isCopyType(rightType.inner, context) &&
    semanticTypesEqual(leftType, rightType.inner, context)
  ) {
    return `${parenthesize(right)}.as_ref().is_some_and(|value| ${left} ${expression.operator} *value)`;
  }
  const callbackTypeParameter =
    inferCallbackTypeParameter(expression.left, context) ?? inferCallbackTypeParameter(expression.right, context);
  if (callbackTypeParameter && ['===', '!==', '==', '!='].includes(expression.operator)) {
    const equality = expression.operator === '===' || expression.operator === '==';
    const same = `crate::FlightCallback::flight_same(&${parenthesize(left)}, &${parenthesize(right)})`;
    return equality ? same : `!${parenthesize(same)}`;
  }
  if (isNullishExpression(expression.right) && ['===', '!==', '==', '!='].includes(expression.operator)) {
    return `${parenthesize(left)}.${expression.operator === '===' || expression.operator === '==' ? 'is_none' : 'is_some'}()`;
  }
  if (isNullishExpression(expression.left) && ['===', '!==', '==', '!='].includes(expression.operator)) {
    return `${parenthesize(right)}.${expression.operator === '===' || expression.operator === '==' ? 'is_none' : 'is_some'}()`;
  }
  if (expression.operator === '**') return `${parenthesize(left)}.powf(${right})`;
  if (expression.operator === '>>>') {
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '>>' || expression.operator === '<<') {
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '&' || expression.operator === '|' || expression.operator === '^') {
    const leftType = inferIrExpressionType(expression.left, context);
    if (leftType?.kind === 'named' && context.enumNames.has(leftType.name)) {
      return `(${left} ${expression.operator} ${right})`;
    }
    return emitBitwiseOperation(left, right, expression.operator);
  }
  if (expression.operator === '??' || expression.operator === '??undefined') {
    if (leftType?.kind !== 'nullable' && resolvedLeft?.kind !== 'dynamic') return left;
    if (leftType?.kind === 'nullable' && rightType?.kind === 'nullable') {
      return `${parenthesize(left)}.clone().or(${right})`;
    }
    return `${parenthesize(left)}.clone().unwrap_or(${right})`;
  }
  if (
    (expression.operator === '&&' || expression.operator === '||') &&
    leftType?.kind === 'nullable' &&
    inferIrExpressionType(expression.right, rightContext)?.kind === 'primitive' &&
    (inferIrExpressionType(expression.right, rightContext) as Extract<IrType, { kind: 'primitive' }>).name === 'Bool'
  ) {
    return `${parenthesize(left)}.is_some() ${expression.operator} ${right}`;
  }
  if (
    (expression.operator === '&&' || expression.operator === '||') &&
    resolvedLeft?.kind === 'primitive' &&
    resolvedLeft.name === 'Bool'
  ) {
    return `${parenthesize(emitCondition(expression.left, context))} ${expression.operator} ${parenthesize(
      emitCondition(expression.right, rightContext),
    )}`;
  }
  if (
    (expression.operator === '&&' || expression.operator === '||') &&
    ((resolvedLeft?.kind === 'primitive' && resolvedLeft.name !== 'Bool') ||
      (!resolvedLeft &&
        resolvedRight?.kind === 'primitive' &&
        (resolvedRight.name === 'Float' || resolvedRight.name === 'Int')))
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
  const entityRuntimeAssignment = emitEntityRuntimeAssignment(expression, context, true);
  if (entityRuntimeAssignment) return entityRuntimeAssignment;
  const runtimeFieldAssignment = emitEntityRuntimeFieldAssignment(expression, context, true);
  if (runtimeFieldAssignment) return runtimeFieldAssignment;
  if (isErasedEntityRuntimeAccess(expression.left)) rejectEntityRuntimeStorage();
  if (isErasedEntityRuntimeTreeAccess(expression.left) && !isNativeEntityRuntimeTreeAccess(expression.left, context)) {
    rejectEntityRuntimeStorage();
  }
  const bufferViewWrite = emitBufferViewWrite(expression, context, true);
  if (bufferViewWrite) return bufferViewWrite;
  const nullishAssignment = emitNullishAssignment(expression, context, true);
  if (nullishAssignment) return nullishAssignment;
  if (
    expression.left.kind === 'identifier' &&
    context.timerHandleNames.has(expression.left.name) &&
    expression.operator === '=' &&
    expression.right.kind === 'literal' &&
    expression.right.value === 0
  ) {
    const left = emitPlaceExpression(expression.left, context);
    return `{ ${left} = None; ${left}.clone() }`;
  }
  if (expression.left.kind === 'property' && isDynamicHostTree(expression.left.object, context)) {
    return `crate::host_set(${emitRustStringLiteral(`host.${expression.left.name}`)}, ${emitExpression(expression.right, context)})`;
  }
  if (
    expression.left.kind === 'identifier' &&
    context.atomicBoolNames.has(expression.left.name) &&
    expression.operator === '='
  ) {
    const name = context.constantNames.get(expression.left.name) ?? screamingSnakeCase(expression.left.name);
    const right = emitExpression(expression.right, context, primitive('Bool'));
    return `{ ${name}.store(${right}, std::sync::atomic::Ordering::Relaxed); ${right} }`;
  }
  if (isArrayLengthAssignment(expression, context)) {
    const collection = emitCollectionPlace(expression.left.object, context);
    const right = emitExpression(expression.right, context);
    if (expression.operator === '-=') {
      return `{ let __flight_length = ${collection}.len().saturating_sub(${parenthesize(right)} as usize); ${collection}.truncate(__flight_length); __flight_length as f64 }`;
    }
    return expression.right.kind === 'literal' && expression.right.value === 0
      ? `{ ${collection}.clear(); ${right} }`
      : `{ let __flight_length = ${parenthesize(right)} as usize; ${collection}.truncate(__flight_length); __flight_length as f64 }`;
  }
  if (isExtensibleArrayElementAssignment(expression, context)) {
    return emitExtensibleArrayElementAssignment(expression, context, true);
  }
  const mapElementAssignment = emitMapElementAssignment(expression, context, true);
  if (mapElementAssignment) return mapElementAssignment;
  const mapPropertyAssignment = emitMapPropertyAssignment(expression, context, true);
  if (mapPropertyAssignment) return mapPropertyAssignment;
  const placeContext = assignmentPlaceContext(expression.left, context);
  const left =
    expression.left.kind === 'element'
      ? emitElement(expression.left, placeContext)
      : emitPlaceExpression(expression.left, placeContext);
  const leftType = inferIrExpressionType(expression.left, context);
  const rightType = inferIrExpressionType(expression.right, context);
  const emittedRight = emitExpression(expression.right, context, leftType);
  const recursiveRight =
    expression.operator === '=' && expression.left.kind === 'property'
      ? emitRecursiveStructStorageValue(expression.left, expression.right, leftType, rightType, context)
      : undefined;
  const sharedCopy =
    leftType?.kind === 'nullable' &&
    isSharedHandleType(leftType.inner, context) &&
    rightType &&
    isSharedHandleType(rightType, context)
      ? `${parenthesize(emitExpression(expression.right, context, leftType.inner))}.clone()`
      : undefined;
  const right =
    recursiveRight ??
    (leftType?.kind === 'nullable' && rightType?.kind !== 'nullable' && !isNullishExpression(expression.right)
      ? `Some(${sharedCopy ?? emitExpression(expression.right, context, leftType.inner)})`
      : emittedRight);
  const resolvedLeft = resolveSemanticType(leftType, context) ?? leftType;
  const assignment =
    expression.operator === '+=' && resolvedLeft?.kind === 'primitive' && resolvedLeft.name === 'String'
      ? `${left}.push_str(&${parenthesize(right)})`
      : emitAssignmentOperation(left, right, expression.operator);
  return `{ ${assignment}; ${left}.clone() }`;
}

function emitAssignmentStatement(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
): string {
  const entityRuntimeAssignment = emitEntityRuntimeAssignment(expression, context, false);
  if (entityRuntimeAssignment) return entityRuntimeAssignment;
  const runtimeFieldAssignment = emitEntityRuntimeFieldAssignment(expression, context, false);
  if (runtimeFieldAssignment) return runtimeFieldAssignment;
  if (isErasedEntityRuntimeAccess(expression.left)) rejectEntityRuntimeStorage();
  if (isErasedEntityRuntimeTreeAccess(expression.left) && !isNativeEntityRuntimeTreeAccess(expression.left, context)) {
    rejectEntityRuntimeStorage();
  }
  const bufferViewWrite = emitBufferViewWrite(expression, context, false);
  if (bufferViewWrite) return bufferViewWrite;
  const nullishAssignment = emitNullishAssignment(expression, context, false);
  if (nullishAssignment) return nullishAssignment;
  if (
    expression.left.kind === 'identifier' &&
    context.timerHandleNames.has(expression.left.name) &&
    expression.operator === '=' &&
    expression.right.kind === 'literal' &&
    expression.right.value === 0
  ) {
    return `${emitPlaceExpression(expression.left, context)} = None`;
  }
  if (expression.left.kind === 'property' && isDynamicHostTree(expression.left.object, context)) {
    return `crate::host_set(${emitRustStringLiteral(`host.${expression.left.name}`)}, ${emitExpression(expression.right, context)})`;
  }
  if (
    expression.left.kind === 'identifier' &&
    context.atomicBoolNames.has(expression.left.name) &&
    expression.operator === '='
  ) {
    const name = context.constantNames.get(expression.left.name) ?? screamingSnakeCase(expression.left.name);
    return `${name}.store(${emitExpression(expression.right, context, primitive('Bool'))}, std::sync::atomic::Ordering::Relaxed)`;
  }
  if (isArrayLengthAssignment(expression, context)) {
    const collection = emitCollectionPlace(expression.left.object, context);
    if (expression.operator === '-=') {
      return `{ let __flight_length = ${collection}.len().saturating_sub(${parenthesize(emitExpression(expression.right, context))} as usize); ${collection}.truncate(__flight_length); }`;
    }
    return expression.right.kind === 'literal' && expression.right.value === 0
      ? `${collection}.clear()`
      : `${collection}.truncate(${parenthesize(emitExpression(expression.right, context))} as usize)`;
  }
  if (isExtensibleArrayElementAssignment(expression, context)) {
    return emitExtensibleArrayElementAssignment(expression, context, false);
  }
  const mapElementAssignment = emitMapElementAssignment(expression, context, false);
  if (mapElementAssignment) return mapElementAssignment;
  const mapPropertyAssignment = emitMapPropertyAssignment(expression, context, false);
  if (mapPropertyAssignment) return mapPropertyAssignment;
  const placeContext = assignmentPlaceContext(expression.left, context);
  const left =
    expression.left.kind === 'element'
      ? emitElement(expression.left, placeContext)
      : emitPlaceExpression(expression.left, placeContext);
  const leftType = inferIrExpressionType(expression.left, context);
  const rightType = inferIrExpressionType(expression.right, context);
  const emittedRight = emitExpression(expression.right, context, leftType);
  const recursiveRight =
    expression.operator === '=' && expression.left.kind === 'property'
      ? emitRecursiveStructStorageValue(expression.left, expression.right, leftType, rightType, context)
      : undefined;
  const right =
    recursiveRight ??
    (leftType?.kind === 'nullable' && rightType?.kind !== 'nullable' && !isNullishExpression(expression.right)
      ? `Some(${emitExpression(expression.right, context, leftType.inner)})`
      : emittedRight);
  const resolvedLeft = resolveSemanticType(leftType, context) ?? leftType;
  return expression.operator === '+=' && resolvedLeft?.kind === 'primitive' && resolvedLeft.name === 'String'
    ? `${left}.push_str(&${parenthesize(right)})`
    : emitAssignmentOperation(left, right, expression.operator);
}

function assignmentPlaceContext(expression: IrExpression, context: EmitContext): EmitContext {
  if (expression.kind !== 'identifier') return context;
  const root = expressionRootIdentifier(expression);
  if (!root || !context.nonNullableNames.has(root)) return context;
  const nonNullableNames = new Set(context.nonNullableNames);
  nonNullableNames.delete(root);
  return { ...context, nonNullableNames };
}

function emitNullishAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (expression.operator !== '??=') return undefined;
  if (!isRustPlaceExpression(expression.left)) {
    throw new RustEmissionError('nullish assignment requires a Rust place expression');
  }
  const leftType = inferIrExpressionType(expression.left, context);
  const place = emitPlaceExpression(expression.left, assignmentPlaceContext(expression.left, context));
  if (leftType?.kind !== 'nullable') {
    return returnValue ? `${place}.clone()` : '{}';
  }
  const rightType = inferIrExpressionType(expression.right, context);
  const value =
    rightType?.kind === 'nullable'
      ? emitExpression(expression.right, context, leftType)
      : `Some(${emitExpression(expression.right, context, leftType.inner)})`;
  const initialize = `if ${place}.is_none() { ${place} = ${value}; }`;
  if (!returnValue) return initialize;
  const result = rightType?.kind === 'nullable' ? `${place}.clone()` : `${place}.as_ref().unwrap().clone()`;
  return `{ ${initialize} ${result} }`;
}

function emitAssignmentOperation(left: string, right: string, operator: string): string {
  const bitwise = new Set(['&=', '|=', '^=', '<<=', '>>=', '>>>=']);
  return bitwise.has(operator)
    ? `${left} = ${emitBitwiseOperation(left, right, operator.slice(0, -1))}`
    : `${left} ${operator} ${right}`;
}

function emitBufferViewWrite(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (
    expression.operator !== '=' ||
    expression.left.kind !== 'element' ||
    expression.left.object.kind !== 'new' ||
    expression.left.object.callee.kind !== 'identifier'
  ) {
    return undefined;
  }
  const view = typedArrayType(expression.left.object.callee.name);
  const buffer = expression.left.object.arguments[0];
  if (!view || !buffer) return undefined;
  const inferredBuffer = inferIrExpressionType(buffer, context);
  const bufferType = resolveSemanticType(inferredBuffer, context) ?? inferredBuffer;
  if (bufferType?.kind !== 'named' || bufferType.name !== 'ByteBuffer') return undefined;
  const width = typedArrayByteWidth(view.rust);
  const value = emitExpression(expression.right, context, {
    arguments: [],
    kind: 'named',
    name: view.ir,
  });
  const index = emitExpression(expression.left.index, context, primitive('Float'));
  const collection = emitCollectionPlace(buffer, context);
  if (width === 1) {
    return `{ let __flight_index = ${parenthesize(index)} as usize; let __flight_value = ${value}; ${collection}[__flight_index] = __flight_value as u8;${returnValue ? ' __flight_value' : ''} }`;
  }
  return `{ let __flight_index = ${parenthesize(index)} as usize * ${String(width)}_usize; let __flight_value = ${value}; let __flight_bytes = __flight_value.to_ne_bytes(); ${collection}[__flight_index..__flight_index + ${String(width)}_usize].copy_from_slice(&__flight_bytes);${returnValue ? ' __flight_value' : ''} }`;
}

function isArrayLengthAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
): expression is Extract<IrExpression, { kind: 'assignment' }> & {
  left: Extract<IrExpression, { kind: 'property' }>;
} {
  if (
    !['=', '-='].includes(expression.operator) ||
    expression.left.kind !== 'property' ||
    expression.left.name !== 'length'
  ) {
    return false;
  }
  const objectType = inferIrExpressionType(expression.left.object, context);
  return objectType?.kind === 'array' || (objectType?.kind === 'named' && Boolean(typedArrayType(objectType.name)));
}

function isExtensibleArrayElementAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
): expression is Extract<IrExpression, { kind: 'assignment' }> & {
  left: Extract<IrExpression, { kind: 'element' }>;
} {
  return (
    expression.operator === '=' &&
    expression.left.kind === 'element' &&
    inferIrExpressionType(expression.left.object, context)?.kind === 'array'
  );
}

function emitExtensibleArrayElementAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }> & {
    left: Extract<IrExpression, { kind: 'element' }>;
  },
  context: EmitContext,
  returnValue: boolean,
): string {
  const collectionType = inferIrExpressionType(expression.left.object, context);
  const elementType = collectionType?.kind === 'array' ? collectionType.element : undefined;
  const collection = emitCollectionPlace(expression.left.object, context);
  const index = emitExpression(expression.left.index, context);
  const value = emitExpression(expression.right, context, elementType);
  return `{ let __flight_index = ${parenthesize(index)} as usize; let __flight_value = ${value}; if __flight_index == ${collection}.len() { ${collection}.push(__flight_value); } else { ${collection}[__flight_index] = __flight_value; }${returnValue ? ` ${collection}[__flight_index].clone()` : ''} }`;
}

function emitMutablePortableRecordCastArgument(
  expression: IrExpression,
  expectedType: IrType | undefined,
  context: EmitContext,
): string | undefined {
  if (expression.kind !== 'cast') return undefined;
  const target = resolveSemanticType(expression.type, context) ?? expression.type;
  if (
    expectedType?.kind !== 'named' ||
    expectedType.name !== 'RustMap' ||
    target.kind !== 'named' ||
    target.name !== 'RustMap'
  ) {
    return undefined;
  }
  const value = unwrapCasts(expression.expression);
  if (value.kind !== 'element') return undefined;
  const objectType = inferIrExpressionType(value.object, context);
  const collectionType = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (collectionType?.kind !== 'named' || collectionType.name !== 'RustMap') return undefined;
  const storedType = resolveSemanticType(collectionType.arguments[1], context) ?? collectionType.arguments[1];
  if (storedType?.kind !== 'dynamic' || !storedType.portable) return undefined;
  const collection =
    objectType?.kind === 'nullable'
      ? `${emitPlaceExpression(value.object, context)}.as_mut().unwrap()`
      : emitCollectionPlace(value.object, context);
  const key = emitExpression(value.index, context, collectionType.arguments[0] ?? primitive('String'));
  const valuePath = dynamicValuePath(storedType);
  return `{ let __flight_key = ${key}; let __flight_value = ${collection}.iter_mut().find(|(key, _)| key == &__flight_key).map(|(_, value)| value).expect("TypeScript Record key was absent"); match __flight_value { ${valuePath}::Record(entries) => entries, _ => panic!("TypeScript Record cast received a non-record portable value") } }`;
}

function emitMapElementAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (expression.operator !== '=' || expression.left.kind !== 'element') return undefined;
  const objectType = inferIrExpressionType(expression.left.object, context);
  const collectionType = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (collectionType?.kind !== 'named' || collectionType.name !== 'RustMap') return undefined;
  const keyType = collectionType.arguments[0] ?? { kind: 'dynamic' };
  const valueType = collectionType.arguments[1] ?? { kind: 'dynamic' };
  const collection =
    objectType?.kind === 'nullable'
      ? `${emitPlaceExpression(expression.left.object, context)}.as_mut().unwrap()`
      : emitCollectionPlace(expression.left.object, context);
  const key = emitExpression(expression.left.index, context, keyType);
  const value = emitExpression(expression.right, context, valueType);
  const stored = returnValue ? '__flight_value.clone()' : '__flight_value';
  return `{ let __flight_key = ${key}; let __flight_value = ${value}; if let Some((_, value)) = ${collection}.iter_mut().find(|(key, _)| key == &__flight_key) { *value = ${stored}; } else { ${collection}.push((__flight_key, ${stored})); }${returnValue ? ' __flight_value' : ''} }`;
}

function emitMapPropertyAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (expression.left.kind !== 'property') return undefined;
  return emitMapElementAssignment(
    {
      ...expression,
      left: {
        index: { kind: 'literal', value: expression.left.name },
        kind: 'element',
        object: expression.left.object,
      },
    },
    context,
    returnValue,
  );
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
    const runtimeUpdate = emitEntityRuntimeFieldUpdate(expression, context);
    if (runtimeUpdate) return runtimeUpdate;
    const operand = emitPlaceExpression(expression.operand, context);
    const operator = expression.operator === '++' ? '+=' : '-=';
    return `{ ${operand} ${operator} 1.0; ${operand} }`;
  }
  const operandType = inferIrExpressionType(expression.operand, context);
  const resolvedOperandType = resolveSemanticType(operandType, context) ?? operandType;
  const nullableOperandInner =
    operandType?.kind === 'nullable'
      ? (resolveSemanticType(operandType.inner, context) ?? operandType.inner)
      : undefined;
  if (expression.operator === '!' && resolvedOperandType?.kind === 'dynamic') {
    return `!${parenthesize(emitExpression(expression.operand, context, primitive('Bool')))}`;
  }
  if (
    expression.operator === '!' &&
    operandType?.kind === 'nullable' &&
    nullableOperandInner?.kind === 'primitive' &&
    nullableOperandInner.name === 'String'
  ) {
    return `${parenthesize(emitExpression(expression.operand, context))}.as_ref().map_or(true, |value| value.is_empty())`;
  }
  if (expression.operator === '!' && operandType?.kind === 'nullable') {
    return `${parenthesize(emitExpression(expression.operand, context))}.is_none()`;
  }
  if (expression.operator === '!' && operandType?.kind === 'primitive' && operandType.name === 'String') {
    return `${parenthesize(emitExpression(expression.operand, context))}.is_empty()`;
  }
  if (expression.operator === '!' && operandType && isReferenceLike(operandType, context)) return 'false';
  if (
    expression.operator === '!' &&
    ((operandType?.kind === 'primitive' && (operandType.name === 'Float' || operandType.name === 'Int')) ||
      isPortableNumericStorageType(operandType))
  ) {
    return `${parenthesize(emitExpression(expression.operand, context))} == 0.0_f64`;
  }
  if (expression.operator === 'delete' && isErasedEntityRuntimeTreeAccess(expression.operand)) {
    if (
      expression.operand.kind === 'element' &&
      isErasedEntityRuntimeAccess(expression.operand) &&
      isNativeEntityRuntimeAccess(expression.operand, context)
    ) {
      return `${entityRuntimeSlot(expression.operand.object, context)}.lock().unwrap().take().is_some()`;
    }
    rejectEntityRuntimeStorage();
  }
  if (expression.operator === 'typeof') {
    const hostTag = inferHostPropertyTypeofTag(expression.operand, context);
    if (hostTag) return `${emitRustStringLiteral(hostTag)}.to_owned()`;
    const resolved = resolveSemanticType(operandType, context) ?? operandType;
    if (resolved?.kind === 'union') {
      const operand = emitExpression(expression.operand, context);
      return `${parenthesize(emitUnionTypeof(`&${parenthesize(operand)}`, resolved.variants, context))}.to_owned()`;
    }
    if (resolved?.kind === 'nullable') {
      const operand = emitExpression(expression.operand, context);
      const inner = resolveSemanticType(resolved.inner, context) ?? resolved.inner;
      if (inner.kind === 'dynamic' && inner.portable) {
        const valuePath = dynamicValuePath(inner);
        return `${parenthesize(`match ${parenthesize(operand)}.as_ref() { None => "undefined", Some(value) => match value { ${valuePath}::Undefined => "undefined", ${valuePath}::Null | ${valuePath}::Array(_) | ${valuePath}::Record(_) | ${valuePath}::Error { .. } | ${valuePath}::Object => "object", ${valuePath}::Bool(_) => "boolean", ${valuePath}::Number(_) => "number", ${valuePath}::String(_) => "string", ${valuePath}::Function => "function", ${valuePath}::Symbol => "symbol" } }`)}.to_owned()`;
      }
      return `${parenthesize(
        `${parenthesize(operand)}.as_ref().map_or("undefined", |_| ${emitRustStringLiteral(
          typeOfTag(resolved.inner, context),
        )})`,
      )}.to_owned()`;
    }
    if (resolved?.kind === 'primitive') {
      return `${emitRustStringLiteral(
        resolved.name === 'Bool'
          ? 'boolean'
          : resolved.name === 'String'
            ? 'string'
            : resolved.name === 'Void'
              ? 'undefined'
              : 'number',
      )}.to_owned()`;
    }
    if (resolved?.kind === 'dynamic') {
      const operand = emitExpression(expression.operand, context);
      const valuePath = dynamicValuePath(resolved);
      return `${parenthesize(`match &${parenthesize(operand)} { ${valuePath}::Undefined => "undefined", ${valuePath}::Null | ${valuePath}::Array(_) | ${valuePath}::Record(_) | ${valuePath}::Error { .. } | ${valuePath}::Object => "object", ${valuePath}::Bool(_) => "boolean", ${valuePath}::Number(_) => "number", ${valuePath}::String(_) => "string", ${valuePath}::Function => "function", ${valuePath}::Symbol => "symbol" }`)}.to_owned()`;
    }
    if (
      resolved?.kind === 'anonymous' ||
      resolved?.kind === 'array' ||
      resolved?.kind === 'function' ||
      resolved?.kind === 'named'
    ) {
      return `${emitRustStringLiteral(resolved.kind === 'function' ? 'function' : 'object')}.to_owned()`;
    }
    throw new RustEmissionError(`typeof operand has no inferred Rust type: ${JSON.stringify(expression.operand)}`);
  }
  const operand = emitExpression(expression.operand, context);
  if (expression.operator === 'void') return `{ ${operand}; () }`;
  if (expression.operator === 'delete') {
    throw new RustEmissionError('delete Rust lowering is not implemented');
  }
  if (expression.operator === '~') {
    return operandType?.kind === 'named' && context.enumNames.has(operandType.name)
      ? `(!${operand})`
      : `(!__flight_js_to_i32(${operand})) as f64`;
  }
  return `(${expression.operator}${operand})`;
}

function inferHostPropertyTypeofTag(expression: IrExpression, context: EmitContext): string | undefined {
  if (expression.kind !== 'property') return undefined;
  const binding = expression.binding;
  const receiver = resolveSemanticType(inferIrExpressionType(expression.object, context), context);
  if (!binding && receiver?.kind !== 'dynamic') return undefined;
  if (!binding) return 'undefined';
  if (binding === 'DomWindowBackend') {
    if (['devicePixelRatio', 'innerHeight', 'innerWidth', 'screenX', 'screenY'].includes(expression.name)) {
      return 'number';
    }
    if (expression.name === 'isSecureContext') return 'boolean';
    if (['localStorage', 'screen', 'visualViewport'].includes(expression.name)) return 'object';
    if (
      [
        'addEventListener',
        'alert',
        'close',
        'confirm',
        'focus',
        'getScreenDetails',
        'matchMedia',
        'moveTo',
        'open',
        'prompt',
        'removeEventListener',
        'resizeTo',
        'showDirectoryPicker',
        'showOpenFilePicker',
        'showSaveFilePicker',
      ].includes(expression.name)
    ) {
      return 'function';
    }
    throw new RustEmissionError(`typeof window.${expression.name} has no configured host-property tag`);
  }
  if (binding === 'DomDocumentBackend') {
    if (expression.name === 'hidden') return 'boolean';
    if (expression.name === 'title') return 'string';
    if (['body', 'documentElement', 'fonts', 'head', 'pointerLockElement'].includes(expression.name)) {
      return 'object';
    }
    if (
      [
        'addEventListener',
        'createElement',
        'createTextNode',
        'exitFullscreen',
        'exitPointerLock',
        'getElementById',
        'hasFocus',
        'querySelector',
        'removeEventListener',
      ].includes(expression.name)
    ) {
      return 'function';
    }
    throw new RustEmissionError(`typeof document.${expression.name} has no configured host-property tag`);
  }
  if (binding === 'DomNavigatorBackend') {
    if (expression.name === 'maxTouchPoints') return 'number';
    if (['language', 'platform'].includes(expression.name)) return 'string';
    if (
      [
        'clipboard',
        'connection',
        'geolocation',
        'gpu',
        'languages',
        'mediaDevices',
        'mediaSession',
        'permissions',
        'storage',
        'virtualKeyboard',
        'wakeLock',
      ].includes(expression.name)
    ) {
      return 'object';
    }
    if (['getBattery', 'getGamepads', 'share', 'vibrate'].includes(expression.name)) return 'function';
    throw new RustEmissionError(`typeof navigator.${expression.name} has no configured host-property tag`);
  }
  return undefined;
}

function isErasedEntityRuntimeAccess(expression: IrExpression): boolean {
  return (
    expression.kind === 'element' &&
    expression.index.kind === 'identifier' &&
    expression.index.name === 'EntityRuntimeKey'
  );
}

function isErasedEntityRuntimeTreeAccess(expression: IrExpression): boolean {
  if (isErasedEntityRuntimeAccess(expression)) return true;
  return expression.kind === 'property' && isErasedEntityRuntimeTreeAccess(expression.object);
}

function isNativeEntityObject(expression: IrExpression, context: EmitContext): boolean {
  const type = inferIrExpressionType(expression, context);
  return Boolean(type && isNativeEntityType(type, context));
}

function isNativeEntityType(type: IrType, context: EmitContext, visited: ReadonlySet<string> = new Set()): boolean {
  if (type.kind !== 'named') return false;
  if (context.entityTypes.has(type.name) || context.entityTypeParameters.has(type.name)) return true;
  if (visited.has(type.name)) return false;
  const declaration = context.namedTypes.get(type.name);
  return Boolean(
    declaration?.kind === 'named' && isNativeEntityType(declaration, context, new Set([...visited, type.name])),
  );
}

function isNativeEntityRuntimeAccess(expression: IrExpression, context: EmitContext): boolean {
  return (
    expression.kind === 'element' &&
    isErasedEntityRuntimeAccess(expression) &&
    isNativeEntityObject(expression.object, context)
  );
}

function isNativeEntityRuntimeTreeAccess(expression: IrExpression, context: EmitContext): boolean {
  if (isErasedEntityRuntimeAccess(expression)) return isNativeEntityRuntimeAccess(expression, context);
  return expression.kind === 'property' && isNativeEntityRuntimeTreeAccess(expression.object, context);
}

function entityRuntimeSlot(expression: IrExpression, context: EmitContext): string {
  const root = expressionRootIdentifier(expression);
  const source = emitPlaceExpression(expression, context);
  const reference = root && context.borrowedNames.has(root) ? source : `&${parenthesize(source)}`;
  return `${entityTraitTypePath(context)}::__flight_entity_runtime(${reference})`;
}

function emitEntityRuntimeValue(expression: IrExpression, context: EmitContext): string {
  return `({ let __flight_runtime = ${entityRuntimeSlot(expression, context)}.lock().unwrap().clone().expect("entity runtime was read before initialization"); __flight_runtime })`;
}

function emitEntityRuntimeAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (
    expression.operator !== '=' ||
    expression.left.kind !== 'element' ||
    !isErasedEntityRuntimeAccess(expression.left) ||
    !isNativeEntityRuntimeAccess(expression.left, context)
  ) {
    return undefined;
  }
  const slot = entityRuntimeSlot(expression.left.object, context);
  if (isNullishExpression(expression.right)) {
    return returnValue ? `{ *${slot}.lock().unwrap() = None; () }` : `*${slot}.lock().unwrap() = None`;
  }
  const value = emitExpression(expression.right, context, {
    arguments: [],
    kind: 'named',
    name: 'EntityRuntime',
  });
  return returnValue
    ? `{ let __flight_runtime = ${value}; *${slot}.lock().unwrap() = Some(__flight_runtime.clone()); __flight_runtime }`
    : `*${slot}.lock().unwrap() = Some(${value})`;
}

function emitEntityRuntimeFieldAssignment(
  expression: Extract<IrExpression, { kind: 'assignment' }>,
  context: EmitContext,
  returnValue: boolean,
): string | undefined {
  if (expression.left.kind !== 'property') return undefined;
  const objectType = inferIrExpressionType(expression.left.object, context);
  const runtime = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (runtime?.kind !== 'named' || !context.entityRuntimeTypes.has(runtime.name)) {
    return undefined;
  }
  const owner =
    objectType?.kind === 'nullable'
      ? `${emitPlaceExpression(expression.left.object, context)}.as_ref().unwrap()`
      : emitPlaceExpression(expression.left.object, context);
  const fieldType = inferPropertyType(runtime, expression.left.name, context);
  const actualType = inferIrExpressionType(expression.right, context);
  const value =
    fieldType?.kind === 'nullable' && actualType?.kind !== 'nullable' && !isNullishExpression(expression.right)
      ? `Some(${emitExpression(expression.right, context, fieldType.inner)})`
      : emitExpression(expression.right, context, fieldType);
  const slot = entityRuntimeFieldSlot(runtime.name, expression.left.name, context);
  const genericStorage = entityRuntimeGenericSlotStorageType(runtime, slot, context);
  if (genericStorage) {
    const storedField = `__flight_storage.${safeName(expression.left.name)}`;
    const late = context.entityRuntimeLateFields.has(`${slot}\0${expression.left.name}`);
    const field = late
      ? `(*${storedField}.as_mut().expect("entity runtime field ${expression.left.name} was read before initialization"))`
      : storedField;
    const assigned =
      expression.operator === '='
        ? late
          ? `${storedField} = Some(__flight_value.clone())`
          : `${field} = __flight_value.clone()`
        : emitAssignmentOperation(field, '__flight_value', expression.operator);
    const result = returnValue ? ` ${expression.operator === '=' ? '__flight_value' : `${field}.clone()`}` : '';
    return `{ let __flight_runtime = ${parenthesize(owner)}.clone(); let __flight_value = ${value}; let __flight_slot = __flight_runtime.__flight_generic_slot::<${genericStorage}>(); let mut __flight_storage = __flight_slot.lock().unwrap(); ${assigned};${result} }`;
  }
  const storedField = entityRuntimeStorageField('__flight_storage', slot, expression.left.name);
  const late = context.entityRuntimeLateFields.has(`${slot}\0${expression.left.name}`);
  const field = late
    ? `(*${storedField}.as_mut().expect("entity runtime field ${expression.left.name} was read before initialization"))`
    : storedField;
  const assignedField = late ? `${storedField} = Some(__flight_value.clone())` : `${field} = __flight_value.clone()`;
  if (returnValue) {
    if (expression.operator === '=') {
      return `{ let __flight_runtime = ${owner}; let __flight_value = ${value}; { let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); ${assignedField}; } __flight_value }`;
    }
    const assignment = emitAssignmentOperation(field, '__flight_value', expression.operator);
    return `{ let __flight_runtime = ${owner}; let __flight_value = ${value}; let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); ${assignment}; ${field}.clone() }`;
  }
  const assignment =
    expression.operator === '='
      ? late
        ? `${storedField} = Some(__flight_value)`
        : `${field} = __flight_value`
      : emitAssignmentOperation(field, '__flight_value', expression.operator);
  return `{ let __flight_runtime = ${owner}; let __flight_value = ${value}; let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); ${assignment}; }`;
}

function emitEntityRuntimeFieldUpdate(
  expression: Extract<IrExpression, { kind: 'unary' }>,
  context: EmitContext,
): string | undefined {
  if (expression.operand.kind !== 'property') return undefined;
  const objectType = inferIrExpressionType(expression.operand.object, context);
  const runtime = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (runtime?.kind !== 'named' || !context.entityRuntimeTypes.has(runtime.name)) return undefined;
  const owner =
    objectType?.kind === 'nullable'
      ? `${emitPlaceExpression(expression.operand.object, context)}.as_ref().unwrap()`
      : emitPlaceExpression(expression.operand.object, context);
  const slot = entityRuntimeFieldSlot(runtime.name, expression.operand.name, context);
  const genericStorage = entityRuntimeGenericSlotStorageType(runtime, slot, context);
  if (genericStorage) {
    const storedField = `__flight_storage.${safeName(expression.operand.name)}`;
    const field = context.entityRuntimeLateFields.has(`${slot}\0${expression.operand.name}`)
      ? `(*${storedField}.as_mut().expect("entity runtime field ${expression.operand.name} was read before initialization"))`
      : storedField;
    const operator = expression.operator === '++' ? '+=' : '-=';
    return `{ let __flight_runtime = ${parenthesize(owner)}.clone(); let __flight_slot = __flight_runtime.__flight_generic_slot::<${genericStorage}>(); let mut __flight_storage = __flight_slot.lock().unwrap(); ${field} ${operator} 1.0; ${field}.clone() }`;
  }
  const storedField = entityRuntimeStorageField('__flight_storage', slot, expression.operand.name);
  const field = context.entityRuntimeLateFields.has(`${slot}\0${expression.operand.name}`)
    ? `(*${storedField}.as_mut().expect("entity runtime field ${expression.operand.name} was read before initialization"))`
    : storedField;
  const operator = expression.operator === '++' ? '+=' : '-=';
  return `{ let __flight_runtime = ${owner}; let mut __flight_storage = __flight_runtime.inner.lock().unwrap(); ${field} ${operator} 1.0; ${field}.clone() }`;
}

function rejectEntityRuntimeStorage(): never {
  throw new RustEmissionError(
    'EntityRuntimeKey storage requires an aggregate native entity runtime representation; refusing to erase observable runtime state',
  );
}

function emitClosure(
  expression: Extract<IrExpression, { kind: 'function' }>,
  context: EmitContext,
  expectedType?: IrType | undefined,
  wrapFunction = true,
  moveClosure = wrapFunction,
): string {
  rejectPortableTaskExecution(expression.execution);
  const resolvedExpected = resolveSemanticType(expectedType, context) ?? expectedType;
  const callbackTypeParameter =
    expectedType?.kind === 'named' && context.callbackTypeParameters.has(expectedType.name)
      ? expectedType.name
      : undefined;
  registerContextualAnonymousTypes(resolvedExpected, context, expression.name ?? 'closure');
  const callback =
    callbackTypeParameter || (expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name))
      ? true
      : resolvedExpected?.kind === 'function' || wrapFunction;
  const contextualReturn =
    expression.returns &&
    resolvedExpected?.kind === 'function' &&
    structurallyCompatibleTypes(expression.returns, resolvedExpected.returns, context)
      ? resolvedExpected.returns
      : expression.returns;
  const returns =
    contextualReturn ??
    (callbackTypeParameter
      ? primitive('Void')
      : resolvedExpected?.kind === 'function'
        ? resolvedExpected.returns
        : expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
          ? primitive('Float')
          : (inferFunctionExpressionReturnType(expression) ??
            (expression.expression
              ? (inferIrExpressionType(expression.expression, context) ?? primitive('Float'))
              : primitive('Void'))));
  const nextContext = functionContext(context, expression.name ?? 'closure', expression, returns);
  const fallbackParameter =
    resolvedExpected?.kind === 'function'
      ? resolvedExpected.parameters
      : expectedType?.kind === 'named' && context.callbackTypes.has(expectedType.name)
        ? [primitive('Float')]
        : [];
  if (callbackTypeParameter && expression.parameters.length === 1) {
    nextContext.symbolTypes.set(expression.parameters[0]!.name, {
      arguments: [{ arguments: [], kind: 'named', name: callbackTypeParameter }],
      kind: 'named',
      name: 'FlightCallbackArgs',
    });
  } else {
    registerParameters(expression.parameters, nextContext, fallbackParameter);
  }
  registerLocalTypes(expression.body, nextContext);
  const forwardClosureSlots = emitForwardClosureSlotDeclarations(nextContext);
  const declaredParameters =
    callbackTypeParameter && expression.parameters.length === 1
      ? [`${safeName(expression.parameters[0]!.name)}: <${callbackTypeParameter} as crate::FlightCallback>::Args`]
      : expression.parameters.map((parameter, index) =>
          emitParameter(parameter, nextContext, fallbackParameter[index], expression, false),
        );
  const omittedParameters =
    callbackTypeParameter || expression.parameters.at(-1)?.rest
      ? []
      : fallbackParameter
          .slice(expression.parameters.length)
          .map(
            (parameter, index) =>
              `__flight_unused_${String(expression.parameters.length + index)}: ${emitType(parameter, nextContext)}`,
          );
  const parameters = [...declaredParameters, ...omittedParameters];
  const defaults = expression.parameters.flatMap((parameter, index) => {
    if (!parameter.initializer) return [];
    const contextual = fallbackParameter[index];
    if (
      (contextual?.kind === 'nullable' || parameter.type.kind === 'nullable') &&
      isNullishExpression(parameter.initializer)
    ) {
      return [];
    }
    const parameterType =
      contextual?.kind === 'nullable'
        ? contextual.inner
        : parameter.type.kind === 'nullable'
          ? parameter.type.inner
          : parameter.type;
    const name = safeName(parameter.name);
    return [`let ${name} = ${name}.unwrap_or(${emitExpression(parameter.initializer, nextContext, parameterType)});`];
  });
  const closurePrefix = [...forwardClosureSlots, ...defaults];
  const body = expression.expression
    ? `{ ${closurePrefix.length > 0 ? `${closurePrefix.join(' ')} ` : ''}${emitExpression(expression.expression, nextContext, returns)} }`
    : emitStatementsAsBlock(expression.body, nextContext, closurePrefix);
  const closure = `${moveClosure ? 'move ' : ''}|${parameters.join(', ')}| -> ${emitType(returns, nextContext)} ${body}`;
  const anonymousDefinitions = emitAnonymousDefinitions(nextContext);
  const definedClosure = anonymousDefinitions ? `{ ${anonymousDefinitions} ${closure} }` : closure;
  const capturedNames = moveClosure ? collectClonedClosureCaptures(expression, context) : [];
  const utf16Captures = moveClosure
    ? [...context.utf16ViewNames.entries()].flatMap(([name, view]) =>
        usesStringUtf16Access(expression, name) ? [view] : [],
      )
    : [];
  const recursiveSlots = moveClosure
    ? [...context.recursiveClosureSlots.entries()].flatMap(([name, slot]) =>
        containsIdentifier(expression, name) ? [slot] : [],
      )
    : [];
  const capturedClosure =
    capturedNames.length > 0 || utf16Captures.length > 0 || recursiveSlots.length > 0
      ? `{ ${[
          ...recursiveSlots.map((slot) => `let ${slot} = ${slot}.clone();`),
          ...utf16Captures.map((view) => `let ${view} = ${view}.clone();`),
          ...capturedNames.map(
            (name) =>
              `let ${context.mutatedNames.has(name) ? 'mut ' : ''}${safeName(name)} = ${safeName(name)}.clone();`,
          ),
        ].join(' ')} ${definedClosure} }`
      : definedClosure;
  if (callbackTypeParameter) {
    return `${callbackTypeParameter}::flight_from_tuple_callback(${capturedClosure})`;
  }
  if (callback && wrapFunction) {
    const functionType =
      resolvedExpected?.kind === 'function'
        ? resolvedExpected
        : {
            kind: 'function' as const,
            parameters: expression.parameters.map((parameter) => parameter.type),
            returns,
          };
    const erased = `Box<dyn FnMut(${functionType.parameters.map((item) => emitType(item, nextContext)).join(', ')}) -> ${emitType(functionType.returns, nextContext)} + Send + 'static>`;
    return `std::sync::Arc::new(std::sync::Mutex::new(Box::new(${capturedClosure}) as ${erased}))`;
  }
  return capturedClosure;
}

function collectClonedClosureCaptures(
  expression: Extract<IrExpression, { kind: 'function' }>,
  context: EmitContext,
): string[] {
  const localNames = new Set(expression.parameters.map((parameter) => parameter.name));
  const visitLocals = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'function') return;
    if ('kind' in value && value.kind === 'variable' && 'declarations' in value) {
      for (const variable of value.declarations as IrVariable[]) localNames.add(variable.name);
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visitLocals);
      else visitLocals(child);
    }
  };
  visitLocals(expression.body);
  const candidates = [...context.symbolTypes.entries()].flatMap(([name, type]) => {
    if (
      localNames.has(name) ||
      context.constantNames.has(name) ||
      context.recursiveClosureSlots.has(name) ||
      !containsIdentifier(expression, name)
    ) {
      return [];
    }
    const resolved = resolveSemanticType(type, context) ?? type;
    return context.sharedCaptureNames.has(name) ||
      resolved.kind === 'function' ||
      isReferenceLike(type, context) ||
      isSharedHandleType(type, context) ||
      (type.kind === 'named' && context.callbackTypeParameters.has(type.name))
      ? [name]
      : [];
  });
  return [...new Set(candidates)].sort();
}

function emitType(type: IrType, context: EmitContext): string {
  switch (type.kind) {
    case 'anonymous': {
      const name = context.anonymousTypes.get(typeKey(type));
      if (!name) {
        throw new RustEmissionError(`anonymous structural type has no synthesized Rust identity: ${typeKey(type)}`);
      }
      const parameters = anonymousTypeParameterNames(type, context);
      return parameters.length > 0 ? `${name}<${parameters.join(', ')}>` : name;
    }
    case 'array':
      return `Vec<${emitType(type.element, context)}>`;
    case 'dynamic':
      return type.portable ? 'crate::FlightValue' : 'crate::OpaqueHostValue';
    case 'function':
      return `std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(${type.parameters.map((item) => emitType(item, context)).join(', ')}) -> ${emitType(type.returns, context)} + Send + 'static>>>`;
    case 'named': {
      if (type.arguments.length === 0 && context.lexicalTypeParameters.has(type.name)) return type.name;
      if (type.name.startsWith('RustStructural:')) return type.name.slice('RustStructural:'.length);
      if (type.name !== 'EntityRuntime' && context.entityRuntimeTypes.has(type.name)) {
        const parameters = context.namedTypeParameters.get(type.name) ?? [];
        const arguments_ = parameters.map((_parameter, index) => type.arguments[index] ?? primitive('Void'));
        return `${type.name}${
          arguments_.length > 0 ? `<${arguments_.map((argument) => emitType(argument, context)).join(', ')}>` : ''
        }`;
      }
      if (isStructuralUtilityType(type)) {
        const resolved = resolveSemanticType(type, context);
        if (resolved && typeKey(resolved) !== typeKey(type)) return emitType(resolved, context);
      }
      if (type.name === 'FlightCallbackArgs') {
        const callback = type.arguments[0] ?? { kind: 'dynamic' };
        return `<${emitType(callback, context)} as crate::FlightCallback>::Args`;
      }
      if (type.name === 'FlightNever') return 'std::convert::Infallible';
      if (type.name === 'FlightSymbol') return 'crate::FlightSymbol';
      if (type.name === 'FlightTimeout') return 'crate::FlightTimeout';
      if (type.name === 'Date') return 'crate::OpaqueHostValue';
      if (context.numericNamespaceNames.has(type.name)) return 'f64';
      if (nativeHostHandleTypes.has(type.name)) return `crate::${type.name}`;
      if (type.name === 'Nothing') return 'crate::OpaqueHostValue';
      if (type.name === 'FlightRegex') return 'regex::Regex';
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
      if (type.name === 'ByteBuffer' || type.name === 'ArrayBufferView') return 'Vec<u8>';
      if (type.name === 'RustMap') {
        return `Vec<(${emitType(type.arguments[0] ?? { kind: 'dynamic' }, context)}, ${emitType(type.arguments[1] ?? { kind: 'dynamic' }, context)})>`;
      }
      if (type.name === 'RustSet') {
        return `Vec<${emitType(type.arguments[0] ?? { kind: 'dynamic' }, context)}>`;
      }
      if (type.name === 'RustTuple2') {
        return `(${emitType(type.arguments[0] ?? { kind: 'dynamic' }, context)}, ${emitType(type.arguments[1] ?? { kind: 'dynamic' }, context)})`;
      }
      const declarationType = context.namedTypes.get(type.name);
      const parameters = context.namedTypeParameters.get(type.name) ?? [];
      const arguments_ = declarationType
        ? parameters.flatMap((parameter, index) => {
            const used = emittedDeclarationUsesNamedParameter(type.name, declarationType, parameter, context);
            return used ? [type.arguments[index] ?? { kind: 'dynamic' as const }] : [];
          })
        : type.arguments;
      return `${type.name}${arguments_.length > 0 ? `<${arguments_.map((item) => emitType(item, context)).join(', ')}>` : ''}`;
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
    case 'task':
      return `crate::FlightTask<${emitType(type.output, context)}>`;
    case 'union':
      return emitUnionType(type.variants, context);
  }
}

function dynamicValuePath(type: IrType | undefined): 'crate::FlightValue' | 'crate::OpaqueHostValue' {
  return type?.kind === 'dynamic' && type.portable ? 'crate::FlightValue' : 'crate::OpaqueHostValue';
}

function emitStructConstructorType(type: IrType, context: EmitContext): string {
  const emitted = emitType(type, context);
  return emitted.includes('<') ? emitted.replace('<', '::<') : emitted;
}

function emitUnionType(variants: readonly IrType[], context: EmitContext): string {
  const [first, ...rest] = variants;
  if (!first) return 'crate::OpaqueHostValue';
  if (rest.length === 0) return emitType(first, context);
  return `crate::FlightUnion2<${emitType(first, context)}, ${emitUnionType(rest, context)}>`;
}

function unionVariantPattern(variants: readonly IrType[], variantIndex: number, unionName?: string): string {
  const constructor = unionName ?? 'crate::FlightUnion2';
  if (variantIndex === 0) return `${constructor}::A(_)`;
  if (variants.length <= 2) return `${constructor}::B(_)`;
  return `crate::FlightUnion2::B(${unionVariantPattern(variants.slice(1), variantIndex - 1)})`;
}

function emitNamedUnionConstructor(type: Extract<IrType, { kind: 'named' }>, context: EmitContext): string {
  const emitted = emitStructConstructorType(type, context);
  return context.localTypeNames.has(type.name) || context.importedTypeNames.has(type.name)
    ? emitted
    : `flighthq_types::${emitted}`;
}

function wrapUnionValue(
  value: string,
  variants: readonly IrType[],
  variantIndex: number,
  context: EmitContext,
  unionName?: string,
): string {
  if (variants.length <= 1) return value;
  const constructor =
    unionName ??
    `crate::FlightUnion2::<${emitType(variants[0]!, context)}, ${emitUnionType(variants.slice(1), context)}>`;
  if (variantIndex === 0) {
    return `${constructor}::A(${value})`;
  }
  return `${constructor}::B(${wrapUnionValue(value, variants.slice(1), variantIndex - 1, context)})`;
}

function unwrapUnionValue(
  value: string,
  variants: readonly IrType[],
  variantIndex: number,
  unionName?: string,
): string {
  if (variants.length <= 1) return value;
  const constructor = unionName ?? 'crate::FlightUnion2';
  if (variantIndex === 0) {
    return `match ${value} { ${constructor}::A(value) => value, ${constructor}::B(_) => panic!("TypeScript union narrowing failed") }`;
  }
  return `match ${value} { ${constructor}::A(_) => panic!("TypeScript union narrowing failed"), ${constructor}::B(value) => ${unwrapUnionValue(
    'value',
    variants.slice(1),
    variantIndex - 1,
  )} }`;
}

function emitUnionTypeof(value: string, variants: readonly IrType[], context: EmitContext): string {
  const [first, ...rest] = variants;
  if (!first) return '"undefined"';
  if (rest.length === 0) return emitRustStringLiteral(typeOfTag(first, context));
  return `match ${value} { crate::FlightUnion2::A(_) => ${emitRustStringLiteral(typeOfTag(first, context))}, crate::FlightUnion2::B(value) => ${emitUnionTypeof(
    'value',
    rest,
    context,
  )} }`;
}

function typeOfTag(type: IrType, context: EmitContext): string {
  const resolved = resolveSemanticType(type.kind === 'nullable' ? type.inner : type, context);
  if (resolved?.kind === 'primitive') {
    if (resolved.name === 'Bool') return 'boolean';
    if (resolved.name === 'String') return 'string';
    if (resolved.name === 'Void') return 'undefined';
    return 'number';
  }
  return resolved?.kind === 'function' ? 'function' : 'object';
}

function emitTypeDeclaration(
  name: string,
  exported: boolean,
  type: IrType,
  context: EmitContext,
  typeParameters: readonly string[] = [],
): string {
  const visibility = exported ? 'pub ' : 'pub(crate) ';
  if (name !== 'EntityRuntime' && context.entityRuntimeTypes.has(name)) {
    if (context.entityRuntimeClosureError) {
      throw new RustEmissionError(
        `aggregate native entity runtime closure is unavailable: ${context.entityRuntimeClosureError}`,
      );
    }
    const slot =
      (context.entityRuntimeSlotTypes.has(name) || context.entityRuntimeGenericSlotTypes.has(name)) &&
      type.kind === 'anonymous'
        ? emitEntityRuntimeSlotDeclaration(name, exported, type, context, typeParameters)
        : '';
    const marker = entityRuntimeMarkerType(typeParameters);
    const generics = typeParameters.length > 0 ? `<${typeParameters.join(', ')}>` : '';
    const runtime = marker
      ? `<std::marker::PhantomData<${marker}> as ${entityRuntimeMarkerTraitPath(context)}>::Runtime`
      : entityRuntimeTypePath(context);
    return `${slot}${slot ? '\n' : ''}${visibility}type ${name}${generics} = ${runtime};`;
  }
  if (type.kind !== 'anonymous') {
    const aliasContext = {
      ...typeDeclarationContext(context, name, type),
      lexicalTypeParameters: new Set(typeParameters),
    };
    const emittedType = emitType(type, aliasContext);
    const effectiveTypeParameters = typeParameters.filter((parameter) =>
      new RegExp(`\\b${parameter}\\b`, 'u').test(emittedType),
    );
    const generics = effectiveTypeParameters.length > 0 ? `<${effectiveTypeParameters.join(', ')}>` : '';
    return `${emitAnonymousDefinitions(aliasContext, exported, !exported)}${visibility}type ${name}${generics} = ${emittedType};`;
  }
  const structuralContext = {
    ...typeDeclarationContext(context, name, type),
    lexicalTypeParameters: new Set(typeParameters),
  };
  const fields = flattenStructFields(type, structuralContext);
  const effectiveTypeParameters = typeParameters.filter((parameter) =>
    fields.some((field) => emittedTypeUsesNamedParameter(field.type, parameter, structuralContext)),
  );
  const generics = effectiveTypeParameters.length > 0 ? `<${effectiveTypeParameters.join(', ')}>` : '';
  if (context.entityRuntimeTypes.has(name)) {
    if (context.entityRuntimeClosureError) {
      throw new RustEmissionError(
        `aggregate native entity runtime closure is unavailable: ${context.entityRuntimeClosureError}`,
      );
    }
    if (name !== 'EntityRuntime') {
      return `${visibility}type ${name} = ${entityRuntimeTypePath(context)};`;
    }
    const storageName = 'EntityRuntimeStorage';
    const storageFields = fields.map((field) => {
      const fieldType =
        field.optional && field.type.kind !== 'nullable'
          ? `Option<${emitStructFieldType(field.type, name, structuralContext)}>`
          : emitStructFieldType(field.type, name, structuralContext);
      const storageType = context.entityRuntimeLateFields.has(`EntityRuntime\0${field.name}`)
        ? `Option<${fieldType}>`
        : fieldType;
      return `pub ${safeName(field.name)}: ${storageType},`;
    });
    storageFields.push(
      ...[...context.entityRuntimeSlotTypes]
        .sort((left, right) => left.localeCompare(right))
        .map((owner) => `pub ${snakeCase(owner)}: crate::${owner}Storage,`),
    );
    storageFields.push(
      'pub generic_slots: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send>>,',
    );
    return `${emitAnonymousDefinitions(structuralContext, exported, !exported)}${[
      '#[derive(Clone, Default)]',
      `${visibility}struct ${name} {`,
      `  #[doc(hidden)] pub inner: std::sync::Arc<std::sync::Mutex<${storageName}>>,`,
      '}',
      '#[doc(hidden)]',
      '#[derive(Default)]',
      `${visibility}struct ${storageName} {`,
      indent(storageFields.join('\n')),
      '}',
      `impl PartialEq for ${name} {`,
      '  fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.inner, &other.inner) }',
      '}',
      `impl ${name} {`,
      '  #[doc(hidden)]',
      "  pub fn __flight_generic_slot<Slot: Default + Send + 'static>(&self) -> std::sync::Arc<std::sync::Mutex<Slot>> {",
      '    let mut storage = self.inner.lock().unwrap();',
      '    let slot = storage.generic_slots.entry(std::any::TypeId::of::<Slot>()).or_insert_with(|| Box::new(std::sync::Arc::new(std::sync::Mutex::new(Slot::default()))));',
      '    slot.downcast_ref::<std::sync::Arc<std::sync::Mutex<Slot>>>().expect("entity runtime generic slot type identity collision").clone()',
      '  }',
      '}',
      '#[doc(hidden)]',
      `${visibility}trait FlightEntityRuntimeMarker {`,
      '  type Runtime;',
      '}',
      `impl<Marker> FlightEntityRuntimeMarker for std::marker::PhantomData<Marker> {`,
      '  type Runtime = EntityRuntime;',
      '}',
    ].join('\n')}`;
  }
  if (name === 'SignalData') {
    const storageName = 'SignalDataStorage';
    const fieldType = (field: IrTypeField): string =>
      field.optional && field.type.kind !== 'nullable'
        ? `Option<${emitStructFieldType(field.type, name, structuralContext)}>`
        : emitStructFieldType(field.type, name, structuralContext);
    return [
      `${visibility}struct ${name}${generics} {`,
      `  #[doc(hidden)] pub inner: std::sync::Arc<std::sync::Mutex<${storageName}${generics}>>,`,
      '}',
      `impl${generics} Clone for ${name}${generics} {`,
      '  fn clone(&self) -> Self { Self { inner: self.inner.clone() } }',
      '}',
      '#[doc(hidden)]',
      `${visibility}struct ${storageName}${generics} {`,
      indent(fields.map((field) => `pub ${safeName(field.name)}: ${fieldType(field)},`).join('\n')),
      '}',
      `impl${generics} ${name}${generics} {`,
      `  pub fn new(${fields.map((field) => `${safeName(field.name)}: ${fieldType(field)}`).join(', ')}) -> Self {`,
      `    Self { inner: std::sync::Arc::new(std::sync::Mutex::new(${storageName} {`,
      fields.map((field) => `      ${safeName(field.name)},`).join('\n'),
      '    })) }',
      '  }',
      '}',
    ].join('\n');
  }
  const supportsDefault = fields.every(
    (field) => field.optional || rustTypeSupportsDefault(field.type, structuralContext),
  );
  // Rust's built-in derive adds `T: Default` for every generic mentioned by a
  // field, even when the field's container has an unconditional default (for
  // example `Option<T>` or `Vec<T>`). Emit the same field-wise construction by
  // hand so a registry of callbacks can still be default-constructed.
  const derivesDefault = supportsDefault && effectiveTypeParameters.length === 0;
  const implementsUnboundedDefault = supportsDefault && effectiveTypeParameters.length > 0;
  const entity = context.entityTypes.has(name);
  const entityRuntime = entity ? entityRuntimeTypePath(context) : undefined;
  const entityTrait = entity ? entityTraitTypePath(context) : undefined;
  const emitted = [
    `#[derive(Clone${derivesDefault ? ', Default' : ''})]`,
    `${visibility}struct ${name}${generics} {`,
    indent(
      [
        '#[doc(hidden)] pub __flight_identity: std::sync::Arc<()>,',
        ...(entityRuntime
          ? [
              `#[doc(hidden)] pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<${entityRuntime}>>>,`,
              '#[doc(hidden)] pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,',
            ]
          : []),
        ...fields.map(
          (field) =>
            `pub ${safeName(field.name)}: ${
              field.optional
                ? field.type.kind === 'nullable'
                  ? emitStructFieldType(field.type, name, structuralContext)
                  : `Option<${emitStructFieldType(field.type, name, structuralContext)}>`
                : emitStructFieldType(field.type, name, structuralContext)
            },`,
        ),
      ].join('\n'),
    ),
    '}',
    ...(implementsUnboundedDefault
      ? [
          `impl${generics} Default for ${name}${generics} {`,
          '  fn default() -> Self {',
          '    Self {',
          '      __flight_identity: Default::default(),',
          ...(entityRuntime
            ? [
                '      __flight_entity_runtime: Default::default(),',
                '      __flight_entity_snapshot: Default::default(),',
              ]
            : []),
          ...fields.map((field) => `      ${safeName(field.name)}: Default::default(),`),
          '    }',
          '  }',
          '}',
        ]
      : []),
    `impl${generics} PartialEq for ${name}${generics} {`,
    '  fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity) }',
    '}',
  ];
  if (entity && name === 'Entity') {
    emitted.push(
      '#[doc(hidden)]',
      `${visibility}trait FlightEntity: std::any::Any + Send + Sync {`,
      `  fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<${entityRuntime!}>>>;`,
      '  fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>;',
      "  fn __flight_downcast<T: Clone + 'static>(&self) -> Option<T> where Self: Sized {",
      '    if let Some(snapshot) = self.__flight_entity_snapshot() {',
      '      if let Some(value) = snapshot.downcast_ref::<T>() { return Some(value.clone()); }',
      '    }',
      '    (self as &dyn std::any::Any).downcast_ref::<T>().cloned()',
      '  }',
      '  fn __flight_fresh_clone(&self) -> Self where Self: Sized;',
      '}',
    );
  }
  if (entity && entityTrait) {
    const cloneBoundGenerics =
      effectiveTypeParameters.length > 0
        ? `<${effectiveTypeParameters.map((parameter) => `${parameter}: Clone + Send + Sync + 'static`).join(', ')}>`
        : '';
    emitted.push(
      `impl${cloneBoundGenerics} ${entityTrait} for ${name}${generics} {`,
      `  fn __flight_entity_runtime(&self) -> &std::sync::Arc<std::sync::Mutex<Option<${entityRuntime!}>>> { &self.__flight_entity_runtime }`,
      '  fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> { &self.__flight_entity_snapshot }',
      '  fn __flight_fresh_clone(&self) -> Self {',
      '    let mut cloned = self.clone();',
      '    cloned.__flight_identity = std::sync::Arc::new(());',
      '    cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(self.__flight_entity_runtime.lock().unwrap().clone()));',
      '    cloned',
      '  }',
      '}',
    );
  }
  return `${emitAnonymousDefinitions(structuralContext, exported, !exported)}${emitted.join('\n')}`;
}

function emittedDeclarationUsesNamedParameter(
  name: string,
  declaration: IrType,
  parameter: string,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): boolean {
  const key = `${name}\0${parameter}`;
  if (visited.has(key)) return true;
  const next = new Set([...visited, key]);
  if (declaration.kind === 'anonymous') {
    return flattenStructFields(declaration, context).some((field) =>
      emittedTypeUsesNamedParameter(field.type, parameter, context, next),
    );
  }
  return emittedTypeUsesNamedParameter(declaration, parameter, context, next);
}

function emittedTypeUsesNamedParameter(
  type: IrType,
  parameter: string,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): boolean {
  switch (type.kind) {
    case 'anonymous':
      return flattenStructFields(type, context).some((field) =>
        emittedTypeUsesNamedParameter(field.type, parameter, context, visited),
      );
    case 'array':
      return emittedTypeUsesNamedParameter(type.element, parameter, context, visited);
    case 'function':
      return (
        type.parameters.some((item) => emittedTypeUsesNamedParameter(item, parameter, context, visited)) ||
        emittedTypeUsesNamedParameter(type.returns, parameter, context, visited)
      );
    case 'named': {
      if (type.name === parameter) return true;
      const declaration = context.namedTypes.get(type.name);
      const declarationParameters = context.namedTypeParameters.get(type.name) ?? [];
      if (!declaration || declarationParameters.length === 0) {
        return type.arguments.some((argument) => emittedTypeUsesNamedParameter(argument, parameter, context, visited));
      }
      return declarationParameters.some(
        (declarationParameter, index) =>
          emittedDeclarationUsesNamedParameter(type.name, declaration, declarationParameter, context, visited) &&
          Boolean(
            type.arguments[index] && emittedTypeUsesNamedParameter(type.arguments[index]!, parameter, context, visited),
          ),
      );
    }
    case 'nullable':
      return emittedTypeUsesNamedParameter(type.inner, parameter, context, visited);
    case 'task':
      return emittedTypeUsesNamedParameter(type.output, parameter, context, visited);
    case 'union':
      return type.variants.some((variant) => emittedTypeUsesNamedParameter(variant, parameter, context, visited));
    case 'dynamic':
    case 'primitive':
      return false;
  }
}

function emitEntityRuntimeSlotDeclaration(
  name: string,
  exported: boolean,
  type: Extract<IrType, { kind: 'anonymous' }>,
  context: EmitContext,
  typeParameters: readonly string[] = [],
): string {
  const visibility = exported ? 'pub ' : 'pub(crate) ';
  const structuralContext = {
    ...typeDeclarationContext(context, name, type),
    lexicalTypeParameters: new Set(typeParameters),
  };
  const fields = type.fields.filter((field) => context.entityRuntimeFieldSlots.get(`${name}\0${field.name}`) === name);
  const emitted = fields.map((field) => {
    const fieldType =
      field.optional && field.type.kind !== 'nullable'
        ? `Option<${emitStructFieldType(field.type, name, structuralContext)}>`
        : emitStructFieldType(field.type, name, structuralContext);
    const storageType = context.entityRuntimeLateFields.has(`${name}\0${field.name}`)
      ? `Option<${fieldType}>`
      : fieldType;
    return `pub ${safeName(field.name)}: ${storageType},`;
  });
  const generics = typeParameters.length > 0 ? `<${typeParameters.join(', ')}>` : '';
  const marker = entityRuntimeMarkerType(typeParameters);
  if (marker) {
    emitted.push('#[doc(hidden)]', `pub __flight_marker: std::marker::PhantomData<${marker}>,`);
  }
  const defaults = [
    ...fields.map((field) => `      ${safeName(field.name)}: Default::default(),`),
    ...(marker ? ['      __flight_marker: std::marker::PhantomData,'] : []),
  ].join('\n');
  return `${emitAnonymousDefinitions(structuralContext, exported, !exported)}${[
    '#[doc(hidden)]',
    `${visibility}struct ${name}Storage${generics} {`,
    indent(emitted.join('\n')),
    '}',
    `impl${generics} Default for ${name}Storage${generics} {`,
    '  fn default() -> Self {',
    '    Self {',
    defaults,
    '    }',
    '  }',
    '}',
  ].join('\n')}`;
}

function entityRuntimeTypePath(context: EmitContext): string {
  if (context.localTypeNames.has('EntityRuntime')) return 'EntityRuntime';
  const importedModule =
    context.importedModules.get('EntityRuntime') ??
    context.importedModules.get('Entity') ??
    context.importedModules.get('EntityRuntimeKey');
  return importedModule ? `${importedModule}::EntityRuntime` : 'crate::EntityRuntime';
}

function entityRuntimeMarkerTraitPath(context: EmitContext): string {
  if (context.localTypeNames.has('EntityRuntime')) return 'FlightEntityRuntimeMarker';
  const importedModule =
    context.importedModules.get('EntityRuntime') ??
    context.importedModules.get('Entity') ??
    context.importedModules.get('EntityRuntimeKey');
  return importedModule ? `${importedModule}::FlightEntityRuntimeMarker` : 'crate::FlightEntityRuntimeMarker';
}

function entityRuntimeMarkerType(typeParameters: readonly string[]): string | undefined {
  if (typeParameters.length === 0) return undefined;
  return typeParameters.length === 1 ? typeParameters[0] : `(${typeParameters.join(', ')})`;
}

function entityTraitTypePath(context: EmitContext): string {
  if (context.localTypeNames.has('Entity')) return 'FlightEntity';
  const importedModule =
    context.importedModules.get('FlightEntity') ??
    context.importedModules.get('EntityRuntime') ??
    context.importedModules.get('Entity') ??
    context.importedModules.get('EntityRuntimeKey');
  return importedModule ? `${importedModule}::FlightEntity` : 'crate::FlightEntity';
}

function emitStructFieldType(type: IrType, ownerName: string, context: EmitContext): string {
  if (type.kind === 'named' && type.name === ownerName) return `Box<${emitType(type, context)}>`;
  if (type.kind === 'nullable') {
    return `Option<${emitStructFieldType(type.inner, ownerName, context)}>`;
  }
  return emitType(type, context);
}

function recursiveStructFieldStorage(
  type: IrType,
  ownerName: string,
  optional = false,
): 'direct' | 'nullable' | undefined {
  if (type.kind === 'named' && type.name === ownerName) return optional ? 'nullable' : 'direct';
  return type.kind === 'nullable' && type.inner.kind === 'named' && type.inner.name === ownerName
    ? 'nullable'
    : undefined;
}

function recursiveStructPropertyStorage(
  objectType: IrType | undefined,
  propertyName: string,
  context: EmitContext,
): 'direct' | 'nullable' | undefined {
  const receiver = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (receiver?.kind !== 'named') return undefined;
  const resolved = resolveSemanticType(receiver, context);
  if (resolved?.kind !== 'anonymous') return undefined;
  const field = semanticStructFields(receiver, context).find((candidate) => candidate.name === propertyName);
  return field ? recursiveStructFieldStorage(field.type, receiver.name, field.optional) : undefined;
}

function emitRecursiveStructStorageValue(
  property: Extract<IrExpression, { kind: 'property' }>,
  value: IrExpression,
  logicalType: IrType | undefined,
  valueType: IrType | undefined,
  context: EmitContext,
): string | undefined {
  const storage = recursiveStructPropertyStorage(
    inferIrExpressionType(property.object, context),
    property.name,
    context,
  );
  if (!storage || !logicalType) return undefined;
  if (storage === 'direct') return `Box::new(${emitExpression(value, context, logicalType)})`;
  if (isNullishExpression(value)) return 'None';
  const inner = logicalType.kind === 'nullable' ? logicalType.inner : logicalType;
  return valueType?.kind === 'nullable'
    ? `${parenthesize(emitExpression(value, context, logicalType))}.map(Box::new)`
    : `Some(Box::new(${emitExpression(value, context, inner)}))`;
}

function emitRecursiveStructFieldStorageValue(
  ownerName: string,
  field: IrTypeField,
  value: IrExpression,
  context: EmitContext,
): string | undefined {
  const storage = recursiveStructFieldStorage(field.type, ownerName, field.optional);
  if (!storage) return undefined;
  const logicalType =
    field.optional && field.type.kind !== 'nullable' ? ({ inner: field.type, kind: 'nullable' } as const) : field.type;
  if (storage === 'direct') return `Box::new(${emitExpression(value, context, logicalType)})`;
  if (isNullishExpression(value)) return 'None';
  const valueType = inferIrExpressionType(value, context);
  const inner = logicalType.kind === 'nullable' ? logicalType.inner : logicalType;
  return valueType?.kind === 'nullable'
    ? `${parenthesize(emitExpression(value, context, logicalType))}.map(Box::new)`
    : `Some(Box::new(${emitExpression(value, context, inner)}))`;
}

function flattenStructFields(
  type: Extract<IrType, { kind: 'anonymous' }>,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): Extract<IrType, { kind: 'anonymous' }>['fields'] {
  const inherited = type.extends.flatMap((base) => {
    if (base.kind === 'anonymous') return flattenStructFields(base, context, visited);
    if (base.kind !== 'named') return [];
    const application = typeKey(base);
    if (visited.has(application)) return [];
    const concrete = resolveSemanticType(base, context);
    if (concrete?.kind !== 'anonymous') return [];
    const fields = flattenStructFields(concrete, context, new Set([...visited, application]));
    const importedModule = context.importedModules.get(base.name);
    if (!importedModule || context.localTypeNames.has(base.name)) return fields;
    const nestedNames = importedNestedStructuralNames(base.name, concrete, importedModule);
    return fields.map((field) => ({
      ...field,
      type: externalizeImportedNestedType(field.type, nestedNames),
    }));
  });
  const fields = new Map(inherited.map((field) => [field.name, field]));
  for (const field of type.fields) {
    const inheritedField = fields.get(field.name);
    fields.set(
      field.name,
      inheritedField && structurallyCompatibleTypes(inheritedField.type, field.type, context)
        ? { ...field, type: inheritedField.type }
        : field,
    );
  }
  return [...fields.values()];
}

function registerEntityRuntimeFamilies(context: EmitContext): void {
  const namedTypes = context.namedTypes as Map<string, IrType>;
  if (!namedTypes.has('Entity') && !namedTypes.has('EntityRuntime')) return;

  const reaches = (
    name: string,
    root: 'Entity' | 'EntityRuntime',
    visited: ReadonlySet<string> = new Set(),
  ): boolean => {
    if (name === root) return true;
    if (visited.has(name)) return false;
    const declaration = namedTypes.get(name);
    if (!declaration) return false;
    const nextVisited = new Set([...visited, name]);
    if (declaration.kind === 'named') return reaches(declaration.name, root, nextVisited);
    return (
      declaration.kind === 'anonymous' &&
      declaration.extends.some((base) => base.kind === 'named' && reaches(base.name, root, nextVisited))
    );
  };

  const runtimeTypes = new Set(
    [...namedTypes.keys()]
      .filter((name) => reaches(name, 'EntityRuntime'))
      .sort((left, right) => left.localeCompare(right)),
  );
  const entityTypes = new Set(
    [...namedTypes.keys()].filter((name) => reaches(name, 'Entity')).sort((left, right) => left.localeCompare(right)),
  );
  for (const runtimeName of runtimeTypes) {
    if (!runtimeName.endsWith('Runtime')) continue;
    const entityName = runtimeName.slice(0, -'Runtime'.length);
    if (namedTypes.has(entityName)) entityTypes.add(entityName);
  }
  (context.entityRuntimeTypes as Set<string>).clear();
  runtimeTypes.forEach((name) => (context.entityRuntimeTypes as Set<string>).add(name));
  (context.entityTypes as Set<string>).clear();
  entityTypes.forEach((name) => (context.entityTypes as Set<string>).add(name));

  const root = namedTypes.get('EntityRuntime');
  if (root?.kind !== 'anonymous') return;
  const originalContext: EmitContext = {
    ...context,
    namedTypes: new Map(namedTypes),
  };
  const rootFields = flattenStructFields(root, originalContext);
  const rootFieldNames = new Set(rootFields.map((field) => field.name));
  const occurrences = new Map<string, IrTypeField[]>();
  const unavailableFields = context.entityRuntimeUnavailableFields as Map<string, string>;
  const genericSlotTypes = context.entityRuntimeGenericSlotTypes as Set<string>;
  for (const name of runtimeTypes) {
    const declaration = originalContext.namedTypes.get(name);
    if (declaration?.kind !== 'anonymous') continue;
    const parameters = originalContext.namedTypeParameters.get(name) ?? [];
    for (const field of declaration.fields) {
      const lexical = parameters.find((parameter) => typeUsesNamedParameter(field.type, parameter));
      if (lexical) {
        genericSlotTypes.add(name);
        continue;
      }
      const fields = occurrences.get(field.name) ?? [];
      fields.push(field);
      occurrences.set(field.name, fields);
    }
  }

  const slottedFields = new Set<string>();
  const aggregateFields: IrTypeField[] = [];
  for (const [fieldName, fields] of occurrences) {
    const [first, ...rest] = fields;
    if (
      !first ||
      rest.some(
        (field) =>
          runtimeStorageTypeKey(first.type, originalContext) !== runtimeStorageTypeKey(field.type, originalContext),
      ) ||
      fields.some((field) => collectAnonymousTypes(field.type).length > 0)
    ) {
      slottedFields.add(fieldName);
      continue;
    }
    aggregateFields.push({
      ...first,
      optional: fields.every((field) => field.optional),
    });
  }
  aggregateFields.sort((left, right) => left.name.localeCompare(right.name));
  const rootStorageFields = [...rootFields, ...aggregateFields.filter((field) => !rootFieldNames.has(field.name))].sort(
    (left, right) => left.name.localeCompare(right.name),
  );
  namedTypes.set('EntityRuntime', {
    extends: [],
    fields: rootStorageFields,
    kind: 'anonymous',
  });
  const addedFields = aggregateFields.filter((field) => !rootFieldNames.has(field.name)).map((field) => field.name);
  (context.openInterfaceFields as Map<string, ReadonlySet<string>>).set('EntityRuntime', new Set(addedFields));

  const fieldSlots = context.entityRuntimeFieldSlots as Map<string, string>;
  const slotTypes = context.entityRuntimeSlotTypes as Set<string>;
  const mappings = new Map<string, ReadonlyMap<string, string>>();
  const unavailableMappings = new Map<string, ReadonlyMap<string, string>>();
  const mapFields = (name: string, visited: ReadonlySet<string> = new Set()): ReadonlyMap<string, string> => {
    const cached = mappings.get(name);
    if (cached) return cached;
    if (visited.has(name)) return new Map();
    const declaration = originalContext.namedTypes.get(name);
    const mapped = new Map<string, string>();
    const unavailable = new Map<string, string>();
    const nextVisited = new Set([...visited, name]);
    if (declaration?.kind === 'named') {
      for (const [field, owner] of mapFields(declaration.name, nextVisited)) mapped.set(field, owner);
      for (const [field, reason] of unavailableMappings.get(declaration.name) ?? []) {
        unavailable.set(field, reason);
      }
    } else if (declaration?.kind === 'anonymous') {
      for (const base of declaration.extends) {
        if (base.kind !== 'named') continue;
        for (const [field, owner] of mapFields(base.name, nextVisited)) mapped.set(field, owner);
        for (const [field, reason] of unavailableMappings.get(base.name) ?? []) {
          unavailable.set(field, reason);
        }
      }
      for (const field of declaration.fields) {
        const parameters = originalContext.namedTypeParameters.get(name) ?? [];
        if (parameters.some((parameter) => typeUsesNamedParameter(field.type, parameter))) {
          mapped.set(field.name, name);
          unavailable.delete(field.name);
          continue;
        }
        const unavailableReason = unavailableFields.get(`${name}\0${field.name}`);
        if (unavailableReason) {
          mapped.delete(field.name);
          unavailable.set(field.name, unavailableReason);
          continue;
        }
        const owner = slottedFields.has(field.name) ? name : 'EntityRuntime';
        mapped.set(field.name, owner);
        unavailable.delete(field.name);
        if (owner !== 'EntityRuntime') slotTypes.add(owner);
      }
    }
    mappings.set(name, mapped);
    unavailableMappings.set(name, unavailable);
    return mapped;
  };
  for (const name of runtimeTypes) {
    for (const [field, owner] of mapFields(name)) fieldSlots.set(`${name}\0${field}`, owner);
    for (const [field, reason] of unavailableMappings.get(name) ?? []) {
      unavailableFields.set(`${name}\0${field}`, reason);
    }
  }
  for (const field of rootStorageFields) fieldSlots.set(`EntityRuntime\0${field.name}`, 'EntityRuntime');

  for (const field of rootStorageFields) {
    if (!field.optional && !rustTypeSupportsDefault(field.type, context)) {
      (context.entityRuntimeLateFields as Set<string>).add(`EntityRuntime\0${field.name}`);
    }
  }
  for (const owner of new Set([...slotTypes, ...genericSlotTypes])) {
    const declaration = originalContext.namedTypes.get(owner);
    if (declaration?.kind !== 'anonymous') continue;
    for (const field of declaration.fields) {
      if (!slottedFields.has(field.name) && !genericSlotTypes.has(owner)) continue;
      if (!field.optional && !rustTypeSupportsDefault(field.type, originalContext)) {
        (context.entityRuntimeLateFields as Set<string>).add(`${owner}\0${field.name}`);
      }
    }
  }
  if (!context.localTypeNames.has('EntityRuntime') && !context.entityRuntimeAggregateAvailable) {
    const localStorageAdditions = [...runtimeTypes].flatMap((name) => {
      if (name === 'EntityRuntime' || !context.localTypeNames.has(name)) return [];
      const declaration = originalContext.namedTypes.get(name);
      if (declaration?.kind !== 'anonymous') return [];
      return declaration.fields.flatMap((field) => {
        const owner = fieldSlots.get(`${name}\0${field.name}`);
        return owner && (owner !== 'EntityRuntime' || !rootFieldNames.has(field.name)) ? [`${name}.${field.name}`] : [];
      });
    });
    if (localStorageAdditions.length > 0) {
      context.entityRuntimeClosureError = `imported EntityRuntime aggregate cannot acquire package-local storage fields: ${localStorageAdditions.join(
        ', ',
      )}`;
    }
  }
}

function runtimeStorageTypeKey(type: IrType, context: EmitContext): string {
  return typeKey(runtimeStorageCanonicalType(type, context));
}

function runtimeStorageCanonicalType(
  type: IrType,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): IrType {
  if (type.kind === 'named') {
    const declaration = context.namedTypes.get(type.name);
    const application = typeKey(type);
    if (declaration && declaration.kind !== 'anonymous' && !visited.has(application)) {
      const parameters = context.namedTypeParameters.get(type.name) ?? [];
      const substitutions = new Map(
        parameters.flatMap((parameter, index) =>
          type.arguments[index] ? [[parameter, type.arguments[index]!] as const] : [],
        ),
      );
      const applied = substitutions.size > 0 ? substituteIrType(declaration, substitutions) : declaration;
      return runtimeStorageCanonicalType(applied, context, new Set([...visited, application]));
    }
    return {
      ...type,
      arguments: type.arguments.map((argument) => runtimeStorageCanonicalType(argument, context, visited)),
    };
  }
  if (type.kind === 'anonymous') return type;
  if (type.kind === 'array') {
    return {
      element: runtimeStorageCanonicalType(type.element, context, visited),
      kind: 'array',
    };
  }
  if (type.kind === 'function') {
    return {
      kind: 'function',
      parameters: type.parameters.map((parameter) => runtimeStorageCanonicalType(parameter, context, visited)),
      returns: runtimeStorageCanonicalType(type.returns, context, visited),
    };
  }
  if (type.kind === 'nullable') {
    return {
      inner: runtimeStorageCanonicalType(type.inner, context, visited),
      kind: 'nullable',
    };
  }
  if (type.kind === 'task') {
    return {
      kind: 'task',
      output: runtimeStorageCanonicalType(type.output, context, visited),
    };
  }
  if (type.kind === 'union') {
    return {
      kind: 'union',
      variants: type.variants.map((variant) => runtimeStorageCanonicalType(variant, context, visited)),
    };
  }
  return type;
}

function registerOpenInterfaceFamilies(context: EmitContext): void {
  const namedTypes = context.namedTypes as Map<string, IrType>;
  const openFields = context.openInterfaceFields as Map<string, ReadonlySet<string>>;
  const originalTypes = new Map(namedTypes);
  const originalContext: EmitContext = {
    ...context,
    namedTypes: originalTypes,
    openInterfaceFields: new Map(),
  };
  const directDescendants = new Map<string, Set<string>>();
  for (const [name, type] of originalTypes) {
    if (type.kind !== 'anonymous') continue;
    for (const base of type.extends) {
      if (base.kind !== 'named' || !originalTypes.has(base.name)) continue;
      const descendants = directDescendants.get(base.name) ?? new Set<string>();
      descendants.add(name);
      directDescendants.set(base.name, descendants);
    }
  }
  const descendantsOf = (baseName: string): string[] => {
    const found = new Set<string>();
    const pending = [...(directDescendants.get(baseName) ?? [])];
    while (pending.length > 0) {
      const name = pending.pop()!;
      if (found.has(name)) continue;
      found.add(name);
      pending.push(...(directDescendants.get(name) ?? []));
    }
    return [...found];
  };
  for (const [baseName, baseType] of originalTypes) {
    if (baseType.kind !== 'anonymous') continue;
    const descendants = descendantsOf(baseName).filter((name) => {
      const type = originalTypes.get(name);
      return type?.kind === 'anonymous' && type.fields.some((field) => field.name === 'kind');
    });
    if (descendants.length === 0) continue;
    const baseFields = flattenStructFields(baseType, originalContext);
    if (!baseFields.some((field) => field.name === 'kind')) continue;
    const existingNames = new Set(baseFields.map((field) => field.name));
    const added = new Map<string, IrTypeField>();
    const conflicts = new Set<string>();
    for (const descendantName of descendants) {
      const descendant = originalTypes.get(descendantName);
      if (descendant?.kind !== 'anonymous') continue;
      for (const field of flattenStructFields(descendant, originalContext)) {
        if (existingNames.has(field.name) || conflicts.has(field.name)) continue;
        const previous = added.get(field.name);
        if (!previous) {
          added.set(field.name, field);
          continue;
        }
        if (!structurallyCompatibleTypes(previous.type, field.type, originalContext)) {
          added.delete(field.name);
          conflicts.add(field.name);
          continue;
        }
        added.set(field.name, {
          ...previous,
          optional: previous.optional && field.optional,
        });
      }
    }
    if (added.size === 0) continue;
    const enrichedBase: IrType = {
      ...baseType,
      fields: [...baseType.fields, ...added.values()],
    };
    const familyTypes = new Map(originalTypes);
    familyTypes.set(baseName, enrichedBase);
    const familyContext: EmitContext = {
      ...originalContext,
      namedTypes: familyTypes,
    };
    if (
      [baseName, ...descendants].some((name) => {
        const type = familyTypes.get(name);
        return (
          type?.kind === 'anonymous' &&
          flattenStructFields(type, familyContext).some(
            (field) => !field.optional && !rustTypeSupportsDefault(field.type, familyContext),
          )
        );
      })
    ) {
      continue;
    }
    namedTypes.set(baseName, enrichedBase);
    openFields.set(baseName, new Set(added.keys()));
    for (const descendant of descendants) {
      if (!openFields.has(descendant)) openFields.set(descendant, new Set());
    }
  }
}

function semanticStructFields(type: IrType, context: EmitContext): IrTypeField[] {
  const resolved = resolveSemanticType(type, context);
  if (resolved?.kind !== 'anonymous') return [];
  const fields = flattenStructFields(resolved, context);
  if (type.kind !== 'named') return fields;
  const importedModule = context.importedModules.get(type.name);
  if (!importedModule || context.localTypeNames.has(type.name)) return fields;
  const nestedNames = importedNestedStructuralNames(type.name, resolved, importedModule);
  return fields.map((field) => ({
    ...field,
    type: externalizeImportedNestedType(field.type, nestedNames),
  }));
}

function importedNestedStructuralNames(
  ownerName: string,
  type: Extract<IrType, { kind: 'anonymous' }>,
  importedModule: string,
): ReadonlyMap<string, string> {
  const nestedNames = new Map<string, string>();
  for (const nested of collectAnonymousTypes(type)) {
    const key = typeKey(nested);
    if (key === typeKey(type) || nestedNames.has(key)) continue;
    nestedNames.set(key, `${importedModule}::${pascalCase(ownerName)}Record${String(nestedNames.size + 1)}`);
  }
  return nestedNames;
}

function externalizeImportedNestedType(type: IrType, names: ReadonlyMap<string, string>): IrType {
  if (type.kind === 'anonymous') {
    const name = names.get(typeKey(type));
    if (name)
      return {
        arguments: [type],
        kind: 'named',
        name: `RustStructural:${name}`,
      };
    return {
      extends: type.extends.map((item) => externalizeImportedNestedType(item, names)),
      fields: type.fields.map((field) => ({
        ...field,
        type: externalizeImportedNestedType(field.type, names),
      })),
      kind: 'anonymous',
    };
  }
  if (type.kind === 'array') {
    return {
      element: externalizeImportedNestedType(type.element, names),
      kind: 'array',
    };
  }
  if (type.kind === 'function') {
    return {
      kind: 'function',
      parameters: type.parameters.map((item) => externalizeImportedNestedType(item, names)),
      returns: externalizeImportedNestedType(type.returns, names),
    };
  }
  if (type.kind === 'named') {
    return {
      arguments: type.arguments.map((item) => externalizeImportedNestedType(item, names)),
      kind: 'named',
      name: type.name,
    };
  }
  if (type.kind === 'nullable') {
    return {
      inner: externalizeImportedNestedType(type.inner, names),
      kind: 'nullable',
    };
  }
  if (type.kind === 'task') {
    return {
      kind: 'task',
      output: externalizeImportedNestedType(type.output, names),
    };
  }
  if (type.kind === 'union') {
    return {
      kind: 'union',
      variants: type.variants.map((item) => externalizeImportedNestedType(item, names)),
    };
  }
  return type;
}

function structurallyCompatibleTypes(
  left: IrType,
  right: IrType,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): boolean {
  const resolvedLeft = resolveSemanticType(left, context) ?? left;
  const resolvedRight = resolveSemanticType(right, context) ?? right;
  const pair = `${typeKey(resolvedLeft)}\u0000${typeKey(resolvedRight)}`;
  if (visited.has(pair)) return true;
  if (resolvedLeft.kind !== resolvedRight.kind) return false;
  const nextVisited = new Set([...visited, pair]);
  switch (resolvedLeft.kind) {
    case 'anonymous': {
      if (resolvedRight.kind !== 'anonymous') return false;
      const rightFields = new Map(resolvedRight.fields.map((field) => [field.name, field]));
      return (
        resolvedLeft.fields.length === resolvedRight.fields.length &&
        resolvedLeft.fields.every((field) => {
          const candidate = rightFields.get(field.name);
          return Boolean(
            candidate &&
            candidate.optional === field.optional &&
            structurallyCompatibleTypes(field.type, candidate.type, context, nextVisited),
          );
        })
      );
    }
    case 'array':
      return (
        resolvedRight.kind === 'array' &&
        structurallyCompatibleTypes(resolvedLeft.element, resolvedRight.element, context, nextVisited)
      );
    case 'dynamic':
      return true;
    case 'function':
      return (
        resolvedRight.kind === 'function' &&
        resolvedLeft.parameters.length === resolvedRight.parameters.length &&
        resolvedLeft.parameters.every((parameter, index) =>
          structurallyCompatibleTypes(parameter, resolvedRight.parameters[index]!, context, nextVisited),
        ) &&
        structurallyCompatibleTypes(resolvedLeft.returns, resolvedRight.returns, context, nextVisited)
      );
    case 'named':
      return (
        resolvedRight.kind === 'named' &&
        resolvedLeft.name === resolvedRight.name &&
        resolvedLeft.arguments.length === resolvedRight.arguments.length &&
        resolvedLeft.arguments.every((argument, index) =>
          structurallyCompatibleTypes(argument, resolvedRight.arguments[index]!, context, nextVisited),
        )
      );
    case 'nullable':
      return (
        resolvedRight.kind === 'nullable' &&
        structurallyCompatibleTypes(resolvedLeft.inner, resolvedRight.inner, context, nextVisited)
      );
    case 'primitive':
      return resolvedRight.kind === 'primitive' && resolvedLeft.name === resolvedRight.name;
    case 'task':
      return (
        resolvedRight.kind === 'task' &&
        structurallyCompatibleTypes(resolvedLeft.output, resolvedRight.output, context, nextVisited)
      );
    case 'union':
      return (
        resolvedRight.kind === 'union' &&
        resolvedLeft.variants.length === resolvedRight.variants.length &&
        resolvedLeft.variants.every((variant, index) =>
          structurallyCompatibleTypes(variant, resolvedRight.variants[index]!, context, nextVisited),
        )
      );
  }
}

function substituteIrType(type: IrType, substitutions: ReadonlyMap<string, IrType>): IrType {
  if (type.kind === 'named' && type.arguments.length === 0) {
    const replacement = substitutions.get(type.name);
    if (replacement) return replacement;
  }
  switch (type.kind) {
    case 'anonymous':
      return {
        extends: type.extends.map((item) => substituteIrType(item, substitutions)),
        fields: type.fields.map((field) => ({
          ...field,
          type: substituteIrType(field.type, substitutions),
        })),
        kind: 'anonymous',
      };
    case 'array':
      return {
        element: substituteIrType(type.element, substitutions),
        kind: 'array',
      };
    case 'function':
      return {
        kind: 'function',
        parameters: type.parameters.map((item) => substituteIrType(item, substitutions)),
        returns: substituteIrType(type.returns, substitutions),
      };
    case 'named':
      return {
        arguments: type.arguments.map((item) => substituteIrType(item, substitutions)),
        kind: 'named',
        name: type.name,
      };
    case 'nullable':
      return {
        inner: substituteIrType(type.inner, substitutions),
        kind: 'nullable',
      };
    case 'task':
      return {
        kind: 'task',
        output: substituteIrType(type.output, substitutions),
      };
    case 'union':
      return {
        kind: 'union',
        variants: type.variants.map((item) => substituteIrType(item, substitutions)),
      };
    case 'dynamic':
    case 'primitive':
      return type;
  }
}

function typeUsesNamedParameter(type: IrType, name: string): boolean {
  switch (type.kind) {
    case 'anonymous':
      return (
        type.extends.some((item) => typeUsesNamedParameter(item, name)) ||
        type.fields.some((field) => typeUsesNamedParameter(field.type, name))
      );
    case 'array':
      return typeUsesNamedParameter(type.element, name);
    case 'function':
      return (
        type.parameters.some((item) => typeUsesNamedParameter(item, name)) || typeUsesNamedParameter(type.returns, name)
      );
    case 'named':
      return type.name === name || type.arguments.some((item) => typeUsesNamedParameter(item, name));
    case 'nullable':
      return typeUsesNamedParameter(type.inner, name);
    case 'task':
      return typeUsesNamedParameter(type.output, name);
    case 'union':
      return type.variants.some((item) => typeUsesNamedParameter(item, name));
    case 'dynamic':
    case 'primitive':
      return false;
  }
}

function collectReferencedNamedTypes(type: IrType): ReadonlySet<string> {
  const names = new Set<string>();
  const visit = (candidate: IrType): void => {
    switch (candidate.kind) {
      case 'anonymous':
        candidate.extends.forEach(visit);
        candidate.fields.forEach((field) => visit(field.type));
        break;
      case 'array':
        visit(candidate.element);
        break;
      case 'function':
        candidate.parameters.forEach(visit);
        visit(candidate.returns);
        break;
      case 'named':
        names.add(candidate.name);
        candidate.arguments.forEach(visit);
        break;
      case 'nullable':
        visit(candidate.inner);
        break;
      case 'task':
        visit(candidate.output);
        break;
      case 'union':
        candidate.variants.forEach(visit);
        break;
      case 'dynamic':
      case 'primitive':
        break;
    }
  };
  visit(type);
  return names;
}

function inferCallbackTypeParameters(declaration: IrFunctionDeclaration): ReadonlySet<string> {
  const callbackContainers = new Set(['FlightCallbackArgs', 'Signal', 'SignalConnection', 'SignalData']);
  const usesCallbackContainer = (type: IrType, parameter: string, insideCallback = false): boolean => {
    switch (type.kind) {
      case 'anonymous':
        return (
          type.extends.some((item) => usesCallbackContainer(item, parameter, insideCallback)) ||
          type.fields.some((field) => usesCallbackContainer(field.type, parameter, insideCallback))
        );
      case 'array':
        return usesCallbackContainer(type.element, parameter, insideCallback);
      case 'function':
        return (
          type.parameters.some((item) => usesCallbackContainer(item, parameter, insideCallback)) ||
          usesCallbackContainer(type.returns, parameter, insideCallback)
        );
      case 'named': {
        const nested = insideCallback || callbackContainers.has(type.name);
        return (
          (nested && type.name === parameter) ||
          type.arguments.some((item) => usesCallbackContainer(item, parameter, nested))
        );
      }
      case 'nullable':
        return usesCallbackContainer(type.inner, parameter, insideCallback);
      case 'task':
        return usesCallbackContainer(type.output, parameter, insideCallback);
      case 'union':
        return type.variants.some((item) => usesCallbackContainer(item, parameter, insideCallback));
      case 'dynamic':
      case 'primitive':
        return false;
    }
  };
  return new Set(
    declaration.typeParameters.filter((parameter) =>
      [...declaration.parameters.map((item) => item.type), declaration.returns].some((type) =>
        usesCallbackContainer(type, parameter),
      ),
    ),
  );
}

function inferEntityTypeParameters(
  owner: unknown,
  typeParameters: readonly string[],
  context: EmitContext,
): ReadonlySet<string> {
  const lexical = new Set(typeParameters);
  const found = new Set<string>();
  const aliases = new Map<string, string>();
  const collectAliases = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'function') return;
    if ('kind' in value && value.kind === 'variable' && 'declarations' in value) {
      for (const variable of value.declarations as IrVariable[]) {
        if (!variable.initializer) continue;
        const initializer = unwrapCasts(variable.initializer);
        if (initializer.kind !== 'object' || initializer.properties.length !== 1) continue;
        const spread = initializer.properties[0];
        if (spread?.kind !== 'spread') continue;
        const sourceType = inferIrExpressionType(spread.expression, context);
        if (sourceType?.kind === 'named' && lexical.has(sourceType.name)) {
          aliases.set(variable.name, sourceType.name);
        }
      }
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(collectAliases);
      else collectAliases(child);
    }
  };
  collectAliases(owner);
  const visit = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'cast') {
      const expression = value as Extract<IrExpression, { kind: 'cast' }>;
      if (expression.type.kind === 'named' && lexical.has(expression.type.name)) {
        const sourceType = inferIrExpressionType(unwrapCasts(expression.expression), context);
        const sourceEntityType = sourceType?.kind === 'nullable' ? sourceType.inner : sourceType;
        if (sourceEntityType && isNativeEntityType(sourceEntityType, context)) {
          found.add(expression.type.name);
        }
      }
    }
    if ('kind' in value && value.kind === 'element') {
      const expression = value as Extract<IrExpression, { kind: 'element' }>;
      if (isErasedEntityRuntimeAccess(expression)) {
        const objectType = inferIrExpressionType(expression.object, context);
        if (objectType?.kind === 'named' && lexical.has(objectType.name)) found.add(objectType.name);
        if (expression.object.kind === 'identifier') {
          const alias = aliases.get(expression.object.name);
          if (alias) found.add(alias);
        }
      }
    }
    if (
      'kind' in value &&
      value.kind === 'binary' &&
      'left' in value &&
      'right' in value &&
      value.left &&
      typeof value.left === 'object' &&
      'kind' in value.left &&
      value.left.kind === 'identifier' &&
      'name' in value.left &&
      value.left.name === 'EntityRuntimeKey' &&
      value.right &&
      typeof value.right === 'object'
    ) {
      const receiver = inferIrExpressionType(value.right as IrExpression, context);
      if (receiver?.kind === 'named' && lexical.has(receiver.name)) found.add(receiver.name);
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(owner);
  return found;
}

function inferCallbackArgumentStorage(
  declaration: IrFunctionDeclaration,
  callbackTypeParameters: ReadonlySet<string>,
): ReadonlyMap<string, string> {
  const storage = new Map<string, string>();
  const visit = (value: unknown, callback?: { argument: string; typeParameter: string }): void => {
    if (!value || typeof value !== 'object') return;
    if (
      'kind' in value &&
      value.kind === 'cast' &&
      'type' in value &&
      value.type &&
      typeof value.type === 'object' &&
      'kind' in value.type &&
      value.type.kind === 'named' &&
      'name' in value.type &&
      typeof value.type.name === 'string' &&
      callbackTypeParameters.has(value.type.name) &&
      'expression' in value &&
      value.expression &&
      typeof value.expression === 'object'
    ) {
      const expression = unwrapCasts(value.expression as IrExpression);
      if (expression.kind === 'function' && expression.parameters.length === 1) {
        const callbackExpression = expression as Extract<IrExpression, { kind: 'function' }>;
        const parameter = callbackExpression.parameters[0]!;
        visit(callbackExpression.body, {
          argument: parameter.name,
          typeParameter: value.type.name,
        });
        if (callbackExpression.expression) {
          visit(callbackExpression.expression, {
            argument: parameter.name,
            typeParameter: value.type.name,
          });
        }
        return;
      }
    }
    if (
      callback &&
      'kind' in value &&
      value.kind === 'assignment' &&
      'left' in value &&
      value.left &&
      typeof value.left === 'object' &&
      'kind' in value.left &&
      value.left.kind === 'identifier' &&
      'name' in value.left &&
      typeof value.left.name === 'string' &&
      'right' in value &&
      value.right &&
      typeof value.right === 'object' &&
      'kind' in value.right &&
      value.right.kind === 'identifier' &&
      'name' in value.right &&
      value.right.name === callback.argument
    ) {
      storage.set(value.left.name, callback.typeParameter);
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach((item) => visit(item, callback));
      else visit(child, callback);
    }
  };
  visit(declaration.body);
  return storage;
}

function inferCallbackTypeParameter(expression: IrExpression, context: EmitContext): string | undefined {
  if (expression.kind === 'cast') {
    const inner = inferCallbackTypeParameter(expression.expression, context);
    if (inner) return inner;
  }
  const type = inferIrExpressionType(expression, context);
  const candidate = type?.kind === 'nullable' ? type.inner : type;
  return candidate?.kind === 'named' && context.callbackTypeParameters.has(candidate.name) ? candidate.name : undefined;
}

function emitCallbackArguments(
  arguments_: readonly IrExpression[],
  callbackTypeParameter: string,
  context: EmitContext,
): string {
  if (arguments_.length === 1 && arguments_[0]?.kind === 'spread') {
    const value = emitExpression(arguments_[0].expression, context, {
      arguments: [{ arguments: [], kind: 'named', name: callbackTypeParameter }],
      kind: 'named',
      name: 'FlightCallbackArgs',
    });
    return `${parenthesize(value)}.clone()`;
  }
  const values = arguments_.map((argument) =>
    argument.kind === 'spread' ? emitExpression(argument.expression, context) : emitExpression(argument, context),
  );
  if (values.length === 0) return '()';
  return `(${values.join(', ')}${values.length === 1 ? ',' : ''})`;
}

function functionContext(context: EmitContext, ownerName: string, owner: unknown, returns: IrType): EmitContext {
  const anonymousTypes = new Map(context.anonymousTypes);
  let index = anonymousTypes.size + 1;
  for (const type of [...collectAnonymousTypes(owner), ...collectInferredObjectTypes(owner)]) {
    const key = typeKey(type);
    if (!anonymousTypes.has(key)) {
      anonymousTypes.set(key, `${pascalCase(ownerName)}Record${String(index++)}`);
    }
  }
  const sharedCaptureNames = new Set([
    ...context.sharedCaptureNames,
    ...collectSharedMutableLocals(owner, context.mutatingFunctions),
  ]);
  const forwardClosureCaptureNames = collectForwardClosureCaptures(owner);
  const recursiveClosureSlots = new Map(context.recursiveClosureSlots);
  for (const name of forwardClosureCaptureNames) {
    recursiveClosureSlots.set(name, `__flight_forward_${safeName(name)}`);
  }
  const utf16ViewNames = new Map(context.utf16ViewNames);
  if (owner && typeof owner === 'object' && 'parameters' in owner && Array.isArray(owner.parameters)) {
    for (const parameter of owner.parameters as IrParameter[]) utf16ViewNames.delete(parameter.name);
  }
  return {
    ...context,
    anonymousTypeOwner: ownerName,
    anonymousTypes,
    borrowedNames: new Set(),
    captureReturns: false,
    continueEpilogue: [],
    currentReturnType: returns,
    errorValueNames: new Set(context.errorValueNames),
    excludedUnionVariants: new Map(),
    forwardClosureCaptureNames,
    inheritedAnonymousTypeKeys: new Set(context.anonymousTypes.keys()),
    knownNullNames: new Set(),
    mutatedNames: collectMutatedNames(owner, context.mutatingFunctions),
    nonNullableNames: new Set(context.nonNullableNames),
    placeAliases: new Map(),
    preservedNames: new Set(),
    rawClosureNames: new Set(context.rawClosureNames),
    recursiveClosureSlots,
    sharedCaptureNames,
    symbolTypes: new Map(context.symbolTypes),
    timerHandleNames: new Set([...context.timerHandleNames, ...collectTimerHandleNames(owner)]),
    unionNarrowings: new Map(),
    utf16ViewNames,
  };
}

function prepareParameterUtf16Views(parameters: readonly IrParameter[], body: unknown, context: EmitContext): string[] {
  const views = context.utf16ViewNames as Map<string, string>;
  return parameters.flatMap((parameter) => {
    const resolved = resolveSemanticType(parameter.type, context);
    if (
      resolved?.kind !== 'primitive' ||
      resolved.name !== 'String' ||
      (parameter.optional && !parameter.initializer) ||
      !usesStringUtf16Access(body, parameter.name)
    ) {
      return [];
    }
    const view = `__flight_utf16_${safeName(parameter.name)}`;
    views.set(parameter.name, view);
    return [
      `let ${view}: std::sync::Arc<Vec<u16>> = std::sync::Arc::new(${safeName(parameter.name)}.encode_utf16().collect());`,
    ];
  });
}

function usesStringUtf16Access(value: unknown, name: string): boolean {
  if (!value || typeof value !== 'object') return false;
  if (
    'kind' in value &&
    value.kind === 'property' &&
    'name' in value &&
    value.name === 'length' &&
    'object' in value &&
    value.object &&
    typeof value.object === 'object' &&
    'kind' in value.object &&
    value.object.kind === 'identifier' &&
    'name' in value.object &&
    value.object.name === name
  ) {
    return true;
  }
  if (
    'kind' in value &&
    value.kind === 'element' &&
    'object' in value &&
    value.object &&
    typeof value.object === 'object' &&
    'kind' in value.object &&
    value.object.kind === 'identifier' &&
    'name' in value.object &&
    value.object.name === name
  ) {
    return true;
  }
  if (
    'kind' in value &&
    value.kind === 'call' &&
    'callee' in value &&
    value.callee &&
    typeof value.callee === 'object' &&
    'kind' in value.callee &&
    value.callee.kind === 'property' &&
    'name' in value.callee &&
    (value.callee.name === 'charCodeAt' || value.callee.name === 'codePointAt') &&
    'object' in value.callee &&
    value.callee.object &&
    typeof value.callee.object === 'object' &&
    'kind' in value.callee.object &&
    value.callee.object.kind === 'identifier' &&
    'name' in value.callee.object &&
    value.callee.object.name === name
  ) {
    return true;
  }
  return Object.values(value).some((child) =>
    Array.isArray(child) ? child.some((item) => usesStringUtf16Access(item, name)) : usesStringUtf16Access(child, name),
  );
}

function collectTimerHandleNames(owner: unknown): ReadonlySet<string> {
  const names = new Set<string>();
  const visit = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if (
      'kind' in value &&
      value.kind === 'call' &&
      'callee' in value &&
      value.callee &&
      typeof value.callee === 'object' &&
      'kind' in value.callee &&
      value.callee.kind === 'identifier' &&
      'name' in value.callee &&
      (value.callee.name === 'clearTimeout' || value.callee.name === 'clearInterval') &&
      'arguments' in value &&
      Array.isArray(value.arguments)
    ) {
      const timer = value.arguments[0] as IrExpression | undefined;
      if (timer?.kind === 'identifier') names.add(timer.name);
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(owner);
  return names;
}

function collectForwardClosureCaptures(owner: unknown): ReadonlySet<string> {
  if (!owner || typeof owner !== 'object' || !('body' in owner) || !Array.isArray(owner.body)) return new Set();
  const statements = owner.body as IrStatement[];
  const declarations = statements.map((statement) =>
    statement.kind === 'variable'
      ? statement.declarations.filter((variable) => !variable.mutable).map((variable) => variable.name)
      : [],
  );
  const captures = (value: unknown, name: string): boolean => {
    if (!value || typeof value !== 'object') return false;
    if ('kind' in value && value.kind === 'function') return containsIdentifier(value, name);
    return Object.values(value).some((child) =>
      Array.isArray(child) ? child.some((item) => captures(item, name)) : captures(child, name),
    );
  };
  const names = new Set<string>();
  for (const [index, statement] of statements.entries()) {
    for (const name of declarations.slice(index + 1).flat()) {
      if (captures(statement, name)) names.add(name);
    }
  }
  return names;
}

function collectSharedMutableLocals(
  owner: unknown,
  mutatingFunctions: ReadonlyMap<string, ReadonlySet<number>>,
): ReadonlySet<string> {
  const mutated = collectMutatedNames(owner, mutatingFunctions);
  const locals = new Set<string>();
  const visitLocals = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'function') return;
    if ('kind' in value && value.kind === 'variable' && 'declarations' in value) {
      for (const variable of value.declarations as IrVariable[]) {
        if (variable.mutable || mutated.has(variable.name)) locals.add(variable.name);
      }
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visitLocals);
      else visitLocals(child);
    }
  };
  if (owner && typeof owner === 'object' && 'body' in owner) visitLocals(owner.body);
  const captured = new Set<string>();
  const visitClosures = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'function') {
      for (const name of locals) {
        if (containsIdentifier(value, name)) captured.add(name);
      }
      return;
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visitClosures);
      else visitClosures(child);
    }
  };
  if (owner && typeof owner === 'object' && 'body' in owner) visitClosures(owner.body);
  return captured;
}

function registerContextualAnonymousTypes(value: unknown, context: EmitContext, ownerName: string): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  let index = anonymousTypes.size + 1;
  for (const type of collectAnonymousTypes(value)) {
    const key = typeKey(type);
    if (!anonymousTypes.has(key)) {
      anonymousTypes.set(key, `${pascalCase(ownerName)}ContextRecord${String(index++)}`);
    }
  }
}

function registerGlobalResolvedAnonymousTypes(value: unknown, context: EmitContext): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  const visit = (item: unknown): void => {
    if (!item || typeof item !== 'object') return;
    if (
      'kind' in item &&
      item.kind === 'named' &&
      'name' in item &&
      isStructuralUtilityType(item as IrType) &&
      'arguments' in item
    ) {
      const resolved = resolveSemanticType(item as IrType, context);
      if (resolved?.kind === 'anonymous') {
        const key = typeKey(resolved);
        if (!anonymousTypes.has(key)) {
          const prefix =
            (item as Extract<IrType, { kind: 'named' }>).name === 'FlightPartial' ? 'FlightPartial' : 'FlightOmit';
          anonymousTypes.set(key, `${prefix}Record${stableTypeIdentity(key)}`);
        }
      }
    }
    for (const child of Object.values(item)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
}

function registerImportedFunctionAnonymousTypes(
  declarations: readonly IrFunctionDeclaration[],
  context: EmitContext,
): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  const inherited = context.inheritedAnonymousTypeKeys as Set<string>;
  for (const declaration of [...declarations].sort(
    (left, right) =>
      left.origin.packageName.localeCompare(right.origin.packageName) ||
      left.origin.source.localeCompare(right.origin.source) ||
      left.name.localeCompare(right.name),
  )) {
    const crate = context.importedModules.get(declaration.name);
    if (!crate) continue;
    const module = implementationModuleName(declaration.origin.source);
    const prefix = crate === 'crate' ? `crate::${module}` : `${crate}::${module}`;
    const visit = (value: unknown): void => {
      if (!value || typeof value !== 'object') return;
      if (
        'kind' in value &&
        value.kind === 'named' &&
        'name' in value &&
        isStructuralUtilityType(value as IrType) &&
        'arguments' in value
      ) {
        const utility = value as Extract<IrType, { kind: 'named' }>;
        const resolved = resolveSemanticType(utility, context);
        if (resolved?.kind === 'anonymous') {
          const key = typeKey(resolved);
          const name = utility.name === 'FlightPartial' ? 'FlightPartial' : 'FlightOmit';
          anonymousTypes.set(key, `${prefix}::${name}Record${stableTypeIdentity(key)}`);
          inherited.add(key);
        }
      }
      for (const child of Object.values(value)) {
        if (Array.isArray(child)) child.forEach(visit);
        else visit(child);
      }
    };
    visit({ parameters: declaration.parameters, returns: declaration.returns });
  }
}

function implementationModuleName(source: string): string {
  const filename =
    source
      .split(/[\\/]/u)
      .at(-1)
      ?.replace(/\.tsx?$/u, '') ?? source;
  if (
    filename.toLowerCase() === 'index' ||
    filename.toLowerCase() === 'internal' ||
    /test(?:helper|util)/iu.test(filename)
  ) {
    return `_internal_${snakeCase(filename).replace(/^_+/u, '')}`;
  }
  return snakeCase(filename);
}

function registerSharedModuleAnonymousTypes(declarations: readonly IrDeclaration[], context: EmitContext): void {
  const occurrences = new Map<string, { owners: number; type: IrType }>();
  const signatureKeys = new Set<string>();
  const typeParameters = new Set(
    declarations.flatMap((declaration) =>
      declaration.kind === 'function' || declaration.kind === 'type' ? declaration.typeParameters : [],
    ),
  );
  for (const declaration of declarations) {
    for (const type of collectAnonymousTypes(declaration)) {
      const key = typeKey(type);
      const previous = occurrences.get(key);
      occurrences.set(key, {
        owners: (previous?.owners ?? 0) + 1,
        type,
      });
    }
    if (declaration.kind === 'function') {
      for (const type of collectAnonymousTypes({
        parameters: declaration.parameters,
        returns: declaration.returns,
      })) {
        signatureKeys.add(typeKey(type));
      }
    }
  }
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  for (const [key, occurrence] of occurrences) {
    if ((occurrence.owners < 2 && !signatureKeys.has(key)) || anonymousTypes.has(key)) continue;
    if ([...typeParameters].some((parameter) => typeUsesNamedParameter(occurrence.type, parameter))) continue;
    anonymousTypes.set(key, `SharedStructuralRecord${String(anonymousTypes.size + 1)}`);
  }
}

function registerTypeDeclarationAnonymousTypes(declarations: readonly IrDeclaration[], context: EmitContext): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  const anonymousTypeParameters = context.anonymousTypeParameters as Map<string, readonly string[]>;
  for (const declaration of declarations) {
    if (declaration.kind !== 'type' || (declaration.typeParameters.length === 0 && declaration.type.kind !== 'union')) {
      continue;
    }
    let index = 1;
    for (const type of collectResolvedAnonymousTypes(declaration.type, context)) {
      const key = typeKey(type);
      if (key === typeKey(declaration.type)) continue;
      if (!anonymousTypes.has(key)) anonymousTypes.set(key, `${pascalCase(declaration.name)}Record${String(index)}`);
      const parameters = declaration.typeParameters.filter((parameter) => typeUsesNamedParameter(type, parameter));
      if (parameters.length > 0) anonymousTypeParameters.set(key, parameters);
      index++;
    }
  }
}

function registerImportedTypeAnonymousTypes(context: EmitContext): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  const anonymousTypeParameters = context.anonymousTypeParameters as Map<string, readonly string[]>;
  const inherited = context.inheritedAnonymousTypeKeys as Set<string>;
  for (const [owner, declaration] of context.namedTypes) {
    if (context.localTypeNames.has(owner)) continue;
    const module = context.importedModules.get(owner);
    if (!module) continue;
    const typeParameters = context.namedTypeParameters.get(owner) ?? [];
    let index = 1;
    for (const type of collectResolvedAnonymousTypes(declaration, context)) {
      const key = typeKey(type);
      if (key === typeKey(declaration)) continue;
      if (!anonymousTypes.has(key)) {
        anonymousTypes.set(key, `${module}::${pascalCase(owner)}Record${String(index)}`);
        inherited.add(key);
      }
      const parameters = typeParameters.filter((parameter) => typeUsesNamedParameter(type, parameter));
      if (parameters.length > 0) anonymousTypeParameters.set(key, parameters);
      index++;
    }
  }
}

function registerNestedAnonymousTypes(context: EmitContext): void {
  for (const key of [...context.anonymousTypes.keys()]) {
    const type = JSON.parse(key) as IrType;
    if (type.kind === 'anonymous') registerInferredObjectType(type, context);
  }
}

function collectInferredObjectTypes(value: unknown): IrType[] {
  const found = new Map<string, IrType>();
  const visit = (item: unknown): void => {
    if (!item || typeof item !== 'object') return;
    if ('kind' in item && item.kind === 'object' && 'properties' in item) {
      const inferred = inferStaticExpressionType(item as IrExpression);
      if (inferred?.kind === 'anonymous') found.set(typeKey(inferred), inferred);
    }
    for (const child of Object.values(item)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(value);
  return [...found.values()];
}

function typeDeclarationContext(context: EmitContext, ownerName: string, type: IrType): EmitContext {
  const anonymousTypes = new Map(context.anonymousTypes);
  let index = 1;
  for (const nested of collectResolvedAnonymousTypes(type, context)) {
    const key = typeKey(nested);
    if (key === typeKey(type) || anonymousTypes.has(key)) continue;
    anonymousTypes.set(key, `${pascalCase(ownerName)}Record${String(index++)}`);
  }
  return {
    ...context,
    anonymousTypes,
    inheritedAnonymousTypeKeys: new Set(context.anonymousTypes.keys()),
  };
}

function collectResolvedAnonymousTypes(type: IrType, context: EmitContext): IrType[] {
  const found = new Map<string, IrType>();
  const pending = collectAnonymousTypes(type);
  while (pending.length > 0) {
    const candidate = pending.pop()!;
    const key = typeKey(candidate);
    if (found.has(key) || candidate.kind !== 'anonymous') continue;
    found.set(key, candidate);
    for (const field of flattenStructFields(candidate, context)) {
      pending.push(...collectAnonymousTypes(field.type));
    }
  }
  return [...found.values()];
}

function registerParameters(parameters: IrParameter[], context: EmitContext, fallbackTypes: IrType[] = []): void {
  parameters.forEach((parameter, index) => {
    const type = contextualParameterType(parameter.type, fallbackTypes[index], context);
    context.symbolTypes.set(
      parameter.name,
      parameter.optional && !parameter.initializer && type.kind !== 'nullable'
        ? { inner: type, kind: 'nullable' }
        : type,
    );
  });
}

function contextualParameterType(type: IrType, fallback: IrType | undefined, context: EmitContext): IrType {
  if (!fallback) return type;
  return type.kind === 'dynamic' || structurallyCompatibleTypes(type, fallback, context) ? fallback : type;
}

function inferEntitySpreadType(expression: IrExpression, context: EmitContext): IrType | undefined {
  const value = unwrapCasts(expression);
  if (value.kind !== 'object' || value.properties.length !== 1) return undefined;
  const spread = value.properties[0];
  if (spread?.kind !== 'spread') return undefined;
  const sourceType = inferIrExpressionType(spread.expression, context);
  return sourceType && isNativeEntityType(sourceType, context) ? sourceType : undefined;
}

function registerLocalTypes(statements: readonly IrStatement[], context: EmitContext): void {
  const candidates: Array<{ expression: IrExpression; name: string }> = [];
  const visit = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'function') return;
    if ('kind' in value && value.kind === 'variable' && 'declarations' in value) {
      for (const variable of value.declarations as IrVariable[]) {
        if (variable.type) context.symbolTypes.set(variable.name, variable.type);
        if (variable.initializer)
          candidates.push({
            expression: variable.initializer,
            name: variable.name,
          });
      }
    }
    if ('kind' in value && value.kind === 'assignment' && 'left' in value && 'right' in value) {
      const assignment = value as Extract<IrExpression, { kind: 'assignment' }>;
      if (assignment.left.kind === 'identifier') {
        candidates.push({
          expression: assignment.right,
          name: assignment.left.name,
        });
      }
    }
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  statements.forEach(visit);
  for (let pass = 0; pass <= candidates.length; pass++) {
    let changed = false;
    for (const candidate of candidates) {
      if (context.symbolTypes.has(candidate.name)) continue;
      const inferred =
        inferEntitySpreadType(candidate.expression, context) ?? inferIrExpressionType(candidate.expression, context);
      if (!inferred) continue;
      context.symbolTypes.set(candidate.name, javaScriptValueType(inferred));
      changed = true;
    }
    if (!changed) break;
  }
}

function emitAnonymousDefinitions(context: EmitContext, exported = false, packageVisible = false): string {
  if (context.anonymousTypes.size === 0) return '';
  const definitions = [...context.anonymousTypes.entries()]
    .filter(([key]) => !context.inheritedAnonymousTypeKeys.has(key))
    .map(([key, name]) => {
      const type = JSON.parse(key) as IrType;
      if (type.kind !== 'anonymous') throw new RustEmissionError(`invalid anonymous type identity ${name}`);
      const fields = flattenStructFields(type, context);
      const parameters = anonymousTypeParameterNames(type, context);
      const generics = parameters.length > 0 ? `<${parameters.join(', ')}>` : '';
      const derivesDefault = fields.every((field) => field.optional || rustTypeSupportsDefault(field.type, context));
      const visibility = exported ? 'pub ' : packageVisible ? 'pub(crate) ' : '';
      return [
        `#[derive(Clone${derivesDefault ? ', Default' : ''})]`,
        `${visibility}struct ${name}${generics} {`,
        indent(
          [
            `${visibility}__flight_identity: std::sync::Arc<()>,`,
            ...fields.map(
              (field) =>
                `${visibility}${safeName(field.name)}: ${
                  field.optional && field.type.kind !== 'nullable'
                    ? `Option<${emitType(field.type, context)}>`
                    : emitType(field.type, context)
                },`,
            ),
          ].join('\n'),
        ),
        '}',
        `impl${generics} PartialEq for ${name}${generics} {`,
        '  fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity) }',
        '}',
      ].join('\n');
    });
  if (definitions.length === 0) return '';
  return `${definitions.join('\n\n')}\n\n`;
}

function anonymousTypeParameterNames(type: Extract<IrType, { kind: 'anonymous' }>, context: EmitContext): string[] {
  return [
    ...(context.anonymousTypeParameters.get(typeKey(type)) ??
      [...context.lexicalTypeParameters].filter((parameter) => typeUsesNamedParameter(type, parameter))),
  ];
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
      const left = item.left as IrExpression;
      if (left.kind === 'element' && left.object.kind === 'new') {
        const buffer = left.object.arguments[0];
        const bufferRoot = buffer ? expressionRootIdentifier(buffer) : undefined;
        if (bufferRoot) names.add(bufferRoot);
      }
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

function clearsPropertyWithoutReading(
  declaration: IrFunctionDeclaration,
  ownerName: string,
  propertyName: string,
): boolean {
  let clears = false;
  let reads = false;
  const matches = (value: unknown): value is Extract<IrExpression, { kind: 'property' }> =>
    Boolean(
      value &&
      typeof value === 'object' &&
      'kind' in value &&
      value.kind === 'property' &&
      'name' in value &&
      value.name === propertyName &&
      'object' in value &&
      expressionRootIdentifier(value.object as IrExpression) === ownerName,
    );
  const visit = (value: unknown): void => {
    if (!value || typeof value !== 'object') return;
    if ('kind' in value && value.kind === 'assignment' && 'left' in value && matches(value.left)) {
      if (
        'operator' in value &&
        value.operator === '=' &&
        'right' in value &&
        value.right &&
        typeof value.right === 'object' &&
        isNullishExpression(value.right as IrExpression)
      ) {
        clears = true;
      } else {
        reads = true;
      }
      if ('right' in value) visit(value.right);
      return;
    }
    if (matches(value)) reads = true;
    for (const child of Object.values(value)) {
      if (Array.isArray(child)) child.forEach(visit);
      else visit(child);
    }
  };
  visit(declaration.body);
  return clears && !reads;
}

function expressionRootIdentifier(value: unknown): string | undefined {
  if (!value || typeof value !== 'object' || !('kind' in value)) return undefined;
  if (value.kind === 'identifier' && 'name' in value && typeof value.name === 'string') return value.name;
  if (value.kind === 'cast' && 'expression' in value) {
    return expressionRootIdentifier(value.expression);
  }
  if (value.kind === 'call' && 'callee' in value) {
    return expressionRootIdentifier(value.callee);
  }
  if ((value.kind === 'element' || value.kind === 'property') && 'object' in value) {
    return expressionRootIdentifier(value.object);
  }
  return undefined;
}

function referencesAnyIdentifier(value: unknown, names: ReadonlySet<string>): boolean {
  if (!value || typeof value !== 'object') return false;
  if (
    'kind' in value &&
    value.kind === 'identifier' &&
    'name' in value &&
    typeof value.name === 'string' &&
    names.has(value.name)
  ) {
    return true;
  }
  return Object.values(value).some((item) =>
    Array.isArray(item)
      ? item.some((child) => referencesAnyIdentifier(child, names))
      : referencesAnyIdentifier(item, names),
  );
}

function capturesParameterInReturnedClosure(
  owner: IrClassDeclaration | IrExpression | IrFunctionDeclaration,
  name: string,
): boolean {
  if (!('body' in owner) || !Array.isArray(owner.body)) return false;
  if (
    owner.body.some(
      (statement) =>
        statement.kind === 'return' &&
        statement.expression?.kind === 'function' &&
        containsIdentifier(statement.expression, name),
    )
  ) {
    return true;
  }
  const captures = (value: unknown): boolean => {
    if (!value || typeof value !== 'object') return false;
    if ('kind' in value && value.kind === 'function') return containsIdentifier(value, name);
    return Object.values(value).some((item) => (Array.isArray(item) ? item.some(captures) : captures(item)));
  };
  return owner.body.some(captures);
}

function storesParameter(owner: IrClassDeclaration | IrExpression | IrFunctionDeclaration, name: string): boolean {
  if (!('body' in owner) || !Array.isArray(owner.body)) return false;
  const visit = (value: unknown): boolean => {
    if (!value || typeof value !== 'object') return false;
    if (
      'kind' in value &&
      value.kind === 'call' &&
      'callee' in value &&
      value.callee &&
      typeof value.callee === 'object' &&
      'kind' in value.callee &&
      value.callee.kind === 'property' &&
      'name' in value.callee &&
      value.callee.name === 'set' &&
      'arguments' in value &&
      Array.isArray(value.arguments) &&
      value.arguments[1] &&
      containsIdentifier(value.arguments[1], name)
    ) {
      return true;
    }
    if (
      'kind' in value &&
      value.kind === 'call' &&
      'callee' in value &&
      value.callee &&
      typeof value.callee === 'object' &&
      'kind' in value.callee &&
      value.callee.kind === 'property' &&
      'name' in value.callee &&
      (value.callee.name === 'push' || value.callee.name === 'unshift' || value.callee.name === 'splice') &&
      'arguments' in value &&
      Array.isArray(value.arguments) &&
      value.arguments.some((argument) => containsIdentifier(argument, name))
    ) {
      return true;
    }
    if (
      'kind' in value &&
      value.kind === 'assignment' &&
      'left' in value &&
      value.left &&
      typeof value.left === 'object' &&
      'kind' in value.left &&
      (value.left.kind === 'element' || value.left.kind === 'property') &&
      'right' in value &&
      containsIdentifier(value.right, name)
    ) {
      return true;
    }
    if (
      'kind' in value &&
      value.kind === 'return' &&
      'expression' in value &&
      value.expression &&
      containsIdentifier(value.expression, name)
    ) {
      return true;
    }
    return Object.values(value).some((item) => (Array.isArray(item) ? item.some(visit) : visit(item)));
  };
  return owner.body.some(visit);
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

function collectNullCheckedIdentifierNames(value: unknown, names: Set<string>): void {
  if (!value || typeof value !== 'object') return;
  if ('kind' in value && value.kind === 'binary' && 'operator' in value) {
    const expression = value as Extract<IrExpression, { kind: 'binary' }>;
    if (['===', '!==', '==', '!='].includes(expression.operator)) {
      if (expression.left.kind === 'identifier' && isNullishExpression(expression.right)) {
        names.add(expression.left.name);
      }
      if (expression.right.kind === 'identifier' && isNullishExpression(expression.left)) {
        names.add(expression.right.name);
      }
    }
  }
  for (const child of Object.values(value)) {
    if (Array.isArray(child)) child.forEach((item) => collectNullCheckedIdentifierNames(item, names));
    else collectNullCheckedIdentifierNames(child, names);
  }
}

function evaluateStaticBoolean(expression: IrExpression, context: EmitContext): boolean | undefined {
  if (expression.kind === 'cast') return evaluateStaticBoolean(expression.expression, context);
  if (expression.kind === 'literal' && typeof expression.value === 'boolean') return expression.value;
  if (expression.kind === 'unary' && expression.operator === '!') {
    const operand = evaluateStaticBoolean(expression.operand, context);
    return operand === undefined ? undefined : !operand;
  }
  if (expression.kind !== 'binary') return undefined;
  if (expression.operator === '&&' || expression.operator === '||') {
    const left = evaluateStaticBoolean(expression.left, context);
    const right = evaluateStaticBoolean(expression.right, context);
    if (expression.operator === '&&') {
      if (left === false || right === false) return false;
      return left === true && right === true ? true : undefined;
    }
    if (left === true || right === true) return true;
    return left === false && right === false ? false : undefined;
  }
  if (!['===', '!==', '==', '!='].includes(expression.operator)) return undefined;
  const staticValue = (value: IrExpression): boolean | null | number | string | undefined => {
    if (value.kind === 'cast') return staticValue(value.expression);
    if (value.kind === 'literal') return value.value;
    if (value.kind === 'identifier' && context.knownNullNames.has(value.name)) return null;
    if (
      value.kind === 'call' &&
      value.callee.kind === 'property' &&
      value.callee.object.kind === 'identifier' &&
      value.callee.object.name === '_Runtime' &&
      value.callee.name === 'typeofGlobal'
    ) {
      return 'undefined';
    }
    if (value.kind === 'unary' && value.operator === 'typeof') {
      const emitted = emitUnary(value, context);
      if (/^"(?:boolean|function|number|object|string|undefined)"(?:\.to_owned\(\))?$/u.test(emitted)) {
        return JSON.parse(emitted.replace(/\.to_owned\(\)$/u, '')) as string;
      }
      if (runtimeGlobalType(value.operand)) return 'undefined';
      const type = inferIrExpressionType(value.operand, context);
      if (type?.kind === 'primitive') {
        if (type.name === 'Bool') return 'boolean';
        if (type.name === 'String') return 'string';
        if (type.name === 'Void') return 'undefined';
        return 'number';
      }
    }
    return undefined;
  };
  const left = staticValue(expression.left);
  const right = staticValue(expression.right);
  if (left === undefined || right === undefined) return undefined;
  const equal = left === right;
  return expression.operator === '===' || expression.operator === '==' ? equal : !equal;
}

function staticallyReachableStatements(statements: readonly IrStatement[], context: EmitContext): IrStatement[] {
  const reachable: IrStatement[] = [];
  for (const statement of statements) {
    if (statement.kind === 'if') {
      let constant: boolean | undefined;
      try {
        constant = evaluateStaticBoolean(statement.condition, context);
      } catch (error) {
        if (!(error instanceof RustEmissionError)) throw error;
      }
      if (constant !== undefined) {
        const selected = constant ? statement.consequent : statement.otherwise;
        if (selected) reachable.push(selected);
        if (selected && statementAlwaysReturns(selected, context)) break;
        continue;
      }
    }
    reachable.push(statement);
    try {
      if (statementAlwaysReturns(statement, context)) break;
    } catch (error) {
      if (!(error instanceof RustEmissionError)) throw error;
    }
  }
  return reachable;
}

function statementAlwaysReturns(statement: IrStatement, context: EmitContext): boolean {
  if (statement.kind === 'return' || statement.kind === 'throw') return true;
  if (statement.kind === 'block') {
    return statement.statements.some((item) => statementAlwaysReturns(item, context));
  }
  if (statement.kind === 'switch') return switchAlwaysReturns(statement, context);
  if (statement.kind === 'try') {
    if (statement.finallyBody && statementAlwaysReturns(statement.finallyBody, context)) return true;
    return Boolean(
      statement.catchBody &&
      statementAlwaysReturns(statement.tryBody, context) &&
      statementAlwaysReturns(statement.catchBody, context),
    );
  }
  if (statement.kind !== 'if') return false;
  const constant = evaluateStaticBoolean(statement.condition, context);
  if (constant === true) return statementAlwaysReturns(statement.consequent, context);
  if (constant === false) return statement.otherwise ? statementAlwaysReturns(statement.otherwise, context) : false;
  return Boolean(
    statement.otherwise &&
    statementAlwaysReturns(statement.consequent, context) &&
    statementAlwaysReturns(statement.otherwise, context),
  );
}

function switchAlwaysReturns(statement: Extract<IrStatement, { kind: 'switch' }>, context: EmitContext): boolean {
  return (
    statement.cases.some((switchCase) => !switchCase.expression) &&
    statement.cases.every((switchCase) => switchCase.statements.some((item) => statementAlwaysReturns(item, context)))
  );
}

function switchAlwaysExits(statement: Extract<IrStatement, { kind: 'switch' }>, context: EmitContext): boolean {
  if (!statement.cases.some((switchCase) => !switchCase.expression)) return false;
  return statement.cases.every((_switchCase, index) =>
    statement.cases
      .slice(index)
      .some((candidate) => candidate.statements.some((item) => statementAlwaysExitsSwitch(item, context))),
  );
}

function statementAlwaysExitsSwitch(statement: IrStatement, context: EmitContext): boolean {
  if (statement.kind === 'break' || statement.kind === 'return' || statement.kind === 'throw') return true;
  if (statement.kind === 'block') {
    return statement.statements.some((item) => statementAlwaysExitsSwitch(item, context));
  }
  if (statement.kind === 'try') {
    if (statement.finallyBody && statementAlwaysExitsSwitch(statement.finallyBody, context)) return true;
    return Boolean(
      statement.catchBody &&
      statementAlwaysExitsSwitch(statement.tryBody, context) &&
      statementAlwaysExitsSwitch(statement.catchBody, context),
    );
  }
  if (statement.kind !== 'if') return statementAlwaysReturns(statement, context);
  const constant = evaluateStaticBoolean(statement.condition, context);
  if (constant === true) return statementAlwaysExitsSwitch(statement.consequent, context);
  if (constant === false) {
    return statement.otherwise ? statementAlwaysExitsSwitch(statement.otherwise, context) : false;
  }
  return Boolean(
    statement.otherwise &&
    statementAlwaysExitsSwitch(statement.consequent, context) &&
    statementAlwaysExitsSwitch(statement.otherwise, context),
  );
}

function containsStatementKind(value: unknown, kind: IrStatement['kind']): boolean {
  if (!value || typeof value !== 'object') return false;
  if ('kind' in value && value.kind === kind) return true;
  return Object.values(value).some((item) =>
    Array.isArray(item) ? item.some((child) => containsStatementKind(child, kind)) : containsStatementKind(item, kind),
  );
}

function containsRegionStatementKind(value: unknown, kind: IrStatement['kind']): boolean {
  if (!value || typeof value !== 'object') return false;
  if ('kind' in value && value.kind === 'function') return false;
  if ('kind' in value && value.kind === kind) return true;
  return Object.values(value).some((item) =>
    Array.isArray(item)
      ? item.some((child) => containsRegionStatementKind(child, kind))
      : containsRegionStatementKind(item, kind),
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
    resolved?.kind === 'function' ||
    resolved?.kind === 'task' ||
    resolved?.kind === 'union' ||
    (resolved?.kind === 'named' &&
      ['ArrayBufferView', 'ByteBuffer', 'FlightRegex', 'RustMap', 'RustSet'].includes(resolved.name)) ||
    (resolved?.kind === 'named' && Boolean(typedArrayType(resolved.name)))
  );
}

function isConcreteCallbackHandleType(type: IrType, context: EmitContext): boolean {
  if (type.kind !== 'named' || context.lexicalTypeParameters.has(type.name)) return false;
  return resolveSemanticType(type, context)?.kind === 'function';
}

function isPlainStringType(type: IrType | undefined, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context) ?? type;
  return resolved?.kind === 'primitive' && resolved.name === 'String';
}

function isStringRepresentedType(type: IrType | undefined, context: EmitContext): boolean {
  if (!type) return false;
  const candidate = type.kind === 'nullable' ? type.inner : type;
  const resolved = resolveSemanticType(candidate, context) ?? candidate;
  return resolved.kind === 'union' && resolved.variants.every((variant) => isPlainStringType(variant, context));
}

function semanticTypesEqual(left: IrType, right: IrType, context: EmitContext): boolean {
  return typeKey(resolveSemanticType(left, context) ?? left) === typeKey(resolveSemanticType(right, context) ?? right);
}

function emitStructuralProjectionArgument(
  source: string,
  actualType: IrType,
  expectedType: IrType,
  context: EmitContext,
): string | undefined {
  const actual = resolveSemanticType(actualType, context);
  const expected = resolveSemanticType(expectedType, context);
  if (actual?.kind === 'union' && expected?.kind === 'anonymous') {
    const projections = actual.variants.map((variant) =>
      emitStructuralProjectionArgument('value', variant, expectedType, context),
    );
    if (projections.some((projection) => !projection)) return undefined;
    const constructor =
      actualType.kind === 'named' ? emitNamedUnionConstructor(actualType, context) : 'crate::FlightUnion2';
    const matchVariant = (variants: readonly IrType[], offset: number, currentConstructor: string): string => {
      const projection = projections[offset]!;
      if (variants.length <= 1) return projection;
      const rest = variants.slice(1);
      return `${currentConstructor}::A(value) => ${projection}, ${currentConstructor}::B(value) => ${
        rest.length === 1
          ? projections[offset + 1]!
          : `match value { ${matchVariant(rest, offset + 1, 'crate::FlightUnion2')} }`
      }`;
    };
    return `match ${parenthesize(source)} { ${matchVariant(actual.variants, 0, constructor)} }`;
  }
  if (actual?.kind !== 'anonymous' || expected?.kind !== 'anonymous') return undefined;
  if (
    emitType(actualType, context) === emitType(expectedType, context) &&
    !isStructuralUtilityType(actualType) &&
    !isStructuralUtilityType(expectedType)
  ) {
    return undefined;
  }
  const actualFields = new Map(semanticStructFields(actualType, context).map((field) => [field.name, field]));
  const expectedFields = semanticStructFields(expectedType, context);
  const openFields = expectedType.kind === 'named' ? context.openInterfaceFields.get(expectedType.name) : undefined;
  const expectedEntity = isNativeEntityType(expectedType, context);
  const sharesEntityRuntime = expectedEntity && isNativeEntityType(actualType, context);
  const expandsEntity =
    sharesEntityRuntime &&
    expectedType.kind === 'named' &&
    actualType.kind === 'named' &&
    expectedType.name !== actualType.name &&
    Boolean(findEntityRuntimeApplication(expectedType, actualType.name, context));
  if (
    expectedFields.some((field) => {
      const actualField = actualFields.get(field.name);
      return (
        (!actualField &&
          !field.optional &&
          !openFields?.has(field.name) &&
          !(expandsEntity && rustTypeSupportsDefault(field.type, context))) ||
        Boolean(actualField && !field.optional && actualField.optional) ||
        Boolean(actualField && !structurallyCompatibleTypes(actualField.type, field.type, context))
      );
    })
  ) {
    return undefined;
  }
  const owner = '__flight_source';
  const fields = expectedFields.flatMap((field) => {
    const actualField = actualFields.get(field.name);
    if (!actualField && openFields?.has(field.name)) return [];
    if (!actualField) {
      if (field.optional) return [`${safeName(field.name)}: None,`];
      if (expandsEntity && rustTypeSupportsDefault(field.type, context)) {
        return [`${safeName(field.name)}: Default::default(),`];
      }
      throw new RustEmissionError(`structural projection is missing required field ${field.name}`);
    }
    const place = `${owner}.${safeName(field.name)}`;
    const actualFieldType = resolveSemanticType(actualField.type, context) ?? actualField.type;
    const expectedFieldType = resolveSemanticType(field.type, context) ?? field.type;
    const value =
      actualFieldType.kind === 'function' &&
      expectedFieldType.kind === 'function' &&
      emitType(actualField.type, context) !== emitType(field.type, context)
        ? emitStructuralFunctionAdapter(place, actualFieldType, expectedFieldType, context)
        : isCopyType(field.type, context)
          ? place
          : `${parenthesize(place)}.clone()`;
    return [
      `${safeName(field.name)}: ${
        field.optional && !actualField.optional && field.type.kind !== 'nullable' ? `Some(${value})` : value
      },`,
    ];
  });
  return `{ let ${owner} = &${parenthesize(source)}; ${emitStructConstructorType(expectedType, context)} {\n${indent(
    [
      '__flight_identity: std::sync::Arc::clone(&' + owner + '.__flight_identity),',
      ...(expectedEntity
        ? [
            sharesEntityRuntime
              ? '__flight_entity_runtime: std::sync::Arc::clone(&' + owner + '.__flight_entity_runtime),'
              : '__flight_entity_runtime: Default::default(),',
            sharesEntityRuntime
              ? expandsEntity
                ? '__flight_entity_snapshot: ' + owner + '.__flight_entity_snapshot.clone(),'
                : '__flight_entity_snapshot: ' +
                  owner +
                  '.__flight_entity_snapshot.clone().or_else(|| Some(std::sync::Arc::new((*' +
                  owner +
                  ').clone()))),'
              : '__flight_entity_snapshot: Default::default(),',
          ]
        : []),
      ...fields,
      ...(openFields ? ['..Default::default()'] : []),
    ].join('\n'),
  )}\n} }`;
}

function emitCollectionProjectionArgument(
  source: string,
  actualType: IrType,
  expectedType: IrType,
  context: EmitContext,
): string | undefined {
  const actual = resolveSemanticType(actualType, context) ?? actualType;
  const expected = resolveSemanticType(expectedType, context) ?? expectedType;
  if (actual.kind === 'nullable' && expected.kind === 'nullable') {
    const projected = emitCollectionProjectionArgument('__flight_value', actual.inner, expected.inner, context);
    return projected ? `${parenthesize(source)}.as_ref().map(|__flight_value| ${projected})` : undefined;
  }
  if (actual.kind !== 'array' || expected.kind !== 'array') return undefined;
  const expectedElement = resolveSemanticType(expected.element, context) ?? expected.element;
  const actualElement = resolveSemanticType(actual.element, context) ?? actual.element;
  let projectedElement: string | undefined;
  if (expectedElement.kind === 'union' && actualElement.kind !== 'union') {
    const variantIndex = expectedElement.variants.findIndex((variant) =>
      semanticTypesEqual(variant, actual.element, context),
    );
    if (variantIndex >= 0) {
      const unionName =
        expected.element.kind === 'named' ? emitNamedUnionConstructor(expected.element, context) : undefined;
      projectedElement = wrapUnionValue(
        '(__flight_value).clone()',
        expectedElement.variants,
        variantIndex,
        context,
        unionName,
      );
    }
  }
  projectedElement ??= emitStructuralProjectionArgument('__flight_value', actual.element, expected.element, context);
  return projectedElement
    ? `${parenthesize(source)}.iter().map(|__flight_value| ${projectedElement}).collect::<Vec<_>>()`
    : undefined;
}

function emitCollectionProjectionExpression(
  expression: IrExpression,
  actualType: IrType,
  expectedType: IrType,
  context: EmitContext,
): string | undefined {
  const probe = emitCollectionProjectionArgument('__flight_collection_projection', actualType, expectedType, context);
  return probe
    ? emitCollectionProjectionArgument(
        emitExpression(expression, context, actualType),
        actualType,
        expectedType,
        context,
      )
    : undefined;
}

function emitStructuralFunctionAdapter(
  source: string,
  actual: Extract<IrType, { kind: 'function' }>,
  expected: Extract<IrType, { kind: 'function' }>,
  context: EmitContext,
): string {
  const parameters = expected.parameters.map(
    (type, index) => `__flight_argument_${String(index)}: ${emitType(type, context)}`,
  );
  const arguments_ = expected.parameters.map((type, index) => {
    const name = `__flight_argument_${String(index)}`;
    const actualType = actual.parameters[index] ?? type;
    return emitStructuralProjectionArgument(name, type, actualType, context) ?? name;
  });
  const call = emitLockedCallbackCall('__flight_callback.clone()', arguments_);
  const result = emitStructuralProjectionArgument(call, actual.returns, expected.returns, context) ?? call;
  const erased = `Box<dyn FnMut(${expected.parameters
    .map((type) => emitType(type, context))
    .join(', ')}) -> ${emitType(expected.returns, context)} + Send + 'static>`;
  return `{ let __flight_callback = ${parenthesize(source)}.clone(); std::sync::Arc::new(std::sync::Mutex::new(Box::new(move |${parameters.join(
    ', ',
  )}| -> ${emitType(expected.returns, context)} { ${result} }) as ${erased})) }`;
}

function isSharedHandleType(type: IrType, context: EmitContext): boolean {
  return type.kind === 'named' && (type.name === 'SignalData' || context.entityRuntimeTypes.has(type.name));
}

function entityRuntimeFieldSlot(runtimeName: string, fieldName: string, context: EmitContext): string {
  const slot = context.entityRuntimeFieldSlots.get(`${runtimeName}\0${fieldName}`);
  if (!slot) {
    const unavailable = context.entityRuntimeUnavailableFields.get(`${runtimeName}\0${fieldName}`);
    throw new RustEmissionError(
      unavailable
        ? `entity runtime field ${fieldName} is unavailable on static receiver ${runtimeName}: ${unavailable}`
        : `entity runtime field ${fieldName} is ambiguous or absent on static receiver ${runtimeName}`,
    );
  }
  return slot;
}

function entityRuntimeStorageField(storage: string, slot: string, fieldName: string): string {
  const owner = slot === 'EntityRuntime' ? storage : `${storage}.${snakeCase(slot)}`;
  return `${owner}.${safeName(fieldName)}`;
}

function entityRuntimeGenericSlotStorageType(
  runtime: Extract<IrType, { kind: 'named' }>,
  slot: string,
  context: EmitContext,
): string | undefined {
  if (!context.entityRuntimeGenericSlotTypes.has(slot)) return undefined;
  const application = findEntityRuntimeApplication(runtime, slot, context);
  if (!application) {
    throw new RustEmissionError(
      `generic entity runtime slot ${slot} has no concrete application from ${typeKey(runtime)}`,
    );
  }
  const parameters = context.namedTypeParameters.get(slot) ?? [];
  const arguments_ = parameters.map((parameter, index) =>
    emitType(
      application.arguments[index] ?? {
        arguments: [],
        kind: 'named',
        name: parameter,
      },
      context,
    ),
  );
  return `crate::${slot}Storage${arguments_.length > 0 ? `<${arguments_.join(', ')}>` : ''}`;
}

function findEntityRuntimeApplication(
  type: Extract<IrType, { kind: 'named' }>,
  target: string,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): Extract<IrType, { kind: 'named' }> | undefined {
  if (type.name === target) return type;
  const key = typeKey(type);
  if (visited.has(key)) return undefined;
  const declaration = context.namedTypes.get(type.name);
  if (!declaration) return undefined;
  const parameters = context.namedTypeParameters.get(type.name) ?? [];
  const substitutions = new Map<string, IrType>(
    parameters.map(
      (parameter, index) =>
        [
          parameter,
          type.arguments[index] ?? {
            arguments: [],
            kind: 'named',
            name: parameter,
          },
        ] as const,
    ),
  );
  const applied = substitutions.size > 0 ? substituteIrType(declaration, substitutions) : declaration;
  if (applied.kind === 'named') {
    return findEntityRuntimeApplication(applied, target, context, new Set([...visited, key]));
  }
  if (applied.kind !== 'anonymous') return undefined;
  for (const base of applied.extends) {
    if (base.kind !== 'named') continue;
    const found = findEntityRuntimeApplication(base, target, context, new Set([...visited, key]));
    if (found) return found;
  }
  return undefined;
}

function isCopyType(type: IrType, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context);
  if (!resolved) return false;
  if (resolved.kind === 'primitive') return resolved.name !== 'String';
  if (resolved.kind === 'nullable') return isCopyType(resolved.inner, context);
  if (resolved.kind === 'named' && (resolved.name === 'FlightSymbol' || context.enumNames.has(resolved.name)))
    return true;
  return false;
}

function rustTypeSupportsDefault(
  type: IrType,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): boolean {
  if (type.kind === 'array' || type.kind === 'nullable' || type.kind === 'dynamic') return true;
  if (type.kind === 'task') return false;
  if (type.kind === 'primitive') return true;
  if (type.kind === 'function' || type.kind === 'union') return false;
  if (type.kind === 'anonymous') {
    return flattenStructFields(type, context).every(
      (field) => field.optional || rustTypeSupportsDefault(field.type, context, visited),
    );
  }
  if (
    ['ByteBuffer', 'FlightCallbackArgs', 'RustMap', 'RustSet'].includes(type.name) ||
    Boolean(typedArrayType(type.name))
  ) {
    return true;
  }
  if (visited.has(type.name)) return false;
  const resolved = context.namedTypes.get(type.name);
  return resolved ? rustTypeSupportsDefault(resolved, context, new Set([...visited, type.name])) : false;
}

function emitNew(
  expression: Extract<IrExpression, { kind: 'new' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const globalType = runtimeConstructorType(expression.callee);
  if (globalType === 'Set') {
    const inferred = inferIrExpressionType(expression, context);
    const resolvedExpected = resolveSemanticType(
      expectedType?.kind === 'nullable' ? expectedType.inner : expectedType,
      context,
    );
    const setType =
      resolvedExpected?.kind === 'named' && resolvedExpected.name === 'RustSet' ? resolvedExpected : inferred;
    const element: IrType =
      setType?.kind === 'named' ? (setType.arguments[0] ?? { kind: 'dynamic' }) : { kind: 'dynamic' };
    const source = expression.arguments[0];
    if (!source) return 'Vec::new()';
    const values = emitExpression(source, context, { element, kind: 'array' });
    return `{ let mut __flight_set = Vec::new(); for __flight_value in ${values} { if !__flight_set.contains(&__flight_value) { __flight_set.push(__flight_value); } } __flight_set }`;
  }
  if (globalType === 'Map' || globalType === 'WeakMap') {
    const inferred = inferIrExpressionType(expression, context);
    const resolvedExpected = resolveSemanticType(
      expectedType?.kind === 'nullable' ? expectedType.inner : expectedType,
      context,
    );
    const mapType =
      resolvedExpected?.kind === 'named' && resolvedExpected.name === 'RustMap' ? resolvedExpected : inferred;
    const source = expression.arguments[0];
    if (!source) return 'Vec::new()';
    const sourceType = inferIrExpressionType(source, context);
    const resolvedSource = resolveSemanticType(sourceType, context) ?? sourceType;
    if (resolvedSource?.kind === 'named' && resolvedSource.name === 'RustMap') {
      return emitExpression(source, context, sourceType);
    }
    const entriesType: IrType | undefined =
      mapType?.kind === 'named' && mapType.name === 'RustMap'
        ? {
            element: {
              arguments: [mapType.arguments[0] ?? { kind: 'dynamic' }, mapType.arguments[1] ?? { kind: 'dynamic' }],
              kind: 'named',
              name: 'RustTuple2',
            },
            kind: 'array',
          }
        : undefined;
    return emitExpression(source, context, entriesType);
  }
  if (globalType === 'WeakSet') {
    return 'Vec::new()';
  }
  if (globalType === 'Error') {
    const message = expression.arguments[0]
      ? emitExpression(expression.arguments[0], context, primitive('String'))
      : 'String::new()';
    return `crate::FlightValue::Error { name: "Error".to_owned(), message: ${message}, stack: None, cause: None }`;
  }
  if (globalType && opaqueHostConstructors.has(globalType)) {
    return 'crate::OpaqueHostValue::Object';
  }
  if (globalType === 'Array') {
    const length = expression.arguments[0] ? emitExpression(expression.arguments[0], context) : '0.0_f64';
    return `vec![Default::default(); ${parenthesize(length)} as usize]`;
  }
  if (globalType === 'ArrayBuffer') {
    const length = expression.arguments[0] ? emitExpression(expression.arguments[0], context) : '0.0_f64';
    return `vec![0_u8; ${parenthesize(length)} as usize]`;
  }
  if (expression.callee.kind === 'identifier' && expression.callee.name === 'Proxy') {
    const target = expression.arguments[0];
    if (!target) throw new RustEmissionError('Proxy construction requires a target');
    const value = emitExpression(target, context);
    return target.kind === 'identifier' && context.borrowedNames.has(target.name) ? `${value}.clone()` : value;
  }
  const resolvedExpected = resolveSemanticType(
    expectedType?.kind === 'nullable' ? expectedType.inner : expectedType,
    context,
  );
  const expectedTypedArray = resolvedExpected?.kind === 'named' ? typedArrayType(resolvedExpected.name) : undefined;
  const typedArray = expectedTypedArray ?? (globalType ? typedArrayType(globalType) : undefined);
  if (typedArray) {
    const argument = expression.arguments[0];
    const argumentType = argument ? inferIrExpressionType(argument, context) : undefined;
    const sourceType = argumentType?.kind === 'nullable' ? argumentType.inner : argumentType;
    if (
      argument &&
      (sourceType?.kind === 'array' || (sourceType?.kind === 'named' && Boolean(typedArrayType(sourceType.name))))
    ) {
      const source = emitExpression(argument, context);
      return `${parenthesize(source)}.iter().map(|value| (*value) as ${typedArray.rust}).collect()`;
    }
    if (argument && sourceType?.kind === 'named' && sourceType.name === 'ByteBuffer') {
      const source = emitExpression(argument, context);
      const width = typedArrayByteWidth(typedArray.rust);
      return width === 1
        ? `${parenthesize(source)}.iter().map(|value| (*value) as ${typedArray.rust}).collect()`
        : `${parenthesize(source)}.chunks_exact(${String(width)}).map(|bytes| ${typedArray.rust}::from_ne_bytes(bytes.try_into().expect("typed-array byte chunk width"))).collect()`;
    }
    const length = argument ? emitExpression(argument, context) : '0.0_f64';
    return `vec![${typedArray.zero}; ${parenthesize(length)} as usize]`;
  }
  throw new RustEmissionError(
    `new-expression Rust lowering is not implemented: ${emitExpression(expression.callee, context)}`,
  );
}

function emitHostConstruct(expression: Extract<IrExpression, { kind: 'hostConstruct' }>, context: EmitContext): string {
  switch (expression.capability) {
    case 'ImageData':
      return emitImageDataConstruct(expression.arguments, context);
    case 'OffscreenCanvas': {
      if (expression.arguments.length !== 2) {
        throw new RustEmissionError('OffscreenCanvas construction requires width and height');
      }
      const width = emitExpression(expression.arguments[0]!, context, primitive('Float'));
      const height = emitExpression(expression.arguments[1]!, context, primitive('Float'));
      return `crate::host_offscreen_canvas(${width}, ${height})`;
    }
    case 'URL': {
      if (expression.arguments.length < 1 || expression.arguments.length > 2) {
        throw new RustEmissionError('URL construction requires a value and optional base');
      }
      const value = emitExpression(expression.arguments[0]!, context, primitive('String'));
      const base = expression.arguments[1]
        ? `Some(${emitExpression(expression.arguments[1], context, primitive('String'))})`
        : 'None';
      return `crate::host_url(${value}, ${base})`;
    }
    default: {
      const unsupported: never = expression.capability;
      throw new RustEmissionError(`native host construction is not implemented: ${String(unsupported)}`);
    }
  }
}

function emitImageDataConstruct(arguments_: IrExpression[], context: EmitContext): string {
  const first = arguments_[0];
  const second = arguments_[1];
  if (!first || !second) {
    throw new RustEmissionError('ImageData construction requires pixels and width or width and height');
  }
  const firstType = inferIrExpressionType(first, context);
  const resolvedFirst = resolveSemanticType(firstType?.kind === 'nullable' ? firstType.inner : firstType, context);
  if (resolvedFirst?.kind === 'primitive' && (resolvedFirst.name === 'Float' || resolvedFirst.name === 'Int')) {
    if (arguments_.length !== 2) {
      throw new RustEmissionError('ImageData dimension construction with settings is not implemented');
    }
    return `crate::host_image_data(crate::FlightImageDataRequest::Dimensions { width: ${emitExpression(first, context, primitive('Float'))}, height: ${emitExpression(second, context, primitive('Float'))} })`;
  }
  const pixels = resolvedFirst?.kind === 'named' ? typedArrayType(resolvedFirst.name) : undefined;
  if (!pixels || pixels.rust !== 'u8') {
    throw new RustEmissionError('ImageData pixel construction requires a statically typed byte array');
  }
  if (arguments_.length > 3) {
    throw new RustEmissionError('ImageData pixel construction with settings is not implemented');
  }
  const byteArrayType = {
    arguments: [],
    kind: 'named' as const,
    name: 'Uint8ClampedArray',
  };
  const width = emitExpression(second, context, primitive('Float'));
  const heightExpression = arguments_[2];
  const heightType = heightExpression ? inferIrExpressionType(heightExpression, context) : undefined;
  const height =
    !heightExpression || isNullishExpression(heightExpression)
      ? 'None'
      : heightType?.kind === 'nullable'
        ? emitExpression(heightExpression, context, heightType)
        : `Some(${emitExpression(heightExpression, context, primitive('Float'))})`;
  if (firstType?.kind === 'nullable') {
    if (!heightExpression || heightType?.kind === 'nullable' || isNullishExpression(heightExpression)) {
      throw new RustEmissionError('nullable ImageData pixels require a non-null height for dimension fallback');
    }
    const data = emitExpression(first, context, firstType);
    const dimensionsHeight = emitExpression(heightExpression, context, primitive('Float'));
    return `{ let __flight_data = ${data}; let __flight_width = ${width}; let __flight_height = ${dimensionsHeight}; match __flight_data { Some(data) => crate::host_image_data(crate::FlightImageDataRequest::Pixels { data, width: __flight_width, height: Some(__flight_height) }), None => crate::host_image_data(crate::FlightImageDataRequest::Dimensions { width: __flight_width, height: __flight_height }) } }`;
  }
  const data = emitExpression(first, context, byteArrayType);
  return `crate::host_image_data(crate::FlightImageDataRequest::Pixels { data: ${data}, width: ${width}, height: ${height} })`;
}

function emitObject(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const entityRuntimeProperties = expression.properties.filter(
    (property) =>
      property.kind === 'computedProperty' &&
      property.key.kind === 'identifier' &&
      property.key.name === 'EntityRuntimeKey',
  );
  const entityRuntimeProperty = entityRuntimeProperties.length === 1 ? entityRuntimeProperties[0] : undefined;
  const structuralExpression: typeof expression = {
    ...expression,
    properties: expression.properties.filter((property) => !entityRuntimeProperties.includes(property)),
  };
  const nullable = expectedType?.kind === 'nullable';
  const contextualTarget = nullable ? expectedType.inner : expectedType;
  const onlySpread = expression.properties.length === 1 ? expression.properties[0] : undefined;
  const onlySpreadType =
    onlySpread?.kind === 'spread' ? inferIrExpressionType(onlySpread.expression, context) : undefined;
  const nativeEntitySpread =
    onlySpread?.kind === 'spread' && Boolean(onlySpreadType && isNativeEntityType(onlySpreadType, context));
  if (onlySpread?.kind === 'spread' && !nativeEntitySpread) {
    return `${parenthesize(emitExpression(onlySpread.expression, context))}.clone()`;
  }
  const target =
    selectDeclaredObjectType(structuralExpression, contextualTarget, context) ??
    (nativeEntitySpread ? onlySpreadType : undefined) ??
    inferNamedStructuralObjectType(structuralExpression, context) ??
    inferObjectSpreadMapType(structuralExpression, context) ??
    synthesizeObjectLiteralType(structuralExpression, context);
  if (target?.kind === 'anonymous') registerInferredObjectType(target, context);
  const resolved = resolveSemanticType(target, context);
  if (
    entityRuntimeProperties.length > 1 ||
    (entityRuntimeProperties.length === 1 && !(target?.kind === 'named' && context.entityTypes.has(target.name)))
  ) {
    rejectEntityRuntimeStorage();
  }
  if (onlySpread?.kind === 'spread' && target?.kind === 'named' && context.entityTypeParameters.has(target.name)) {
    const source = emitExpression(onlySpread.expression, context, target);
    const root = expressionRootIdentifier(onlySpread.expression);
    const reference = root && context.borrowedNames.has(root) ? source : `&${parenthesize(source)}`;
    const value = `${entityTraitTypePath(context)}::__flight_fresh_clone(${reference})`;
    return nullable ? `Some(${value})` : value;
  }
  if (resolved?.kind === 'named' && resolved.name === 'RustMap') {
    const keyType = resolved.arguments[0] ?? primitive('String');
    const valueType = resolved.arguments[1] ?? { kind: 'dynamic' };
    if (!expression.properties.some((property) => property.kind === 'spread')) {
      const entries = expression.properties.map((property) => {
        if (property.kind === 'spread') throw new RustEmissionError('unreachable record spread');
        const key =
          property.kind === 'computedProperty'
            ? emitExpression(property.key, context, keyType)
            : emitExpression({ kind: 'literal', value: property.name }, context, keyType);
        return `__flight_record.push((${key}, ${emitExpression(property.value, context, valueType)}));`;
      });
      const value = `{ let mut __flight_record = Vec::new(); ${entries.join(' ')} __flight_record }`;
      return nullable ? `Some(${value})` : value;
    }
    const propertyContexts = contextsPreservingNamesUsedLater(expression.properties, context);
    const operations: string[] = [];
    for (const [index, property] of expression.properties.entries()) {
      const propertyContext = propertyContexts[index] ?? context;
      if (property.kind === 'spread') {
        const spread = `__flight_spread_${String(index)}`;
        operations.push(
          `let ${spread} = ${emitExpression(property.expression, propertyContext, target)}; for (__flight_key, __flight_value) in ${spread}.iter().cloned() { if let Some((_, __flight_existing)) = __flight_record.iter_mut().find(|(key, _)| key == &__flight_key) { *__flight_existing = __flight_value; } else { __flight_record.push((__flight_key, __flight_value)); } }`,
        );
        continue;
      }
      const key =
        property.kind === 'computedProperty'
          ? emitExpression(property.key, propertyContext, keyType)
          : emitExpression({ kind: 'literal', value: property.name }, propertyContext, keyType);
      const keyName = `__flight_key_${String(index)}`;
      const valueName = `__flight_value_${String(index)}`;
      operations.push(
        `let ${keyName} = ${key}; let ${valueName} = ${emitExpression(property.value, propertyContext, valueType)}; if let Some((_, __flight_existing)) = __flight_record.iter_mut().find(|(key, _)| key == &${keyName}) { *__flight_existing = ${valueName}; } else { __flight_record.push((${keyName}, ${valueName})); }`,
      );
    }
    const value = `{ let mut __flight_record = Vec::new(); ${operations.join(' ')} __flight_record }`;
    return nullable ? `Some(${value})` : value;
  }
  if (expression.properties.length === 0 && (!target || resolved?.kind === 'dynamic')) {
    return `${dynamicValuePath(resolved)}::Object`;
  }
  if (
    expression.properties.length === 0 &&
    target?.kind === 'named' &&
    !context.namedTypes.has(target.name) &&
    resolved?.kind !== 'anonymous'
  ) {
    return `panic!("cannot construct an untyped ${target.name} without a native value")`;
  }
  if (resolved?.kind !== 'anonymous' || !target) {
    throw new RustEmissionError(
      `object literal requires an inferred structural type (target=${target ? typeKey(target) : 'unknown'}, properties=${expression.properties
        .map((property) => (property.kind === 'property' ? property.name : property.kind))
        .join(',')})`,
    );
  }
  const name = emitStructConstructorType(target, context);
  const openFields = target.kind === 'named' ? context.openInterfaceFields.get(target.name) : undefined;
  const fields = new Map(flattenStructFields(resolved, context).map((field) => [field.name, field]));
  if (target.kind === 'named' && context.entityRuntimeTypes.has(target.name)) {
    const assignments = structuralExpression.properties.map((property) => {
      if (property.kind !== 'property') {
        throw new RustEmissionError(`entity runtime object ${property.kind} lowering is not implemented`);
      }
      const field = fields.get(property.name);
      if (!field) throw new RustEmissionError(`entity runtime field ${property.name} is not in the source closure`);
      const value = emitExpression(property.value, context, field.type);
      const stored =
        field.optional && field.type.kind !== 'nullable' && !isNullishExpression(property.value)
          ? `Some(${value})`
          : value;
      const slot = entityRuntimeFieldSlot(target.name, property.name, context);
      const assigned = context.entityRuntimeLateFields.has(`${slot}\0${property.name}`) ? `Some(${stored})` : stored;
      const genericStorage = entityRuntimeGenericSlotStorageType(target, slot, context);
      return genericStorage
        ? `{ let __flight_slot = __flight_runtime.__flight_generic_slot::<${genericStorage}>(); __flight_slot.lock().unwrap().${safeName(property.name)} = ${assigned}; }`
        : `{ __flight_runtime.inner.lock().unwrap().${entityRuntimeStorageField('', slot, property.name).replace(/^\./u, '')} = ${assigned}; }`;
    });
    const value = `{ let __flight_runtime = ${entityRuntimeTypePath(context)}::default(); ${assignments.join(' ')} __flight_runtime }`;
    return nullable ? `Some(${value})` : value;
  }
  const structuralSpreads = structuralExpression.properties.flatMap((property, index) => {
    if (property.kind !== 'spread') return [];
    const sourceType = inferIrExpressionType(property.expression, context);
    const resolvedSource = resolveSemanticType(sourceType, context) ?? sourceType;
    const structuralSource =
      resolvedSource?.kind === 'nullable'
        ? (resolveSemanticType(resolvedSource.inner, context) ?? resolvedSource.inner)
        : resolvedSource;
    return structuralSource?.kind === 'anonymous'
      ? [
          {
            fields: new Set(flattenStructFields(structuralSource, context).map((field) => field.name)),
            index,
            name: `__flight_spread_${String(index)}`,
            property,
          },
        ]
      : [];
  });
  if (structuralSpreads.length > 0) {
    const values = new Map<
      string,
      { expression: IrExpression; kind: 'property' } | { field: IrTypeField; kind: 'spread'; name: string }
    >();
    structuralExpression.properties.forEach((property, index) => {
      if (property.kind === 'property') {
        if (fields.has(property.name))
          values.set(property.name, {
            expression: property.value,
            kind: 'property',
          });
        return;
      }
      const spread = structuralSpreads.find((candidate) => candidate.index === index);
      if (!spread) return;
      for (const fieldName of spread.fields) {
        const field = fields.get(fieldName);
        if (field) values.set(fieldName, { field, kind: 'spread', name: spread.name });
      }
    });
    const properties = [...fields.values()].map((field) => {
      const value = values.get(field.name);
      if (!value) {
        if (field.optional) return `${safeName(field.name)}: None,`;
        if (openFields?.has(field.name)) return undefined;
        throw new RustEmissionError(`object field ${field.name} is not initialized by its structural spreads`);
      }
      if (value.kind === 'property') {
        const stored =
          target.kind === 'named'
            ? emitRecursiveStructFieldStorageValue(target.name, field, value.expression, context)
            : undefined;
        const emitted = stored ?? emitExpression(value.expression, context, field.type);
        return `${safeName(field.name)}: ${
          field.optional &&
          field.type.kind !== 'nullable' &&
          !recursiveStructFieldStorage(field.type, target.kind === 'named' ? target.name : '', field.optional) &&
          !isNullishExpression(value.expression)
            ? `Some(${emitted})`
            : emitted
        },`;
      }
      const place = `${value.name}.${safeName(field.name)}`;
      return `${safeName(field.name)}: ${isCopyType(value.field.type, context) ? place : `${parenthesize(place)}.clone()`},`;
    });
    const bindings = structuralSpreads.map((spread) => {
      const sourceType = inferIrExpressionType(spread.property.expression, context);
      const resolvedSource = resolveSemanticType(sourceType, context) ?? sourceType;
      const source = emitExpression(spread.property.expression, context, sourceType);
      return `let ${spread.name} = ${
        resolvedSource?.kind === 'nullable' ? `${parenthesize(source)}.unwrap_or_default()` : source
      };`;
    });
    const entitySpread = structuralSpreads.find((spread) => {
      const sourceType = inferIrExpressionType(spread.property.expression, context);
      const sourceEntityType = sourceType?.kind === 'nullable' ? sourceType.inner : sourceType;
      return Boolean(sourceEntityType && isNativeEntityType(sourceEntityType, context));
    });
    const entityRuntimeInitializer =
      target.kind === 'named' && context.entityTypes.has(target.name)
        ? entityRuntimeProperty?.kind === 'computedProperty'
          ? `std::sync::Arc::new(std::sync::Mutex::new(Some(${emitExpression(entityRuntimeProperty.value, context, {
              arguments: [],
              kind: 'named',
              name: 'EntityRuntime',
            })})))`
          : entitySpread
            ? `std::sync::Arc::new(std::sync::Mutex::new(${entitySpread.name}.__flight_entity_runtime.lock().unwrap().clone()))`
            : 'Default::default()'
        : undefined;
    const entitySnapshotInitializer =
      target.kind === 'named' && context.entityTypes.has(target.name)
        ? entitySpread
          ? (() => {
              const sourceType = inferIrExpressionType(entitySpread.property.expression, context);
              const sourceEntityType = sourceType?.kind === 'nullable' ? sourceType.inner : sourceType;
              const expandsEntity =
                sourceEntityType?.kind === 'named' &&
                target.name !== sourceEntityType.name &&
                Boolean(findEntityRuntimeApplication(target, sourceEntityType.name, context));
              const snapshot = `${entitySpread.name}.__flight_entity_snapshot.clone()`;
              return expandsEntity
                ? snapshot
                : `${snapshot}.or_else(|| Some(std::sync::Arc::new(${entitySpread.name}.clone())))`;
            })()
          : 'Default::default()'
        : undefined;
    const value = `{ ${bindings.join(' ')} ${name} {\n${indent(
      [
        '__flight_identity: std::sync::Arc::new(()),',
        ...(entityRuntimeInitializer ? [`__flight_entity_runtime: ${entityRuntimeInitializer},`] : []),
        ...(entitySnapshotInitializer ? [`__flight_entity_snapshot: ${entitySnapshotInitializer},`] : []),
        ...properties.filter((property): property is string => Boolean(property)),
        ...(openFields ? ['..Default::default()'] : []),
      ].join('\n'),
    )}\n} }`;
    return nullable ? `Some(${value})` : value;
  }
  const initialized = new Set<string>();
  const spreads: string[] = [];
  const entitySpreadProperty =
    target.kind === 'named' && context.entityTypes.has(target.name)
      ? structuralExpression.properties.find((property) => {
          if (property.kind !== 'spread') return false;
          const sourceType = inferIrExpressionType(property.expression, context);
          return sourceType?.kind === 'named' && context.entityTypes.has(sourceType.name);
        })
      : undefined;
  if (target.kind === 'named' && target.name === 'SignalData') {
    const arguments_ = [...fields.values()].map((field) => {
      const property = structuralExpression.properties.find(
        (candidate) => candidate.kind === 'property' && candidate.name === field.name,
      );
      if (!property || property.kind !== 'property') {
        if (field.optional) return 'None';
        throw new RustEmissionError(`shared object field ${field.name} is not initialized`);
      }
      const value = emitExpression(property.value, context, field.type);
      return field.optional && field.type.kind !== 'nullable' && !isNullishExpression(property.value)
        ? `Some(${value})`
        : value;
    });
    const value = `${name}::new(${arguments_.join(', ')})`;
    return nullable ? `Some(${value})` : value;
  }
  const orderedPropertyContexts = contextsPreservingNamesUsedLater(structuralExpression.properties, context);
  const properties = structuralExpression.properties.flatMap((property, index) => {
    const propertyContext = orderedPropertyContexts[index] ?? context;
    if (property.kind === 'spread') {
      spreads.push(
        property === entitySpreadProperty
          ? '..__flight_entity_spread.clone()'
          : `..${parenthesize(emitExpression(property.expression, propertyContext, target))}.clone()`,
      );
      return [];
    }
    if (property.kind !== 'property') {
      throw new RustEmissionError(`object ${property.kind} Rust lowering is not implemented`);
    }
    const field = fields.get(property.name);
    if (!field) throw new RustEmissionError(`object field ${property.name} is not present in structural type`);
    initialized.add(property.name);
    const value =
      target.kind === 'named'
        ? (emitRecursiveStructFieldStorageValue(target.name, field, property.value, propertyContext) ??
          emitExpression(property.value, propertyContext, field.type))
        : emitExpression(property.value, propertyContext, field.type);
    return [
      `${safeName(property.name)}: ${
        field.optional &&
        field.type.kind !== 'nullable' &&
        !recursiveStructFieldStorage(field.type, target.kind === 'named' ? target.name : '', field.optional) &&
        !isNullishExpression(property.value)
          ? `Some(${value})`
          : value
      },`,
    ];
  });
  for (const field of resolved.fields) {
    if (spreads.length > 0 || initialized.has(field.name)) continue;
    if (field.optional) properties.push(`${safeName(field.name)}: None,`);
    else if (!openFields?.has(field.name) && rustTypeSupportsDefault(field.type, context)) {
      properties.push(`${safeName(field.name)}: Default::default(),`);
    } else if (!openFields?.has(field.name)) {
      throw new RustEmissionError(`object field ${field.name} is not initialized and has no Rust default`);
    }
  }
  if (spreads.length > 1) throw new RustEmissionError('multiple object spreads require ordered Rust lowering');
  if (spreads.length === 1 && target.kind === 'named' && context.entityTypes.has(target.name)) {
    properties.unshift('__flight_identity: std::sync::Arc::new(()),');
    properties.unshift(
      entitySpreadProperty?.kind === 'spread'
        ? '__flight_entity_snapshot: __flight_entity_spread.__flight_entity_snapshot.clone(),'
        : '__flight_entity_snapshot: Default::default(),',
    );
    if (entityRuntimeProperty?.kind === 'computedProperty') {
      properties.unshift(
        `__flight_entity_runtime: std::sync::Arc::new(std::sync::Mutex::new(Some(${emitExpression(
          entityRuntimeProperty.value,
          context,
          { arguments: [], kind: 'named', name: 'EntityRuntime' },
        )}))),`,
      );
    } else if (entitySpreadProperty?.kind === 'spread') {
      properties.unshift(
        '__flight_entity_runtime: std::sync::Arc::new(std::sync::Mutex::new(__flight_entity_spread.__flight_entity_runtime.lock().unwrap().clone())),',
      );
    }
  }
  if (spreads.length === 0) {
    properties.unshift(
      '__flight_identity: std::sync::Arc::new(()),',
      ...(target.kind === 'named' && context.entityTypes.has(target.name)
        ? [
            '__flight_entity_snapshot: Default::default(),',
            entityRuntimeProperty?.kind === 'computedProperty'
              ? `__flight_entity_runtime: std::sync::Arc::new(std::sync::Mutex::new(Some(${emitExpression(
                  entityRuntimeProperty.value,
                  context,
                  { arguments: [], kind: 'named', name: 'EntityRuntime' },
                )}))),`
              : '__flight_entity_runtime: Default::default(),',
          ]
        : []),
    );
  }
  const value = `${entitySpreadProperty?.kind === 'spread' ? `{ let __flight_entity_spread = ${emitExpression(entitySpreadProperty.expression, context, target)}; ` : ''}${name} {\n${indent(
    [...properties, ...spreads, ...(openFields && spreads.length === 0 ? ['..Default::default()'] : [])].join('\n'),
  )}\n}${entitySpreadProperty?.kind === 'spread' ? ' }' : ''}`;
  return nullable ? `Some(${value})` : value;
}

function contextsPreservingNamesUsedLater(values: readonly unknown[], context: EmitContext): EmitContext[] {
  const contexts = Array.from({ length: values.length }, () => context);
  const namesUsedLater = new Set<string>();
  for (let index = values.length - 1; index >= 0; index--) {
    const namesUsedNow = new Set<string>();
    collectIdentifierNames(values[index], namesUsedNow);
    const consumedName = directlyConsumedObjectPropertyIdentifier(values[index]);
    const preservedNames = new Set([
      ...context.preservedNames,
      ...(consumedName && namesUsedLater.has(consumedName) ? [consumedName] : []),
    ]);
    contexts[index] = preservedNames.size === context.preservedNames.size ? context : { ...context, preservedNames };
    namesUsedNow.forEach((name) => namesUsedLater.add(name));
  }
  return contexts;
}

function identifierNamesUsedLater(values: readonly unknown[]): ReadonlySet<string>[] {
  const result = Array.from<unknown, ReadonlySet<string>>({ length: values.length }, () => new Set());
  const names = new Set<string>();
  for (let index = values.length - 1; index >= 0; index--) {
    result[index] = new Set(names);
    collectIdentifierNames(values[index], names);
  }
  return result;
}

function contextPreservingReferencedNames(
  value: unknown,
  namesUsedLater: ReadonlySet<string>,
  context: EmitContext,
): EmitContext {
  const namesConsumedNow = new Set<string>();
  collectConsumedIdentifierNames(value, namesConsumedNow);
  const preservedNames = new Set([
    ...context.preservedNames,
    ...[...namesConsumedNow].filter((name) => namesUsedLater.has(name)),
  ]);
  return preservedNames.size === context.preservedNames.size ? context : { ...context, preservedNames };
}

function contextPreservingLoopReads(value: unknown, context: EmitContext): EmitContext {
  const names = new Set<string>();
  collectConsumedIdentifierNames(value, names);
  const preservedNames = new Set([...context.preservedNames, ...names]);
  return preservedNames.size === context.preservedNames.size ? context : { ...context, preservedNames };
}

function collectConsumedIdentifierNames(value: unknown, names: Set<string>, consumed = false): void {
  if (!value || typeof value !== 'object') return;
  if ('kind' in value && value.kind === 'identifier') {
    if (consumed && 'name' in value && typeof value.name === 'string') names.add(value.name);
    return;
  }
  if ('kind' in value) {
    switch (value.kind) {
      case 'array':
        if ('elements' in value && Array.isArray(value.elements)) {
          value.elements.forEach((item) => collectConsumedIdentifierNames(item, names, true));
        }
        return;
      case 'assignment':
        if ('left' in value) collectConsumedIdentifierNames(value.left, names);
        if ('right' in value) collectConsumedIdentifierNames(value.right, names, true);
        return;
      case 'binary':
        if (
          'operator' in value &&
          typeof value.operator === 'string' &&
          (value.operator === '??' || value.operator === '??undefined') &&
          'right' in value &&
          value.right &&
          typeof value.right === 'object' &&
          isNullishExpression(value.right as IrExpression)
        ) {
          if ('left' in value) collectConsumedIdentifierNames(value.left, names, true);
          return;
        }
        break;
      case 'await':
      case 'cast':
      case 'spread':
        if ('expression' in value) collectConsumedIdentifierNames(value.expression, names, consumed);
        return;
      case 'call':
      case 'new':
        if ('callee' in value) collectConsumedIdentifierNames(value.callee, names);
        if ('arguments' in value && Array.isArray(value.arguments)) {
          value.arguments.forEach((argument) => collectConsumedIdentifierNames(argument, names, true));
        }
        return;
      case 'conditional':
        if ('condition' in value) collectConsumedIdentifierNames(value.condition, names);
        if ('whenTrue' in value) collectConsumedIdentifierNames(value.whenTrue, names, consumed);
        if ('whenFalse' in value) collectConsumedIdentifierNames(value.whenFalse, names, consumed);
        return;
      case 'object':
        if ('properties' in value && Array.isArray(value.properties)) {
          for (const property of value.properties) {
            if (!property || typeof property !== 'object') continue;
            if ('value' in property) collectConsumedIdentifierNames(property.value, names, true);
            if ('expression' in property) collectConsumedIdentifierNames(property.expression, names, true);
          }
        }
        return;
      case 'return':
      case 'throw':
        if ('expression' in value) collectConsumedIdentifierNames(value.expression, names, true);
        return;
      case 'variable':
        if ('declarations' in value && Array.isArray(value.declarations)) {
          for (const declaration of value.declarations) {
            if (declaration && typeof declaration === 'object' && 'initializer' in declaration) {
              collectConsumedIdentifierNames(declaration.initializer, names, true);
            }
          }
        }
        return;
    }
  }
  for (const child of Object.values(value)) {
    if (Array.isArray(child)) child.forEach((item) => collectConsumedIdentifierNames(item, names));
    else collectConsumedIdentifierNames(child, names);
  }
}

function directlyConsumedObjectPropertyIdentifier(value: unknown): string | undefined {
  if (
    !value ||
    typeof value !== 'object' ||
    !('kind' in value) ||
    value.kind !== 'property' ||
    !('value' in value) ||
    !value.value ||
    typeof value.value !== 'object'
  ) {
    return undefined;
  }
  const expression = unwrapCasts(value.value as IrExpression);
  return expression.kind === 'identifier' ? expression.name : undefined;
}

function collectIdentifierNames(value: unknown, names: Set<string>): void {
  if (!value || typeof value !== 'object') return;
  if ('kind' in value && value.kind === 'identifier' && 'name' in value && typeof value.name === 'string') {
    names.add(value.name);
  }
  for (const child of Object.values(value)) {
    if (Array.isArray(child)) child.forEach((item) => collectIdentifierNames(item, names));
    else collectIdentifierNames(child, names);
  }
}

function inferNamedStructuralObjectType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
): IrType | undefined {
  const signature = objectLiteralPropertySignature(expression, context);
  if (!signature || signature.names.size === 0) return undefined;
  const { names, unknownSpread } = signature;
  const matches = [...context.namedTypes.entries()].flatMap(([name, type]) => {
    if (!context.localTypeNames.has(name) && !context.importedModules.has(name)) return [];
    if ((context.namedTypeParameters.get(name)?.length ?? 0) > 0) return [];
    if (type.kind !== 'anonymous') return [];
    const fields = flattenStructFields(type, context);
    if (
      (!unknownSpread && fields.some((field) => !field.optional && !names.has(field.name))) ||
      [...names].some((fieldName) => !fields.some((field) => field.name === fieldName))
    ) {
      return [];
    }
    return [
      {
        missing: fields.filter((field) => !names.has(field.name)).length,
        type: { arguments: [], kind: 'named' as const, name },
      },
    ];
  });
  const bestScore = Math.min(...matches.map((match) => match.missing));
  const best = matches.filter((match) => match.missing === bestScore);
  const localMatches = best.filter((match) => context.localTypeNames.has(match.type.name));
  const selected = localMatches.length > 0 ? localMatches : best;
  const identities = new Map(
    selected.map((match) => [typeKey(resolveSemanticType(match.type, context) ?? match.type), match.type]),
  );
  return identities.size === 1 ? [...identities.values()][0] : undefined;
}

function inferContextualObjectType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
  declared?: IrType,
): IrType | undefined {
  const contextual = selectDeclaredObjectType(expression, declared, context);
  if (contextual) return contextual;
  const semantic = inferNamedStructuralObjectType(expression, context);
  if (semantic) return semantic;
  const map = inferObjectSpreadMapType(expression, context);
  if (map) return map;
  return synthesizeObjectLiteralType(expression, context);
}

function inferObjectSpreadMapType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
): Extract<IrType, { kind: 'named' }> | undefined {
  const spreadTypes = expression.properties.flatMap((property) => {
    if (property.kind !== 'spread') return [];
    const type = inferIrExpressionType(property.expression, context);
    const resolved = resolveSemanticType(type, context) ?? type;
    return resolved?.kind === 'named' && resolved.name === 'RustMap' ? [resolved] : [];
  });
  const spreadCount = expression.properties.filter((property) => property.kind === 'spread').length;
  const target = spreadTypes[0];
  if (!target || spreadTypes.length !== spreadCount) return undefined;
  if (!spreadTypes.every((type) => semanticTypesEqual(type, target, context))) return undefined;

  const keyType = target.arguments[0] ?? primitive('String');
  const valueType = target.arguments[1] ?? { kind: 'dynamic' as const };
  const resolvedKey = resolveSemanticType(keyType, context) ?? keyType;
  const resolvedValue = resolveSemanticType(valueType, context) ?? valueType;
  for (const property of expression.properties) {
    if (property.kind === 'spread') continue;
    if (property.kind === 'computedProperty') {
      const actualKey = inferIrExpressionType(property.key, context);
      if (!actualKey || (resolvedKey.kind !== 'dynamic' && !semanticTypesEqual(actualKey, keyType, context))) {
        return undefined;
      }
    } else if (
      !(resolvedKey.kind === 'dynamic' || (resolvedKey.kind === 'primitive' && resolvedKey.name === 'String'))
    ) {
      return undefined;
    }
    const actualValue = inferIrExpressionType(property.value, context);
    if (!actualValue || (resolvedValue.kind !== 'dynamic' && !semanticTypesEqual(actualValue, valueType, context))) {
      return undefined;
    }
  }
  return target;
}

function selectDeclaredObjectType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  declared: IrType | undefined,
  context: EmitContext,
): IrType | undefined {
  const target = declared?.kind === 'nullable' ? declared.inner : declared;
  if (!target || target.kind === 'dynamic') return undefined;
  if (target.kind === 'named' && context.entityRuntimeTypes.has(target.name)) return target;
  if (expression.properties.length === 0 && target.kind === 'named' && context.lexicalTypeParameters.has(target.name)) {
    return target;
  }
  const resolved = resolveSemanticType(target, context) ?? target;
  if (resolved.kind === 'union') {
    const matches = resolved.variants.filter((variant) => objectLiteralMatchesType(expression, variant, context));
    const identities = new Map(matches.map((match) => [typeKey(resolveSemanticType(match, context) ?? match), match]));
    return identities.size === 1 ? [...identities.values()][0] : undefined;
  }
  if (resolved.kind === 'anonymous') {
    if (expression.properties.length === 0) return target;
    const signature = objectLiteralPropertySignature(expression, context);
    const fields = flattenStructFields(resolved, context);
    const directNames = expression.properties.flatMap((property) =>
      property.kind === 'property' ? [property.name] : [],
    );
    const discriminantsMatch = fields.every((field) => {
      if (field.discriminantValue === undefined) return true;
      const property = expression.properties.find(
        (candidate) => candidate.kind === 'property' && candidate.name === field.name,
      );
      if (!property || property.kind !== 'property') return Boolean(signature?.unknownSpread);
      const value = constantExpressionValue(property.value, context);
      return value === undefined || value === field.discriminantValue;
    });
    if (signature && discriminantsMatch && directNames.every((name) => fields.some((field) => field.name === name))) {
      return target;
    }
  }
  return objectLiteralMatchesType(expression, target, context) ? target : undefined;
}

function synthesizeObjectLiteralType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
): Extract<IrType, { kind: 'anonymous' }> | undefined {
  const fields = new Map<string, IrTypeField>();
  for (const property of expression.properties) {
    if (property.kind === 'computedProperty') return undefined;
    if (property.kind === 'spread') {
      const spread = inferIrExpressionType(property.expression, context);
      const resolved = resolveSemanticType(spread, context) ?? spread;
      if (resolved?.kind !== 'anonymous') return undefined;
      for (const field of flattenStructFields(resolved, context)) fields.set(field.name, field);
      continue;
    }
    const inferred =
      inferIrExpressionType(property.value, context) ??
      inferStaticExpressionType(property.value) ??
      (property.value.kind === 'array'
        ? ({ element: { kind: 'dynamic' }, kind: 'array' } as const)
        : ({ kind: 'dynamic' } as const));
    const type = javaScriptValueType(inferred);
    fields.set(property.name, { name: property.name, optional: false, type });
  }
  return {
    extends: [],
    fields: [...fields.values()].sort((left, right) => left.name.localeCompare(right.name)),
    kind: 'anonymous',
  };
}

function registerInferredObjectType(type: Extract<IrType, { kind: 'anonymous' }>, context: EmitContext): void {
  const anonymousTypes = context.anonymousTypes as Map<string, string>;
  for (const anonymous of collectAnonymousTypes(type)) {
    const key = typeKey(anonymous);
    if (anonymousTypes.has(key)) continue;
    anonymousTypes.set(key, `${pascalCase(context.anonymousTypeOwner)}SynthesizedRecord${stableTypeIdentity(key)}`);
  }
}

function stableTypeIdentity(key: string): string {
  let hash = 0x81_1c_9d_c5;
  for (let index = 0; index < key.length; index++) {
    hash = Math.imul(hash ^ key.charCodeAt(index), 0x01_00_01_93);
  }
  return String(hash >>> 0);
}

function objectLiteralPropertySignature(
  expression: Extract<IrExpression, { kind: 'object' }>,
  context: EmitContext,
): { names: ReadonlySet<string>; unknownSpread: boolean } | undefined {
  const names = new Set<string>();
  let unknownSpread = false;
  for (const property of expression.properties) {
    if (property.kind === 'computedProperty') return undefined;
    if (property.kind === 'property') {
      names.add(property.name);
      continue;
    }
    const spread = inferIrExpressionType(property.expression, context);
    const resolved = resolveSemanticType(spread, context) ?? spread;
    if (resolved?.kind !== 'anonymous') {
      unknownSpread = true;
      continue;
    }
    for (const field of flattenStructFields(resolved, context)) names.add(field.name);
  }
  return { names, unknownSpread };
}

function objectLiteralMatchesType(
  expression: Extract<IrExpression, { kind: 'object' }>,
  target: IrType,
  context: EmitContext,
): boolean {
  const resolved = resolveSemanticType(target, context) ?? target;
  if (resolved.kind === 'union') {
    return resolved.variants.some((variant) => objectLiteralMatchesType(expression, variant, context));
  }
  if (resolved.kind === 'named' && resolved.name === 'RustMap') return true;
  if (resolved.kind !== 'anonymous') return false;
  const fields = flattenStructFields(resolved, context);
  const signature = objectLiteralPropertySignature(expression, context);
  if (!signature) return false;
  const { names, unknownSpread } = signature;
  const discriminantsMatch = fields.every((field) => {
    if (field.discriminantValue === undefined) return true;
    const property = expression.properties.find(
      (candidate) => candidate.kind === 'property' && candidate.name === field.name,
    );
    if (!property || property.kind !== 'property') return unknownSpread;
    const value = constantExpressionValue(property.value, context);
    return value === undefined || value === field.discriminantValue;
  });
  return (
    discriminantsMatch &&
    fields.every((field) => field.optional || unknownSpread || names.has(field.name)) &&
    [...names].every((name) => fields.some((field) => field.name === name))
  );
}

function constantExpressionValue(
  expression: IrExpression,
  context: EmitContext,
): boolean | number | string | undefined {
  const value = unwrapCasts(expression);
  if (value.kind === 'literal' && value.value !== null) return value.value;
  if (value.kind === 'property' && value.object.kind === 'identifier') {
    return context.constantPropertyValues.get(`${value.object.name}.${value.name}`);
  }
  return undefined;
}

function emitElement(expression: Extract<IrExpression, { kind: 'element' }>, context: EmitContext): string {
  if (isNativeEntityRuntimeAccess(expression, context)) {
    return emitEntityRuntimeValue(expression.object, context);
  }
  const objectType = inferIrExpressionType(expression.object, context);
  if (expression.optional) {
    if (
      objectType?.kind === 'nullable' &&
      (objectType.inner.kind === 'array' ||
        (objectType.inner.kind === 'named' && Boolean(typedArrayType(objectType.inner.name))))
    ) {
      const owner = emitPlaceExpression(expression.object, context);
      return `${owner}.as_ref().and_then(|values| values.get(${emitExpression(expression.index, context)} as usize).cloned())`;
    }
    if (objectType?.kind === 'nullable' && objectType.inner.kind === 'named' && objectType.inner.name === 'RustMap') {
      const keyType = objectType.inner.arguments[0] ?? { kind: 'dynamic' };
      const owner = emitPlaceExpression(expression.object, context);
      const key = emitExpression(expression.index, context, keyType);
      return `${owner}.as_ref().and_then(|entries| entries.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone()))`;
    }
    throw new RustEmissionError('optional element access requires an inferred nullable collection');
  }
  const collectionType = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  if (collectionType?.kind === 'named' && collectionType.name === 'RustTuple2') {
    if (expression.index.kind !== 'literal' || typeof expression.index.value !== 'number') {
      throw new RustEmissionError('tuple element access requires a static numeric index');
    }
    const index = expression.index.value;
    if (index !== 0 && index !== 1) throw new RustEmissionError(`tuple index ${String(index)} is out of bounds`);
    return `${emitPlaceExpression(expression.object, context)}.${String(index)}`;
  }
  if (collectionType?.kind === 'named' && collectionType.name === 'RustMap') {
    const keyType = collectionType.arguments[0] ?? { kind: 'dynamic' };
    const owner =
      objectType?.kind === 'nullable'
        ? `${emitPlaceExpression(expression.object, context)}.as_ref().unwrap()`
        : emitPlaceExpression(expression.object, context);
    const key = emitExpression(expression.index, context, keyType);
    return `${owner}.iter().find(|(entry_key, _)| entry_key == &${key}).map(|(_, value)| value.clone())`;
  }
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

function inferMapElementLookup(
  expression: IrExpression,
  context: EmitContext,
): { expression: Extract<IrExpression, { kind: 'element' }>; type: IrType } | undefined {
  if (expression.kind !== 'element') return undefined;
  const objectType = inferIrExpressionType(expression.object, context);
  const candidate = objectType?.kind === 'nullable' ? objectType.inner : objectType;
  const collection = resolveSemanticType(candidate, context) ?? candidate;
  const type = collection?.kind === 'named' && collection.name === 'RustMap' ? collection.arguments[1] : undefined;
  return type ? { expression, type } : undefined;
}

function emitElementRead(
  expression: Extract<IrExpression, { kind: 'element' }>,
  context: EmitContext,
  expectedType?: IrType,
): string {
  const objectType = inferIrExpressionType(expression.object, context);
  const resolvedObject = resolveSemanticType(objectType?.kind === 'nullable' ? objectType.inner : objectType, context);
  if (resolvedObject?.kind === 'primitive' && resolvedObject.name === 'String') {
    const view =
      expression.object.kind === 'identifier' ? context.utf16ViewNames.get(expression.object.name) : undefined;
    const units = view
      ? `&${view}`
      : `&${parenthesize(emitExpression(expression.object, context, primitive('String')))}.encode_utf16().collect::<Vec<u16>>()`;
    const index = emitExpression(expression.index, context, primitive('Float'));
    return `{ let __flight_units: &[u16] = ${units}; let __flight_raw_index = ${index}; if __flight_raw_index.is_finite() && __flight_raw_index >= 0.0_f64 && __flight_raw_index.fract() == 0.0_f64 { __flight_units.get(__flight_raw_index as usize).map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit])) } else { String::new() } }`;
  }
  if (isDynamicHostTree(expression.object, context)) {
    const result = expectedType ??
      inferDynamicHostElementType(expression.object, context) ?? {
        kind: 'dynamic',
      };
    return emitHostValueExpression(result, '"host.index"', context);
  }
  const place = emitElement(expression, context);
  if (expression.optional) return place;
  if (resolvedObject?.kind === 'named' && resolvedObject.name === 'RustMap' && expectedType?.kind !== 'nullable') {
    return `${parenthesize(place)}.expect("TypeScript Record key was absent")`;
  }
  const elementType = inferIrExpressionType(expression, context);
  if (
    elementType?.kind === 'nullable' &&
    expectedType?.kind !== 'nullable' &&
    expectedType &&
    semanticTypesEqual(elementType.inner, expectedType, context)
  ) {
    return `${place}.clone().unwrap()`;
  }
  return objectType?.kind === 'named' && typedArrayType(objectType.name) ? `(${place} as f64)` : `${place}.clone()`;
}

function inferIrExpressionType(expression: IrExpression, context: EmitContext): IrType | undefined {
  switch (expression.kind) {
    case 'assignment': {
      const left = inferIrExpressionType(expression.left, context);
      const right = inferIrExpressionType(expression.right, context);
      if (expression.operator !== '??=' || left?.kind !== 'nullable') return left ?? right;
      return right?.kind === 'nullable' ? left : left.inner;
    }
    case 'await': {
      const task = promiseType(inferIrExpressionType(expression.expression, context), context);
      return task?.output ?? inferIrExpressionType(expression.expression, context);
    }
    case 'identifier':
      if (expression.name === 'Infinity' || expression.name === 'NaN') return primitive('Float');
      return context.symbolTypes.get(expression.name);
    case 'literal':
      if (typeof expression.value === 'number') return primitive('Float');
      if (typeof expression.value === 'boolean') return primitive('Bool');
      if (typeof expression.value === 'string') return primitive('String');
      return undefined;
    case 'template':
      return primitive('String');
    case 'object':
      return inferObjectSpreadMapType(expression, context) ?? synthesizeObjectLiteralType(expression, context);
    case 'array':
      return expression.elements[0]
        ? {
            element: inferIrExpressionType(expression.elements[0], context) ?? {
              kind: 'dynamic',
            },
            kind: 'array',
          }
        : undefined;
    case 'binary': {
      const left = inferIrExpressionType(expression.left, context);
      const right = inferIrExpressionType(expression.right, context);
      if (['===', '!==', '==', '!=', '<', '<=', '>', '>=', 'instanceof', 'in'].includes(expression.operator)) {
        return primitive('Bool');
      }
      if (expression.operator === '&&' || expression.operator === '||') return left;
      if (expression.operator === '??' || expression.operator === '??undefined') {
        if (isNullishExpression(expression.right)) return left;
        if (left?.kind !== 'nullable') return left;
        return right?.kind === 'nullable' ? left : left.inner;
      }
      if (isPortableNumericStorageType(left) || isPortableNumericStorageType(right)) return primitive('Float');
      return left ?? right;
    }
    case 'conditional': {
      const narrowed = narrowTypeofContexts(expression.condition, context);
      const whenTrue = inferIrExpressionType(expression.whenTrue, narrowed.whenTrue);
      const whenFalse = inferIrExpressionType(expression.whenFalse, narrowed.whenFalse);
      if (isNullishExpression(expression.whenTrue) && whenFalse) {
        return whenFalse.kind === 'nullable' ? whenFalse : { inner: whenFalse, kind: 'nullable' };
      }
      if (isNullishExpression(expression.whenFalse) && whenTrue) {
        return whenTrue.kind === 'nullable' ? whenTrue : { inner: whenTrue, kind: 'nullable' };
      }
      if (whenTrue?.kind === 'nullable' && whenFalse && typeKey(whenTrue.inner) === typeKey(whenFalse)) {
        return whenTrue;
      }
      if (whenFalse?.kind === 'nullable' && whenTrue && typeKey(whenFalse.inner) === typeKey(whenTrue)) {
        return whenFalse;
      }
      return whenTrue ?? whenFalse;
    }
    case 'cast':
      return expression.type;
    case 'function':
      return {
        kind: 'function',
        parameters: expression.parameters.map((parameter) => parameter.type),
        returns:
          expression.returns ??
          (expression.expression ? inferIrExpressionType(expression.expression, context) : undefined) ??
          inferFunctionExpressionReturnType(expression) ??
          (expression.expression ? primitive('Float') : primitive('Void')),
      };
    case 'hostConstruct':
      return { arguments: [], kind: 'named', name: expression.resultType };
    case 'call': {
      const portableGlobal =
        expression.callee.kind === 'identifier' ? expression.callee.name : runtimeGlobalType(expression.callee);
      if (portableGlobal && ['Number', 'parseFloat', 'parseInt'].includes(portableGlobal)) {
        return primitive('Float');
      }
      if (portableGlobal && ['decodeURIComponent', 'encodeURIComponent'].includes(portableGlobal)) {
        return primitive('String');
      }
      if (portableGlobal === 'String') return primitive('String');
      if (portableGlobal === 'isNaN') {
        return primitive('Bool');
      }
      if (
        expression.callee.kind === 'property' &&
        expression.callee.object.kind === 'identifier' &&
        expression.callee.object.name === 'Number' &&
        expression.callee.name === 'parseFloat'
      ) {
        return primitive('Float');
      }
      if (
        expression.callee.kind === 'property' &&
        ((expression.callee.object.kind === 'identifier' && expression.callee.object.name === 'Object') ||
          runtimeGlobalType(expression.callee.object) === 'Object') &&
        expression.callee.name === 'is'
      ) {
        return primitive('Bool');
      }
      if (
        expression.callee.kind === 'property' &&
        expression.callee.object.kind === 'identifier' &&
        expression.callee.object.name === 'String' &&
        expression.callee.name === 'fromCodePoint'
      ) {
        return primitive('String');
      }
      if (
        expression.callee.kind === 'property' &&
        ((expression.callee.object.kind === 'identifier' && expression.callee.object.name === 'Object') ||
          runtimeGlobalType(expression.callee.object) === 'Object') &&
        expression.callee.name === 'keys'
      ) {
        const value = expression.arguments[0];
        const valueType = value ? inferIrExpressionType(value, context) : undefined;
        const collection = valueType?.kind === 'nullable' ? valueType.inner : valueType;
        return {
          element:
            collection?.kind === 'named' && collection.name === 'RustMap'
              ? (collection.arguments[0] ?? primitive('String'))
              : primitive('String'),
          kind: 'array',
        };
      }
      if (
        expression.callee.kind === 'property' &&
        ((expression.callee.object.kind === 'identifier' && expression.callee.object.name === 'Object') ||
          runtimeGlobalType(expression.callee.object) === 'Object') &&
        expression.callee.name === 'entries'
      ) {
        const value = expression.arguments[0];
        const valueType = value ? inferIrExpressionType(value, context) : undefined;
        const collection = valueType?.kind === 'nullable' ? valueType.inner : valueType;
        return {
          element: {
            arguments:
              collection?.kind === 'named' && collection.name === 'RustMap'
                ? collection.arguments
                : [primitive('String'), { kind: 'dynamic' }],
            kind: 'named',
            name: 'RustTuple2',
          },
          kind: 'array',
        };
      }
      if (
        expression.callee.kind === 'property' &&
        (expression.callee.name === 'then' || expression.callee.name === 'catch')
      ) {
        const promise = promiseType(inferIrExpressionType(expression.callee.object, context), context);
        if (promise) {
          if (expression.callee.name === 'catch') return promise;
          const callback = expression.arguments[0]
            ? resolveSemanticType(inferIrExpressionType(expression.arguments[0], context), context)
            : undefined;
          const returns = callback?.kind === 'function' ? callback.returns : primitive('Void');
          return returns.kind === 'task' ? returns : { kind: 'task', output: returns };
        }
      }
      const hostReturnType = inferOptionalHostCallReturnType(expression, context);
      if (hostReturnType) return { inner: hostReturnType, kind: 'nullable' };
      const knownHostReturnType = inferKnownHostCallReturnType(expression, context);
      if (knownHostReturnType) return knownHostReturnType;
      if (runtimeGlobalType(expression)) return { kind: 'dynamic' };
      if (isSymbolConstruction(expression)) {
        return { arguments: [], kind: 'named', name: 'FlightSymbol' };
      }
      if (isArrayFillConstruction(expression) && expression.arguments[0]) {
        return {
          element: inferIrExpressionType(expression.arguments[0], context) ?? {
            kind: 'dynamic',
          },
          kind: 'array',
        };
      }
      if (
        expression.callee.kind === 'property' &&
        expression.callee.object.kind === 'identifier' &&
        expression.callee.object.name === 'Array' &&
        expression.callee.name === 'from'
      ) {
        return { element: { kind: 'dynamic' }, kind: 'array' };
      }
      const collectionMethod =
        expression.callee.kind === 'property' &&
        (() => {
          const owner = inferIrExpressionType(expression.callee.object, context);
          const collection = owner?.kind === 'nullable' ? owner.inner : owner;
          return (
            collection?.kind === 'named' &&
            collection.name === 'RustMap' &&
            ['clear', 'delete', 'get', 'has', 'keys', 'set', 'values'].includes(expression.callee.name)
          );
        })();
      if (!collectionMethod) {
        const callee = resolveSemanticType(inferIrExpressionType(expression.callee, context), context);
        const callback = callee?.kind === 'nullable' ? resolveSemanticType(callee.inner, context) : callee;
        if (callback?.kind === 'function') {
          const returns = callback.returns;
          return expression.optional && returns.kind !== 'nullable' ? { inner: returns, kind: 'nullable' } : returns;
        }
      }
      if (expression.callee.kind === 'identifier') {
        const declaration = context.functions.get(expression.callee.name);
        if (declaration) {
          return substituteIrType(
            declaration.returns,
            inferFunctionTypeSubstitutions(declaration, expression, context),
          );
        }
        const local = context.symbolTypes.get(expression.callee.name);
        if (local?.kind === 'function') return local.returns;
      }
      if (expression.callee.kind === 'property') {
        const owner = inferIrExpressionType(expression.callee.object, context);
        const collection = owner?.kind === 'nullable' ? owner.inner : owner;
        if (collection?.kind === 'array') {
          if (['filter', 'slice', 'sort'].includes(expression.callee.name)) return collection;
          if (expression.callee.name === 'join') return primitive('String');
          if (expression.callee.name === 'find' || expression.callee.name === 'pop') {
            return { inner: collection.element, kind: 'nullable' };
          }
          if (expression.callee.name === 'map' && expression.arguments[0]?.kind === 'function') {
            return {
              element:
                inferContextualFunctionExpressionReturnType(expression.arguments[0], [collection.element], context) ??
                ({ kind: 'dynamic' } as const),
              kind: 'array',
            };
          }
        }
        if (collection?.kind === 'named' && collection.name === 'RustMap') {
          if (expression.callee.name === 'keys') {
            return {
              element: collection.arguments[0] ?? { kind: 'dynamic' },
              kind: 'array',
            };
          }
          if (expression.callee.name === 'values') {
            return {
              element: collection.arguments[1] ?? { kind: 'dynamic' },
              kind: 'array',
            };
          }
          if (expression.callee.name === 'get') {
            return {
              inner: collection.arguments[1] ?? { kind: 'dynamic' },
              kind: 'nullable',
            };
          }
          if (expression.callee.name === 'has' || expression.callee.name === 'delete') {
            return primitive('Bool');
          }
        }
        if (
          collection?.kind === 'named' &&
          collection.name === 'RustSet' &&
          (expression.callee.name === 'has' || expression.callee.name === 'delete')
        ) {
          return primitive('Bool');
        }
        if (
          collection?.kind === 'named' &&
          collection.name === 'RustSet' &&
          (expression.callee.name === 'keys' || expression.callee.name === 'values')
        ) {
          return {
            element: collection.arguments[0] ?? { kind: 'dynamic' },
            kind: 'array',
          };
        }
        if (collection?.kind === 'named' && collection.name === 'FlightRegex') {
          if (expression.callee.name === 'test') return primitive('Bool');
          if (expression.callee.name === 'exec') return regexCaptureType();
        }
        if (collection?.kind === 'primitive' && collection.name === 'String') {
          if (['startsWith', 'endsWith', 'includes'].includes(expression.callee.name)) return primitive('Bool');
          if (
            expression.callee.name === 'charCodeAt' ||
            expression.callee.name === 'codePointAt' ||
            expression.callee.name === 'indexOf' ||
            expression.callee.name === 'search'
          ) {
            return primitive('Float');
          }
          if (expression.callee.name === 'replace' || expression.callee.name === 'slice') {
            return primitive('String');
          }
          if (expression.callee.name === 'match' && expression.arguments[0]?.kind === 'regexp') {
            return regexCaptureType();
          }
          if (expression.callee.name === 'split') {
            return { element: primitive('String'), kind: 'array' };
          }
          if (['padStart', 'repeat', 'toLowerCase', 'toUpperCase', 'trim'].includes(expression.callee.name)) {
            return primitive('String');
          }
        }
        if (
          collection?.kind === 'primitive' &&
          (collection.name === 'Float' || collection.name === 'Int') &&
          (expression.callee.name === 'toFixed' || expression.callee.name === 'toString')
        ) {
          return primitive('String');
        }
        if (
          collection?.kind === 'primitive' &&
          (collection.name === 'Float' || collection.name === 'Int') &&
          [
            'abs',
            'acos',
            'asin',
            'atan',
            'cbrt',
            'ceil',
            'cos',
            'exp',
            'floor',
            'ln',
            'log2',
            'round',
            'sin',
            'sqrt',
            'tan',
            'trunc',
          ].includes(expression.callee.name)
        ) {
          return primitive('Float');
        }
      }
      if (isDynamicHostTree(expression, context)) return { kind: 'dynamic' };
      return undefined;
    }
    case 'element': {
      if (isNativeEntityRuntimeAccess(expression, context)) {
        return { arguments: [], kind: 'named', name: 'EntityRuntime' };
      }
      const inferred = inferIrExpressionType(expression.object, context);
      const object = inferred?.kind === 'nullable' ? inferred.inner : inferred;
      const element =
        object?.kind === 'array'
          ? object.element
          : object?.kind === 'primitive' && object.name === 'String'
            ? primitive('String')
            : object?.kind === 'named'
              ? object.name === 'RustMap'
                ? (object.arguments[1] ?? { kind: 'dynamic' })
                : object.name === 'RustTuple2' &&
                    expression.index.kind === 'literal' &&
                    typeof expression.index.value === 'number'
                  ? object.arguments[expression.index.value]
                  : typedArrayElementType(object.name)
              : undefined;
      const resolvedElement = element ?? inferDynamicHostElementType(expression.object, context);
      if (!resolvedElement) return undefined;
      return expression.optional && resolvedElement.kind !== 'nullable'
        ? { inner: resolvedElement, kind: 'nullable' }
        : resolvedElement;
    }
    case 'property': {
      if (
        expression.object.kind === 'identifier' &&
        ((expression.object.name === 'Number' &&
          ['EPSILON', 'MAX_SAFE_INTEGER', 'MAX_VALUE', 'NaN', 'NEGATIVE_INFINITY', 'POSITIVE_INFINITY'].includes(
            expression.name,
          )) ||
          (expression.object.name === 'Float' && ['INFINITY', 'NAN'].includes(expression.name)) ||
          (expression.object.name === 'Math' && expression.name === 'PI'))
      ) {
        return primitive('Float');
      }
      if (expression.object.kind === 'identifier' && context.errorValueNames.has(expression.object.name)) {
        const property = errorValuePropertyType(expression.name);
        if (property)
          return expression.optional && property.kind !== 'nullable' ? { inner: property, kind: 'nullable' } : property;
      }
      if (expression.binding) return { kind: 'dynamic' };
      const object = inferIrExpressionType(expression.object, context);
      if (!object) return undefined;
      const receiver = object.kind === 'nullable' ? object.inner : object;
      const resolvedReceiver = resolveSemanticType(receiver, context) ?? receiver;
      if (resolvedReceiver.kind === 'dynamic' || isNativeHostHandleType(resolvedReceiver)) {
        return inferDynamicHostPropertyType(expression.name) ?? { kind: 'dynamic' };
      }
      const field = inferPropertyType(receiver, expression.name, context);
      if (!field) return undefined;
      return expression.optional && field.kind !== 'nullable' ? { inner: field, kind: 'nullable' } : field;
    }
    case 'new': {
      const name = runtimeConstructorType(expression.callee);
      if (name === 'Map' || name === 'WeakMap') {
        const source = expression.arguments[0] ? inferIrExpressionType(expression.arguments[0], context) : undefined;
        const sourceType = resolveSemanticType(source, context) ?? source;
        const sourceArguments =
          sourceType?.kind === 'named' && sourceType.name === 'RustMap'
            ? sourceType.arguments
            : sourceType?.kind === 'array' &&
                sourceType.element.kind === 'named' &&
                sourceType.element.name === 'RustTuple2'
              ? sourceType.element.arguments
              : [];
        return {
          arguments: expression.typeArguments.length > 0 ? expression.typeArguments : sourceArguments,
          kind: 'named',
          name: 'RustMap',
        };
      }
      if (name === 'Set' || name === 'WeakSet') {
        const source = expression.arguments[0] ? inferIrExpressionType(expression.arguments[0], context) : undefined;
        return {
          arguments:
            expression.typeArguments.length > 0
              ? expression.typeArguments
              : source?.kind === 'array'
                ? [source.element]
                : [],
          kind: 'named',
          name: 'RustSet',
        };
      }
      if (name === 'Array') {
        return {
          element: expression.typeArguments[0] ?? { kind: 'dynamic' },
          kind: 'array',
        };
      }
      if (name === 'ArrayBuffer') return { arguments: [], kind: 'named', name: 'ByteBuffer' };
      if (name === 'Error') return { kind: 'dynamic', portable: true };
      if (name && opaqueHostConstructors.has(name)) return { kind: 'dynamic' };
      return name && typedArrayType(name) ? { arguments: [], kind: 'named', name } : undefined;
    }
    case 'regexp':
      return { arguments: [], kind: 'named', name: 'FlightRegex' };
    case 'taskAll':
    case 'taskReady':
    case 'taskReject':
      return { kind: 'task', output: expression.output };
    case 'unary':
      if (expression.operator === '!') return primitive('Bool');
      if (expression.operator === 'typeof') return primitive('String');
      if (['+', '-', '~', '++', '--'].includes(expression.operator)) return primitive('Float');
      return undefined;
    default:
      return undefined;
  }
}

function emitRegexp(expression: Extract<IrExpression, { kind: 'regexp' }>): string {
  const flags = expression.flags;
  return `regex::RegexBuilder::new(${emitRustStringLiteral(expression.pattern)}).case_insensitive(${String(
    flags.includes('i'),
  )}).multi_line(${String(flags.includes('m'))}).dot_matches_new_line(${String(
    flags.includes('s'),
  )}).build().expect("upstream TypeScript regular expression must be valid Rust regex syntax")`;
}

function regexCaptureType(): IrType {
  return {
    inner: {
      element: { inner: primitive('String'), kind: 'nullable' },
      kind: 'array',
    },
    kind: 'nullable',
  };
}

function emitRegexCaptures(regex: string, value: string): string {
  return `{ let __flight_regex = &${parenthesize(regex)}; __flight_regex.captures(&${parenthesize(value)}).map(|captures| (0..captures.len()).map(|index| captures.get(index).map(|matched| matched.as_str().to_owned())).collect::<Vec<_>>()) }`;
}

function narrowTypeofContexts(
  condition: IrExpression,
  context: EmitContext,
): { whenFalse: EmitContext; whenTrue: EmitContext } {
  const clone = (): EmitContext => ({
    ...context,
    errorValueNames: new Set(context.errorValueNames),
    excludedUnionVariants: new Map(
      [...context.excludedUnionVariants].map(([name, indices]) => [name, new Set(indices)]),
    ),
    knownNullNames: new Set(context.knownNullNames),
    nonNullableNames: new Set(context.nonNullableNames),
    symbolTypes: new Map(context.symbolTypes),
    unionNarrowings: new Map(context.unionNarrowings),
  });
  const unchanged = { whenFalse: clone(), whenTrue: clone() };
  if (
    condition.kind === 'unary' &&
    condition.operator === '!' &&
    condition.operand.kind === 'binary' &&
    condition.operand.operator === 'instanceof' &&
    runtimeConstructorType(condition.operand.right) === 'Error'
  ) {
    const narrowed = narrowTypeofContexts(condition.operand, context);
    return { whenFalse: narrowed.whenTrue, whenTrue: narrowed.whenFalse };
  }
  if (condition.kind === 'binary' && condition.operator === '||') {
    const left = narrowTypeofContexts(condition.left, context);
    const right = narrowTypeofContexts(condition.right, left.whenFalse);
    return { whenFalse: right.whenFalse, whenTrue: unchanged.whenTrue };
  }
  if (condition.kind === 'binary' && condition.operator === '&&') {
    const left = narrowTypeofContexts(condition.left, context);
    const right = narrowTypeofContexts(condition.right, left.whenTrue);
    return { whenFalse: unchanged.whenFalse, whenTrue: right.whenTrue };
  }
  if (condition.kind === 'identifier') {
    const type = context.symbolTypes.get(condition.name);
    if (type?.kind === 'nullable' && !context.sharedCaptureNames.has(condition.name)) {
      const whenTrue = clone();
      whenTrue.symbolTypes.set(condition.name, type.inner);
      (whenTrue.nonNullableNames as Set<string>).add(condition.name);
      return { whenFalse: clone(), whenTrue };
    }
    return unchanged;
  }
  if (
    condition.kind === 'binary' &&
    condition.operator === 'instanceof' &&
    condition.left.kind === 'identifier' &&
    runtimeConstructorType(condition.right) === 'Error'
  ) {
    const whenTrue = clone();
    (whenTrue.errorValueNames as Set<string>).add(condition.left.name);
    return { whenFalse: clone(), whenTrue };
  }
  if (condition.kind !== 'binary' || !['===', '!==', '==', '!='].includes(condition.operator)) {
    return unchanged;
  }
  const nullableIdentifier =
    condition.left.kind === 'identifier' && isNullishExpression(condition.right)
      ? condition.left
      : condition.right.kind === 'identifier' && isNullishExpression(condition.left)
        ? condition.right
        : undefined;
  if (nullableIdentifier) {
    const type = context.symbolTypes.get(nullableIdentifier.name);
    if (type?.kind === 'nullable' && !context.sharedCaptureNames.has(nullableIdentifier.name)) {
      const positive = ['===', '=='].includes(condition.operator);
      const whenTrue = clone();
      const whenFalse = clone();
      const narrowed = positive ? whenFalse : whenTrue;
      narrowed.symbolTypes.set(nullableIdentifier.name, type.inner);
      (narrowed.nonNullableNames as Set<string>).add(nullableIdentifier.name);
      return { whenFalse, whenTrue };
    }
  }
  const discriminant = discriminatedUnionComparison(condition, context);
  if (discriminant) {
    const whenTrue = clone();
    const whenFalse = clone();
    const excluded = context.excludedUnionVariants.get(discriminant.name) ?? new Set<number>();
    const possible = discriminant.variants.flatMap((_, index) => (excluded.has(index) ? [] : [index]));
    const matching = possible.filter((index) => index === discriminant.matchingIndex);
    const remaining = possible.filter((index) => index !== discriminant.matchingIndex);
    const narrow = (target: EmitContext, indices: readonly number[]): void => {
      const nextExcluded = new Set(discriminant.variants.map((_, index) => index));
      for (const index of indices) nextExcluded.delete(index);
      (target.excludedUnionVariants as Map<string, ReadonlySet<number>>).set(discriminant.name, nextExcluded);
      if (indices.length === 1) {
        const index = indices[0]!;
        target.symbolTypes.set(discriminant.name, discriminant.variants[index]!);
        target.unionNarrowings.set(discriminant.name, {
          index,
          unionName: discriminant.unionName,
          variants: discriminant.variants,
        });
      } else {
        target.symbolTypes.set(discriminant.name, discriminant.sourceType);
        target.unionNarrowings.delete(discriminant.name);
      }
    };
    narrow(whenTrue, discriminant.positive ? matching : remaining);
    narrow(whenFalse, discriminant.positive ? remaining : matching);
    return { whenFalse, whenTrue };
  }
  const typeofExpression =
    condition.left.kind === 'unary' && condition.left.operator === 'typeof'
      ? condition.left
      : condition.right.kind === 'unary' && condition.right.operator === 'typeof'
        ? condition.right
        : undefined;
  const tagExpression = typeofExpression === condition.left ? condition.right : condition.left;
  if (
    !typeofExpression ||
    typeofExpression.operand.kind !== 'identifier' ||
    tagExpression.kind !== 'literal' ||
    typeof tagExpression.value !== 'string'
  ) {
    return unchanged;
  }
  const name = typeofExpression.operand.name;
  const declaredType = context.symbolTypes.get(name);
  const type = resolveSemanticType(declaredType, context) ?? declaredType;
  if (type?.kind !== 'union') return unchanged;
  const matching = type.variants.flatMap((variant, index) =>
    typeOfTag(variant, context) === tagExpression.value ? [index] : [],
  );
  const remaining = type.variants.flatMap((_, index) => (matching.includes(index) ? [] : [index]));
  if (matching.length !== 1 || remaining.length !== 1) return unchanged;
  const positive = ['===', '=='].includes(condition.operator);
  const whenTrueIndex = positive ? matching[0]! : remaining[0]!;
  const whenFalseIndex = positive ? remaining[0]! : matching[0]!;
  const whenTrue = clone();
  const whenFalse = clone();
  whenTrue.symbolTypes.set(name, type.variants[whenTrueIndex]!);
  whenFalse.symbolTypes.set(name, type.variants[whenFalseIndex]!);
  whenTrue.unionNarrowings.set(name, {
    index: whenTrueIndex,
    unionName: declaredType?.kind === 'named' ? declaredType.name : undefined,
    variants: type.variants,
  });
  whenFalse.unionNarrowings.set(name, {
    index: whenFalseIndex,
    unionName: declaredType?.kind === 'named' ? declaredType.name : undefined,
    variants: type.variants,
  });
  return { whenFalse, whenTrue };
}

function discriminatedUnionComparison(
  expression: IrExpression,
  context: EmitContext,
):
  | {
      matchingIndex: number;
      name: string;
      positive: boolean;
      sourceType: IrType;
      unionName?: string | undefined;
      variants: readonly IrType[];
    }
  | undefined {
  if (expression.kind !== 'binary' || !['===', '!==', '==', '!='].includes(expression.operator)) {
    return undefined;
  }
  const comparison = (
    candidate: IrExpression,
    value: IrExpression,
  ): { literal: boolean | number | string; name: string; propertyName: string } | undefined => {
    if (candidate.kind !== 'property' || candidate.object.kind !== 'identifier') return undefined;
    const literal = constantExpressionValue(value, context);
    return literal === undefined ? undefined : { literal, name: candidate.object.name, propertyName: candidate.name };
  };
  const property = comparison(expression.left, expression.right) ?? comparison(expression.right, expression.left);
  if (!property) return undefined;
  const name = property.name;
  const sourceType = context.symbolTypes.get(name);
  const resolved = resolveSemanticType(sourceType, context);
  if (!sourceType || resolved?.kind !== 'union') return undefined;
  const matching = resolved.variants.flatMap((variant, index) => {
    const concrete = resolveSemanticType(variant, context);
    if (concrete?.kind !== 'anonymous') return [];
    const field = flattenStructFields(concrete, context).find((item) => item.name === property.propertyName);
    return field?.discriminantValue === property.literal ? [index] : [];
  });
  if (matching.length !== 1) return undefined;
  return {
    matchingIndex: matching[0]!,
    name,
    positive: expression.operator === '===' || expression.operator === '==',
    sourceType,
    unionName: sourceType?.kind === 'named' ? emitNamedUnionConstructor(sourceType, context) : undefined,
    variants: resolved.variants,
  };
}

function inferPropertyType(object: IrType, name: string, context: EmitContext): IrType | undefined {
  const resolved = resolveSemanticType(object, context) ?? object;
  if (
    (resolved.kind === 'array' || (resolved.kind === 'named' && Boolean(typedArrayType(resolved.name)))) &&
    name === 'length'
  ) {
    return primitive('Float');
  }
  if (resolved.kind === 'primitive' && resolved.name === 'String' && name === 'length') {
    return primitive('Float');
  }
  if (resolved.kind === 'named' && (resolved.name === 'RustMap' || resolved.name === 'RustSet') && name === 'size') {
    return primitive('Float');
  }
  if (resolved.kind === 'named' && resolved.name === 'RustMap') {
    return resolved.arguments[1] ?? { kind: 'dynamic' };
  }
  if (resolved.kind === 'union') {
    const fields = resolved.variants.map((variant) => {
      const concrete = resolveSemanticType(variant, context);
      return concrete?.kind === 'anonymous'
        ? flattenStructFields(concrete, context).find((field) => field.name === name)
        : undefined;
    });
    const first = fields[0];
    if (first && fields.every((field) => field && semanticTypesEqual(field.type, first.type, context))) {
      return first.optional && first.type.kind !== 'nullable' ? { inner: first.type, kind: 'nullable' } : first.type;
    }
  }
  if (resolved.kind !== 'anonymous') return undefined;
  const field = semanticStructFields(object, context).find((item) => item.name === name);
  if (!field) return undefined;
  return field.optional && field.type.kind !== 'nullable' ? { inner: field.type, kind: 'nullable' } : field.type;
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

function runtimeConstructorType(expression: IrExpression): string | undefined {
  if (
    expression.kind === 'identifier' &&
    (typedArrayType(expression.name) ||
      ['Array', 'Error', 'Map', 'Proxy', 'Set', 'WeakMap', 'WeakSet'].includes(expression.name))
  ) {
    return expression.name;
  }
  return runtimeGlobalType(expression);
}

function typedArrayType(name: string): { ir: string; rust: string; zero: string } | undefined {
  return typedArrays[name];
}

function typedArrayElementType(name: string): IrType | undefined {
  const typedArray = typedArrayType(name);
  return typedArray ? { arguments: [], kind: 'named', name: typedArray.ir } : undefined;
}

function typedArrayByteWidth(rustType: string): number {
  if (rustType === 'i8' || rustType === 'u8') return 1;
  if (rustType === 'i16' || rustType === 'u16') return 2;
  if (rustType === 'f32' || rustType === 'i32' || rustType === 'u32') return 4;
  return 8;
}

function isPortableNumericStorageType(type: IrType | undefined): boolean {
  return (
    type?.kind === 'named' &&
    ['RustF32', 'RustF64', 'RustI8', 'RustI16', 'RustI32', 'RustU8', 'RustU16', 'RustU32'].includes(type.name)
  );
}

function javaScriptValueType(type: IrType): IrType {
  return isPortableNumericStorageType(type) ? primitive('Float') : type;
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
  return emitRustStringLiteral('generated Flight function threw');
}

function emitTemplate(expression: Extract<IrExpression, { kind: 'template' }>, context: EmitContext): string {
  const arguments_: string[] = [];
  const format = expression.parts
    .map((part) => {
      if (typeof part === 'string') return part.replaceAll('{', '{{').replaceAll('}', '}}');
      const type = inferIrExpressionType(part, context);
      const value = emitExpression(part, context, type);
      arguments_.push(
        type?.kind === 'nullable'
          ? `${parenthesize(value)}.as_ref().map_or_else(|| "undefined".to_owned(), |value| value.to_string())`
          : value,
      );
      return '{}';
    })
    .join('');
  return `format!(${[emitRustStringLiteral(format), ...arguments_].join(', ')})`;
}

function emitIdentifier(name: string, context: EmitContext): string {
  const alias = context.placeAliases.get(name);
  if (alias) return emitPlaceExpression(alias, context);
  const recursiveSlot = context.recursiveClosureSlots.get(name);
  if (recursiveSlot) return `${recursiveSlot}.lock().unwrap().as_ref().unwrap().clone()`;
  const emitted = context.constantNames.get(name) ?? safeName(name);
  const type = context.symbolTypes.get(name);
  if (context.constantNames.has(name) && type?.kind === 'named' && type.name === 'FlightSymbol') {
    return `*${emitted}`;
  }
  if (context.lazyScalarNames.has(name)) return `*${emitted}`;
  if (context.nonNullableNames.has(name)) {
    const owner = context.mutexValueNames.has(name) ? `(*${emitted}.lock().unwrap())` : emitted;
    const value = `${owner}.${context.mutatedNames.has(name) ? 'as_mut' : 'as_ref'}().unwrap()`;
    return type && isCopyType(type, context) ? `*${parenthesize(value)}` : value;
  }
  if (context.sharedCaptureNames.has(name)) return `(*${emitted}.lock().unwrap())`;
  if (context.mutexCollectionNames.has(name)) return `(*${emitted}.lock().unwrap())`;
  if (context.mutexValueNames.has(name)) return `(*${emitted}.lock().unwrap())`;
  return context.atomicBoolNames.has(name) ? `${emitted}.load(std::sync::atomic::Ordering::Relaxed)` : emitted;
}

function isNullishExpression(expression: IrExpression): boolean {
  return (
    (expression.kind === 'literal' && expression.value === null) ||
    (expression.kind === 'identifier' && expression.name.toLowerCase() === 'undefined')
  );
}

function isSymbolConstruction(expression: IrExpression): boolean {
  if (expression.kind !== 'call') return false;
  if (expression.callee.kind === 'identifier') return expression.callee.name === 'Symbol';
  return (
    expression.callee.kind === 'property' &&
    expression.callee.object.kind === 'identifier' &&
    expression.callee.object.name === 'Symbol' &&
    expression.callee.name === 'for'
  );
}

function emitCondition(expression: IrExpression, context: EmitContext): string {
  if (expression.kind === 'binary' && (expression.operator === '&&' || expression.operator === '||')) {
    return `${parenthesize(emitCondition(expression.left, context))} ${expression.operator} ${parenthesize(
      emitCondition(expression.right, context),
    )}`;
  }
  const type = inferIrExpressionType(expression, context);
  const resolved = resolveSemanticType(type, context) ?? type;
  if (resolved?.kind === 'dynamic') {
    const value = emitExpression(expression, context);
    const valuePath = dynamicValuePath(resolved);
    return `match &${parenthesize(value)} { ${valuePath}::Undefined | ${valuePath}::Null => false, ${valuePath}::Bool(value) => *value, ${valuePath}::Number(value) => *value != 0.0_f64 && !value.is_nan(), ${valuePath}::String(value) => !value.is_empty(), ${valuePath}::Array(_) | ${valuePath}::Record(_) | ${valuePath}::Error { .. } | ${valuePath}::Function | ${valuePath}::Symbol | ${valuePath}::Object => true }`;
  }
  const emitted = emitExpression(expression, context);
  if (type?.kind === 'primitive' && type.name === 'Bool') return emitted;
  if (type?.kind === 'primitive' && (type.name === 'Float' || type.name === 'Int')) {
    return `${parenthesize(emitted)} != 0.0_f64`;
  }
  if (type?.kind === 'primitive' && type.name === 'String') {
    return `!${parenthesize(emitted)}.is_empty()`;
  }
  if (type?.kind === 'nullable') {
    const inner = resolveSemanticType(type.inner, context) ?? type.inner;
    if (inner.kind === 'primitive' && inner.name === 'Bool') {
      return `${parenthesize(emitted)}.unwrap_or(false)`;
    }
    if (inner.kind === 'primitive' && (inner.name === 'Float' || inner.name === 'Int')) {
      return `${parenthesize(emitted)}.unwrap_or(0.0_f64) != 0.0_f64`;
    }
    if (inner.kind === 'primitive' && inner.name === 'String') {
      return `${parenthesize(emitted)}.as_ref().is_some_and(|value| !value.is_empty())`;
    }
    return `${parenthesize(emitted)}.is_some()`;
  }
  if (type && isReferenceLike(type, context)) return 'true';
  return emitted;
}

function isDynamicHostTree(expression: IrExpression, context: EmitContext): boolean {
  const root = expressionRootIdentifier(expression);
  const direct =
    root && context.symbolTypes.has(root) && expression.kind !== 'call'
      ? inferIrExpressionType(expression, context)
      : undefined;
  const resolvedDirect = resolveSemanticType(direct, context) ?? direct;
  const directReceiver =
    resolvedDirect?.kind === 'nullable'
      ? (resolveSemanticType(resolvedDirect.inner, context) ?? resolvedDirect.inner)
      : resolvedDirect;
  if (directReceiver?.kind === 'dynamic' || isNativeHostHandleType(directReceiver)) return true;
  if (!root) return false;
  const type = context.symbolTypes.get(root);
  const resolved = resolveSemanticType(type, context) ?? type;
  const receiver =
    resolved?.kind === 'nullable' ? (resolveSemanticType(resolved.inner, context) ?? resolved.inner) : resolved;
  return receiver?.kind === 'dynamic' || isNativeHostHandleType(receiver);
}

function inferDynamicHostElementType(expression: IrExpression, context: EmitContext): IrType | undefined {
  if (!isDynamicHostTree(expression, context) || expression.kind !== 'property') return undefined;
  if (expression.name === 'axes') return primitive('Float');
  return { kind: 'dynamic' };
}

function inferDynamicHostPropertyType(name: string): IrType | undefined {
  if (
    ['angle', 'currentTime', 'duration', 'index', 'length', 'value', 'videoHeight', 'videoWidth', 'volume'].includes(
      name,
    )
  ) {
    return primitive('Float');
  }
  if (name === 'state') return primitive('String');
  if (name === 'isWordLike') return primitive('Bool');
  if (name === 'segment') return primitive('String');
  if (
    [
      'activeElement',
      'firstChild',
      'lastChild',
      'nextSibling',
      'parentElement',
      'parentNode',
      'previousSibling',
    ].includes(name)
  ) {
    return { inner: { kind: 'dynamic' }, kind: 'nullable' };
  }
  return undefined;
}

function emitLiteral(value: boolean | null | number | string, expectedType?: IrType, context?: EmitContext): string {
  const resolved = context ? resolveSemanticType(expectedType, context) : expectedType;
  if (resolved?.kind === 'dynamic') {
    const valuePath = dynamicValuePath(resolved);
    if (value === null) return `${valuePath}::Null`;
    if (typeof value === 'boolean') return `${valuePath}::Bool(${String(value)})`;
    if (typeof value === 'number') return `${valuePath}::Number(${emitNumberLiteral(value)})`;
    return `${valuePath}::String(${emitRustStringLiteral(value)}.to_owned())`;
  }
  if (value === null) return 'None';
  if (typeof value === 'string') {
    const literal = emitRustStringLiteral(value);
    return resolved?.kind === 'primitive' && resolved.name === 'String' ? `${literal}.to_owned()` : literal;
  }
  if (typeof value === 'boolean') return String(value);
  return coerceExpression(emitNumberLiteral(value), expectedType);
}

function emitNumberLiteral(value: number): string {
  if (Number.isNaN(value)) return 'f64::NAN';
  if (!Number.isFinite(value)) return value > 0 ? 'f64::INFINITY' : 'f64::NEG_INFINITY';
  return Number.isInteger(value)
    ? `${String(value)}.0_f64`
    : `${/[.eE]/u.test(String(value)) ? String(value) : `${String(value)}.0`}_f64`;
}

function emitRustStringLiteral(value: string): string {
  let emitted = '"';
  for (const character of value) {
    const codePoint = character.codePointAt(0)!;
    if (character === '"') emitted += '\\"';
    else if (character === '\\') emitted += '\\\\';
    else if (character === '\n') emitted += '\\n';
    else if (character === '\r') emitted += '\\r';
    else if (character === '\t') emitted += '\\t';
    else if (codePoint < 0x20 || codePoint === 0x7f) {
      emitted += `\\u{${codePoint.toString(16).padStart(4, '0')}}`;
    } else {
      emitted += character;
    }
  }
  return `${emitted}"`;
}

function resolveSemanticType(
  type: IrType | undefined,
  context: EmitContext,
  visited: ReadonlySet<string> = new Set(),
): IrType | undefined {
  if (type?.kind === 'named' && type.name.startsWith('RustStructural:')) {
    return type.arguments[0] ?? type;
  }
  if (type?.kind === 'named' && type.name === 'FlightPartial') {
    const inner = resolveSemanticType(type.arguments[0], context, visited);
    if (inner?.kind === 'anonymous') {
      return {
        extends: [],
        fields: flattenStructFields(inner, context).map((field) => ({
          ...field,
          optional: true,
        })),
        kind: 'anonymous',
      };
    }
    return inner;
  }
  if (type?.kind === 'named' && type.name.startsWith('FlightOmit:')) {
    const inner = resolveSemanticType(type.arguments[0], context, visited);
    if (inner?.kind === 'anonymous') {
      const omitted = new Set<string>(JSON.parse(type.name.slice('FlightOmit:'.length)) as string[]);
      return {
        extends: [],
        fields: flattenStructFields(inner, context).filter((field) => !omitted.has(field.name)),
        kind: 'anonymous',
      };
    }
    return inner;
  }
  if (type?.kind !== 'named') return type;
  if (type.arguments.length === 0 && context.lexicalTypeParameters.has(type.name)) return type;
  const key = typeKey(type);
  if (visited.has(key)) return type;
  const declaration = context.namedTypes.get(type.name);
  if (!declaration) return type;
  const parameters = context.namedTypeParameters.get(type.name) ?? [];
  const substitutions = new Map(
    parameters.flatMap((parameter, index) =>
      type.arguments[index] ? [[parameter, type.arguments[index]!] as const] : [],
    ),
  );
  const applied = substitutions.size > 0 ? substituteIrType(declaration, substitutions) : declaration;
  if (typeKey(applied) === key) return applied;
  return resolveSemanticType(applied, context, new Set([...visited, key]));
}

function isStructuralUtilityType(type: IrType | undefined): type is Extract<IrType, { kind: 'named' }> {
  return type?.kind === 'named' && (type.name === 'FlightPartial' || type.name.startsWith('FlightOmit:'));
}

function isDynamicHostCastTarget(type: IrType, context: EmitContext): boolean {
  const resolved = resolveSemanticType(type, context) ?? type;
  if (resolved.kind === 'nullable') return isDynamicHostCastTarget(resolved.inner, context);
  return resolved.kind === 'named' && (resolved.name === 'RustMap' || resolved.name === 'RustSet');
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
      if (expression.operator === '~') return ~operand;
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
        case '&':
          return left & right;
        case '|':
          return left | right;
        case '^':
          return left ^ right;
        case '<<':
          return left << (right & 31);
        case '>>':
          return left >> (right & 31);
        case '>>>':
          return left >>> (right & 31);
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

function resolveRustImport(
  item: RustImport['names'][number],
  declarations: string,
): RustImport['names'][number] | undefined {
  if (item.kind === 'function') {
    return {
      imported: snakeCase(item.imported),
      kind: item.kind,
      local: snakeCase(item.local),
      ...(item.public ? { public: true } : {}),
    };
  }
  const bindings =
    item.kind === 'type'
      ? [item]
      : item.kind === 'constant' || item.kind === 'mutable'
        ? [
            {
              imported: screamingSnakeCase(item.imported),
              kind: item.kind,
              local: importedConstantBinding(item),
              ...(item.public ? { public: true } : {}),
            },
          ]
        : [
            item,
            {
              imported: snakeCase(item.imported),
              ...(item.kind ? { kind: item.kind } : {}),
              local: snakeCase(item.local),
              ...(item.public ? { public: true } : {}),
            },
          ];
  if (item.public) {
    if (item.kind === 'constant' || item.kind === 'mutable') return bindings.at(-1);
    return bindings[0];
  }
  return bindings.find(({ local }) => new RegExp(`\\b${local}\\b`, 'u').test(declarations));
}

function importedConstantBinding(item: RustImport['names'][number]): string {
  return item.public ? screamingSnakeCase(item.local) : `${snakeCase(item.local)}_constant`;
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

const opaqueHostConstructors = new Set([
  'AbortController',
  'Audio',
  'AudioBuffer',
  'MediaMetadata',
  'ResizeObserver',
  'WebSocket',
]);

const nativeHostHandleTypes = new Set(['FlightImageData', 'FlightOffscreenCanvas', 'FlightUrl']);

function isNativeHostHandleType(type: IrType | undefined): boolean {
  return type?.kind === 'named' && nativeHostHandleTypes.has(type.name);
}

const opaqueHostInstanceConstructors = new Set([
  'HTMLCanvasElement',
  'HTMLImageElement',
  'HTMLVideoElement',
  'ImageBitmap',
  'OffscreenCanvas',
]);

const mutationMethods = new Set([
  'add',
  'clear',
  'copyWithin',
  'delete',
  'fill',
  'pop',
  'push',
  'reverse',
  'set',
  'shift',
  'sort',
  'splice',
  'unshift',
]);

const rustKeywords = new Set([
  'abstract',
  'as',
  'async',
  'await',
  'become',
  'box',
  'break',
  'const',
  'continue',
  'crate',
  'dyn',
  'do',
  'else',
  'enum',
  'extern',
  'false',
  'fn',
  'final',
  'for',
  'gen',
  'if',
  'impl',
  'in',
  'let',
  'loop',
  'macro',
  'match',
  'mod',
  'move',
  'mut',
  'override',
  'priv',
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
  'try',
  'type',
  'typeof',
  'unsafe',
  'unsized',
  'use',
  'where',
  'while',
  'virtual',
  'yield',
]);

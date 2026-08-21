import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import ts from 'typescript';

import { portConfig } from '../../port.config.ts';
import type {
  IrAsyncTaskOperations,
  IrAsyncTaskScope,
  IrDeclaration,
  IrExpression,
  IrFunctionExecution,
  IrFunctionDeclaration,
  IrParameter,
  IrStatement,
  IrTaskConstruction,
  IrType,
  IrTypeField,
  IrVariable,
  LoweringDiagnostic,
  LoweringResult,
  SourceOrigin,
} from '../model/ir.ts';

const fingerprintPrinter = ts.createPrinter({ removeComments: true });

const portableTypeReferenceMap: Readonly<Record<string, string>> = {
  ArrayBuffer: 'ByteBuffer',
  ArrayBufferLike: 'ByteBuffer',
  ArrayBufferView: 'ArrayBufferView',
  Float32Array: 'Float32Array',
  Float64Array: 'Float64Array',
  Int16Array: 'Int16Array',
  Int32Array: 'Int32Array',
  Int8Array: 'Int8Array',
  Uint16Array: 'Uint16Array',
  Uint32Array: 'Uint32Array',
  Uint8Array: 'Uint8Array',
  Uint8ClampedArray: 'Uint8ClampedArray',
};

const portableTypedArrayStorage: Readonly<Record<string, string>> = {
  Float32Array: 'f32',
  Float64Array: 'f64',
  Int8Array: 'i8',
  Int16Array: 'i16',
  Int32Array: 'i32',
  Uint8Array: 'u8',
  Uint8ClampedArray: 'u8',
  Uint16Array: 'u16',
  Uint32Array: 'u32',
};

const portableTypedArrayRank: Readonly<Record<string, number>> = {
  Float32Array: 32,
  Float64Array: 64,
  Int8Array: 8,
  Int16Array: 16,
  Int32Array: 32,
  Uint8Array: 8,
  Uint8ClampedArray: 8,
  Uint16Array: 16,
  Uint32Array: 32,
};

const platformDynamicTypes = new Set([
  'AbortController',
  'AbortSignal',
  'ArrayBuffer',
  'AudioBuffer',
  'AudioBufferSourceNode',
  'AudioContext',
  'AudioNode',
  'AsyncIterable',
  'AsyncIterableIterator',
  'Blob',
  'Buffer',
  'BufferSource',
  'CanvasFillRule',
  'CanvasGradient',
  'CanvasImageSource',
  'CanvasPattern',
  'CanvasRenderingContext2D',
  'CanvasRenderingContext2DSettings',
  'DOMRect',
  'DOMRectReadOnly',
  'DataView',
  'DOMException',
  'Document',
  'Element',
  'Event',
  'EventTarget',
  'EXT_texture_filter_anisotropic',
  'File',
  'FileSystemDirectoryHandle',
  'FileSystemFileHandle',
  'FileSystemHandle',
  'FocusEvent',
  'FontFace',
  'FrameRequestCallback',
  'GlobalCompositeOperation',
  'GainNode',
  'Gamepad',
  'GamepadButton',
  'GamepadEvent',
  'GeolocationCoordinates',
  'GeolocationPosition',
  'GeolocationPositionError',
  'PositionOptions',
  'HTMLCanvasElement',
  'HTMLImageElement',
  'HTMLInputElement',
  'HTMLTextAreaElement',
  'HTMLElement',
  'HTMLVideoElement',
  'Image',
  'Headers',
  'ImageData',
  'ImageBitmap',
  'ImageBitmapOptions',
  'ImageSmoothingQuality',
  'KeyboardEvent',
  'Float64Array',
  'Int32Array',
  'Int8Array',
  'Iterable',
  'IterableIterator',
  'Iterator',
  'Map',
  'MediaDeviceInfo',
  'MediaDevices',
  'MediaStream',
  'MediaStreamConstraints',
  'MediaStreamTrack',
  'MediaTrackConstraints',
  'MediaElementAudioSourceNode',
  'Navigator',
  'Notification',
  'NotificationOptions',
  'NotificationPermission',
  'OffscreenCanvas',
  'PointerEvent',
  'PermissionDescriptor',
  'PermissionStatus',
  'Permissions',
  'ReadableStream',
  'TextDecoder',
  'RegExp',
  'ReadonlyMap',
  'ReadonlySet',
  'Request',
  'RequestInit',
  'Response',
  'RenderingContext',
  'Set',
  'ShareData',
  'StereoPannerNode',
  'StorageManager',
  'TexImageSource',
  'URL',
  'URLSearchParams',
  'Window',
  'WheelEvent',
  'Uint32Array',
  'Uint8Array',
  'Uint8ClampedArray',
  'WebGL2RenderingContext',
  'WebGLBuffer',
  'WebGLContextAttributes',
  'WebGLFramebuffer',
  'WebGLPowerPreference',
  'WebGLProgram',
  'WebGLRenderbuffer',
  'WebGLTexture',
  'WebGLUniformLocation',
  'WritableStream',
  'WeakMap',
  'WeakRef',
  'WeakSet',
]);

const platformGlobalValues = new Set([
  'AbortController',
  'AbortSignal',
  'ArrayBuffer',
  'Blob',
  'Buffer',
  'CSSStyleDeclaration',
  'ClipboardItem',
  'FileReader',
  'HTMLCanvasElement',
  'HTMLImageElement',
  'HTMLVideoElement',
  'Image',
  'ImageData',
  'Intl',
  'File',
  'Float32Array',
  'Uint8Array',
  'FontFace',
  'Number',
  'Object',
  'OffscreenCanvas',
  'Promise',
  'ResizeObserver',
  'Notification',
  'Audio',
  'Date',
  'DeviceMotionEvent',
  'MediaMetadata',
  'TextEncoder',
  'URL',
  'URLSearchParams',
  'WebSocket',
  'atob',
  'btoa',
  'cancelAnimationFrame',
  'crypto',
  'decodeURIComponent',
  'document',
  'encodeURIComponent',
  'fetch',
  'createImageBitmap',
  'getComputedStyle',
  'globalThis',
  'localStorage',
  'location',
  'navigator',
  'isNaN',
  'parseFloat',
  'parseInt',
  'performance',
  'requestAnimationFrame',
  'screen',
  'process',
  'structuredClone',
  'window',
]);

const webGpuConstantNamespaces = new Set([
  'GPUBufferUsage',
  'GPUColorWrite',
  'GPUMapMode',
  'GPUShaderStage',
  'GPUTextureUsage',
]);

const canvasElementMembers = new Set([
  'addEventListener',
  'convertToBlob',
  'getBoundingClientRect',
  'getContext',
  'height',
  'removeEventListener',
  'toDataURL',
  'width',
]);

const webGpuDeviceMembers = new Set([
  'createBindGroup',
  'createBindGroupLayout',
  'createBuffer',
  'createCommandEncoder',
  'createPipelineLayout',
  'createRenderPipeline',
  'createSampler',
  'createShaderModule',
  'createTexture',
  'limits',
  'queue',
]);

const webGpuQueueMembers = new Set(['copyExternalImageToTexture', 'submit', 'writeBuffer', 'writeTexture']);
const webGpuCanvasContextMembers = new Set(['configure', 'getCurrentTexture']);
const webGpuLimitsMembers = new Set(['maxBindGroups', 'maxTextureDimension2D', 'minUniformBufferOffsetAlignment']);

export interface TypeRecoveryCatalog {
  functions?: readonly IrFunctionDeclaration[] | undefined;
  types?: Readonly<Record<string, IrType>> | undefined;
}

export function lowerTypeScriptSource(
  sourceFile: ts.SourceFile,
  packageName: string,
  workspaceDirectory: string,
  recoveryCatalog: TypeRecoveryCatalog = {},
): LoweringResult {
  const diagnostics: LoweringDiagnostic[] = [];
  const declarations: IrDeclaration[] = [];
  let accountedDeclarations = 0;
  const erasedLocalTypes = new Set<string>();
  const collectLocalTypes = (node: ts.Node): void => {
    if (ts.isTypeAliasDeclaration(node) && !ts.isSourceFile(node.parent)) erasedLocalTypes.add(node.name.text);
    ts.forEachChild(node, collectLocalTypes);
  };
  collectLocalTypes(sourceFile);
  const externalTypes = new Set<string>();
  const externalValues = new Map<string, { imported: string; specifier: string }>();
  for (const statement of sourceFile.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    const specifier = statement.moduleSpecifier.text;
    if (specifier.startsWith('.') || specifier.startsWith('@flighthq/')) continue;
    if (statement.importClause?.name) {
      externalTypes.add(statement.importClause.name.text);
      externalValues.set(statement.importClause.name.text, {
        imported: 'default',
        specifier,
      });
    }
    const bindings = statement.importClause?.namedBindings;
    if (bindings && ts.isNamedImports(bindings)) {
      for (const element of bindings.elements) {
        externalTypes.add(element.name.text);
        externalValues.set(element.name.text, {
          imported: element.propertyName?.text ?? element.name.text,
          specifier,
        });
      }
    } else if (bindings && ts.isNamespaceImport(bindings)) {
      externalTypes.add(bindings.name.text);
      externalValues.set(bindings.name.text, { imported: '*', specifier });
    }
  }
  const canvasBindingNames = collectPlatformBindingNames(sourceFile, 'CanvasRenderingContext2D', (node, names) => {
    if (isCanvasValueExpression(node, names)) return true;
    return (
      packageName.toLowerCase().includes('canvas') &&
      ts.isPropertyAccessExpression(node) &&
      node.name.text === 'context'
    );
  });
  const canvasElementBindingNames = new Set([
    ...collectPlatformBindingNames(sourceFile, 'HTMLCanvasElement', isCanvasElementValueExpression),
    ...collectPlatformBindingNames(sourceFile, 'OffscreenCanvas', isCanvasElementValueExpression),
  ]);
  const webGpuDeviceBindingNames = collectPlatformBindingNames(sourceFile, 'GPUDevice', (node, names) =>
    isNamedPlatformValueExpression(node, names, 'device'),
  );
  const webGpuQueueBindingNames = collectPlatformBindingNames(sourceFile, 'GPUQueue', (node, names) =>
    isNamedPlatformValueExpression(node, names, 'queue'),
  );
  const webGpuCanvasContextBindingNames = collectPlatformBindingNames(sourceFile, 'GPUCanvasContext', (node, names) =>
    isNamedPlatformValueExpression(node, names, 'context'),
  );
  const webGpuLimitsBindingNames = collectPlatformBindingNames(sourceFile, 'GPUSupportedLimits', (node, names) =>
    isNamedPlatformValueExpression(node, names, 'limits'),
  );
  const webGlBindingNames = collectPlatformBindingNames(sourceFile, 'WebGL2RenderingContext', isWebGlValueExpression);
  const domWindowBindingNames = collectGlobalRootNames(sourceFile, 'window');
  const domDocumentBindingNames = collectGlobalRootNames(sourceFile, 'document');
  const domNavigatorBindingNames = collectGlobalRootNames(sourceFile, 'navigator');
  const context: LoweringContext = {
    asyncTaskExecutions: new WeakMap(),
    canvasBindingNames,
    canvasElementBindingNames,
    classThis: false,
    diagnostics,
    domDocumentBindingNames,
    domNavigatorBindingNames,
    domWindowBindingNames,
    externalTypes,
    externalValues,
    erasedLocalTypes,
    packageName,
    recoveryFunctions: new Map(recoveryCatalog.functions?.map((declaration) => [declaration.name, declaration]) ?? []),
    recoveryTypes: new Map(Object.entries(recoveryCatalog.types ?? {})),
    scopeBindings: new WeakMap(),
    sourceFile,
    typeScopeBindings: new WeakMap(),
    temporaryIndex: 0,
    taskConstructions: [],
    webGpuCanvasContextBindingNames,
    webGpuDeviceBindingNames,
    webGpuLimitsBindingNames,
    webGpuQueueBindingNames,
    webGlBindingNames,
    workspaceDirectory,
  };
  collectLocalRecoveryTypes(context);
  const asyncTasks = collectAsyncTaskScopes(sourceFile, context);

  for (const statement of sourceFile.statements) {
    try {
      if (ts.isFunctionDeclaration(statement) && statement.name && statement.body) {
        declarations.push(lowerFunction(statement, context));
        accountedDeclarations += 1;
      } else if (ts.isFunctionDeclaration(statement) && statement.name) {
        // TypeScript overload signatures are represented by the following implementation declaration.
        accountedDeclarations += 1;
      } else if (ts.isClassDeclaration(statement) && statement.name) {
        const previousClassThis = context.classThis;
        context.classThis = true;
        try {
          declarations.push(lowerClass(statement, context));
        } finally {
          context.classThis = previousClassThis;
        }
        accountedDeclarations += 1;
      } else if (ts.isInterfaceDeclaration(statement)) {
        declarations.push({
          exported: hasModifier(statement, ts.SyntaxKind.ExportKeyword),
          kind: 'type',
          name: statement.name.text,
          origin: origin(statement, context),
          type: {
            extends:
              statement.heritageClauses
                ?.filter((clause) => clause.token === ts.SyntaxKind.ExtendsKeyword)
                .flatMap((clause) => clause.types.map((item) => lowerExpressionWithTypeArguments(item, context))) ?? [],
            fields: lowerTypeMembers(statement.members, context),
            kind: 'anonymous',
          },
          typeParameters: statement.typeParameters?.map((parameter) => parameter.name.text) ?? [],
        });
        accountedDeclarations += 1;
      } else if (ts.isTypeAliasDeclaration(statement)) {
        declarations.push({
          exported: hasModifier(statement, ts.SyntaxKind.ExportKeyword),
          kind: 'type',
          name: statement.name.text,
          origin: origin(statement, context),
          type: lowerType(statement.type, context),
          typeParameters: statement.typeParameters?.map((parameter) => parameter.name.text) ?? [],
        });
        accountedDeclarations += 1;
      } else if (ts.isEnumDeclaration(statement)) {
        declarations.push({
          exported: hasModifier(statement, ts.SyntaxKind.ExportKeyword),
          kind: 'enum',
          members: statement.members.map((member) => ({
            initializer: member.initializer ? lowerExpression(member.initializer, context) : undefined,
            name: propertyName(member.name, context),
          })),
          methods: [],
          name: statement.name.text,
          origin: origin(statement, context),
        });
        accountedDeclarations += 1;
      } else if (ts.isVariableStatement(statement)) {
        const exported = hasModifier(statement, ts.SyntaxKind.ExportKeyword);
        const mutable = (statement.declarationList.flags & ts.NodeFlags.Const) === 0;
        for (const declaration of statement.declarationList.declarations) {
          if (!ts.isIdentifier(declaration.name)) unsupported(declaration.name, context, 'binding pattern declaration');
          declarations.push({
            exported,
            initializer: declaration.initializer ? lowerExpression(declaration.initializer, context) : undefined,
            kind: 'variable',
            mutable,
            name: declaration.name.text,
            origin: origin(statement, context),
            type: declaration.type ? lowerType(declaration.type, context) : undefined,
          });
        }
        accountedDeclarations += 1;
      } else if (ts.isModuleDeclaration(statement)) {
        if (!mergeNamespace(statement, declarations, context)) {
          unsupported(statement, context, `declaration ${ts.SyntaxKind[statement.kind] ?? statement.kind}`);
        }
        accountedDeclarations += 1;
      }
    } catch (error) {
      if (!(error instanceof UnsupportedSyntaxError)) throw error;
    }
  }

  return {
    accountedDeclarations,
    asyncTasks,
    declarations,
    diagnostics,
    taskConstructions: context.taskConstructions.sort(
      (left, right) =>
        left.origin.source.localeCompare(right.origin.source) ||
        left.origin.line - right.origin.line ||
        left.origin.column - right.origin.column,
    ),
  };
}

function collectAsyncTaskScopes(sourceFile: ts.SourceFile, context: LoweringContext): IrAsyncTaskScope[] {
  const scopes: IrAsyncTaskScope[] = [];
  const visit = (node: ts.Node): void => {
    if (isAsyncFunctionLike(node)) {
      const sourceOrigin = origin(node, context);
      const execution = {
        kind: 'portableTask',
        origin: {
          ...sourceOrigin,
          lexicalPath: asyncTaskLexicalPath(node, sourceFile, sourceOrigin.fingerprint),
        },
      } satisfies Extract<IrFunctionExecution, { kind: 'portableTask' }>;
      context.asyncTaskExecutions.set(node, execution);
      const output = asyncTaskOutput(node, context);
      scopes.push({
        execution,
        matchesLegacyErasurePath:
          ts.isFunctionDeclaration(node) && (ts.isSourceFile(node.parent) || ts.isModuleBlock(node.parent)),
        operations: collectAsyncTaskOperations(node, context),
        output,
      });
      context.taskConstructions.push({ kind: 'async-scope', origin: execution.origin, output });
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  return scopes.sort(
    (left, right) =>
      left.execution.origin.source.localeCompare(right.execution.origin.source) ||
      left.execution.origin.line - right.execution.origin.line ||
      left.execution.origin.column - right.execution.origin.column,
  );
}

function asyncTaskOutput(
  node: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrType {
  if (node.type) {
    const returns = lowerType(node.type, context);
    return returns.kind === 'task' ? returns.output : { kind: 'dynamic' };
  }
  const contextual = contextualAsyncTaskOutput(node, context);
  if (contextual) return contextual;
  return inferAsyncTaskOutput(node, context) ?? { kind: 'dynamic' };
}

function asyncTaskType(
  node: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrType {
  return { kind: 'task', output: asyncTaskOutput(node, context) };
}

function collectLocalRecoveryTypes(context: LoweringContext): void {
  for (const statement of context.sourceFile.statements) {
    const previousDiagnostics = context.diagnostics.length;
    try {
      if (ts.isInterfaceDeclaration(statement)) {
        context.recoveryTypes.set(statement.name.text, {
          extends:
            statement.heritageClauses
              ?.filter((clause) => clause.token === ts.SyntaxKind.ExtendsKeyword)
              .flatMap((clause) => clause.types.map((item) => lowerExpressionWithTypeArguments(item, context))) ?? [],
          fields: lowerTypeMembers(statement.members, context),
          kind: 'anonymous',
        });
      } else if (ts.isTypeAliasDeclaration(statement)) {
        context.recoveryTypes.set(statement.name.text, lowerType(statement.type, context));
      }
    } catch (error) {
      if (!(error instanceof UnsupportedSyntaxError)) throw error;
      context.diagnostics.splice(previousDiagnostics);
    }
  }
}

function contextualAsyncTaskOutput(
  node: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrType | undefined {
  const contextual = contextualFunctionType(node, context);
  const resolved = contextual ? resolveRecoveryType(contextual, context) : undefined;
  if (resolved?.kind !== 'function') return undefined;
  return resolved.returns.kind === 'task' ? resolved.returns.output : undefined;
}

function contextualFunctionType(
  node: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrType | undefined {
  if (ts.isMethodDeclaration(node) && ts.isObjectLiteralExpression(node.parent)) {
    return recoveryObjectFieldType(node.parent, propertyName(node.name, context), context);
  }
  if (ts.isArrowFunction(node) || ts.isFunctionExpression(node)) {
    const arrayMap = contextualArrayMapCallbackType(node, context);
    if (arrayMap) return arrayMap;
    return contextualExpressionType(node, context, new Set());
  }
  return undefined;
}

function contextualArrayMapCallbackType(
  node: ts.ArrowFunction | ts.FunctionExpression,
  context: LoweringContext,
): IrType | undefined {
  const call = node.parent;
  if (
    !ts.isCallExpression(call) ||
    call.arguments[0] !== node ||
    !ts.isPropertyAccessExpression(call.expression) ||
    call.expression.name.text !== 'map'
  ) {
    return undefined;
  }
  const input = inferRecoveryExpressionType(call.expression.expression, context, new Set());
  const resolvedInput = input ? resolveRecoveryType(input, context) : undefined;
  const output = contextualExpressionType(call, context, new Set());
  const resolvedOutput = output ? resolveRecoveryType(output, context) : undefined;
  if (resolvedInput?.kind !== 'array' || resolvedOutput?.kind !== 'array') return undefined;
  return {
    kind: 'function',
    parameters: [resolvedInput.element, { kind: 'primitive', name: 'Float' }, resolvedInput],
    returns: resolvedOutput.element,
  };
}

function contextualExpressionType(
  node: ts.Expression,
  context: LoweringContext,
  visited: Set<ts.Node>,
): IrType | undefined {
  if (visited.has(node)) return undefined;
  visited.add(node);
  const parent = node.parent;
  if (ts.isParenthesizedExpression(parent) && parent.expression === node) {
    return contextualExpressionType(parent, context, visited);
  }
  if ((ts.isAsExpression(parent) || ts.isTypeAssertionExpression(parent)) && parent.expression === node) {
    return parent.type.kind === ts.SyntaxKind.ConstKeyword
      ? contextualExpressionType(parent, context, visited)
      : lowerType(parent.type, context);
  }
  if (ts.isSatisfiesExpression(parent) && parent.expression === node) return lowerType(parent.type, context);
  if (ts.isVariableDeclaration(parent) && parent.initializer === node) {
    return parent.type ? lowerType(parent.type, context) : undefined;
  }
  if (ts.isReturnStatement(parent) && parent.expression === node) {
    const owner = findEnclosingFunction(parent);
    if (!owner) return undefined;
    return contextualReturnExpressionType(owner, context);
  }
  if (ts.isArrowFunction(parent) && parent.body === node) {
    return contextualReturnExpressionType(parent, context);
  }
  if (ts.isPropertyAssignment(parent) && parent.initializer === node && ts.isObjectLiteralExpression(parent.parent)) {
    return recoveryObjectFieldType(parent.parent, propertyName(parent.name, context), context);
  }
  if (ts.isCallExpression(parent)) {
    const index = parent.arguments.indexOf(node);
    if (index >= 0) {
      if (globalPromiseMethod(parent, context) === 'all' && index === 0) {
        const contextual = contextualExpressionType(parent, context, visited);
        const output = contextual?.kind === 'task' ? contextual.output : contextual;
        if (output?.kind === 'array') {
          return { element: { kind: 'task', output: output.element }, kind: 'array' };
        }
      }
      return recoveryCallParameterType(parent, index, context);
    }
  }
  return undefined;
}

function contextualReturnExpressionType(
  node: ts.FunctionLikeDeclaration,
  context: LoweringContext,
): IrType | undefined {
  const returns = declaredOrContextualFunctionReturn(node, context);
  return returns?.kind === 'task' && hasModifier(node, ts.SyntaxKind.AsyncKeyword) ? returns.output : returns;
}

function declaredOrContextualFunctionReturn(
  node: ts.FunctionLikeDeclaration,
  context: LoweringContext,
): IrType | undefined {
  if (node.type) return lowerType(node.type, context);
  if (ts.isArrowFunction(node) || ts.isFunctionExpression(node) || ts.isMethodDeclaration(node)) {
    const contextual = contextualFunctionType(node, context);
    const resolved = contextual ? resolveRecoveryType(contextual, context) : undefined;
    return resolved?.kind === 'function' ? resolved.returns : undefined;
  }
  return undefined;
}

function recoveryObjectFieldType(
  object: ts.ObjectLiteralExpression,
  name: string,
  context: LoweringContext,
): IrType | undefined {
  const declared = contextualExpressionType(object, context, new Set());
  const target = declared && declared.kind !== 'dynamic' ? declared : inferCatalogObjectType(object, context);
  return target ? recoveryFieldType(target, name, context, new Set()) : undefined;
}

function recoveryFieldType(
  type: IrType,
  name: string,
  context: LoweringContext,
  visited: Set<string>,
): IrType | undefined {
  if (type.kind === 'nullable') return recoveryFieldType(type.inner, name, context, visited);
  if (type.kind === 'union') {
    const candidates = type.variants.flatMap((variant) => {
      const field = recoveryFieldType(variant, name, context, new Set(visited));
      return field ? [field] : [];
    });
    const first = candidates[0];
    return first && candidates.every((candidate) => recoveryTypeKey(candidate) === recoveryTypeKey(first))
      ? first
      : undefined;
  }
  if (type.kind === 'named') {
    if (visited.has(type.name)) return undefined;
    const declaration = context.recoveryTypes.get(type.name);
    return declaration ? recoveryFieldType(declaration, name, context, new Set([...visited, type.name])) : undefined;
  }
  if (type.kind !== 'anonymous') return undefined;
  const own = type.fields.find((field) => field.name === name);
  if (own) return own.type;
  const inherited = type.extends.flatMap((base) => {
    const field = recoveryFieldType(base, name, context, new Set(visited));
    return field ? [field] : [];
  });
  const first = inherited[0];
  return first && inherited.every((field) => recoveryTypeKey(field) === recoveryTypeKey(first)) ? first : undefined;
}

function resolveRecoveryType(
  type: IrType,
  context: LoweringContext,
  visited: Set<string> = new Set(),
): IrType | undefined {
  if (type.kind !== 'named') return type;
  if (visited.has(type.name)) return type;
  const declaration = context.recoveryTypes.get(type.name);
  return declaration ? resolveRecoveryType(declaration, context, new Set([...visited, type.name])) : type;
}

function inferCatalogObjectType(object: ts.ObjectLiteralExpression, context: LoweringContext): IrType | undefined {
  const names = objectLiteralRecoveryNames(object, context);
  if (!names) return undefined;
  const matches = [...context.recoveryTypes.entries()].flatMap(([name, type]) => {
    const resolved = resolveRecoveryType(type, context);
    if (resolved?.kind !== 'anonymous') return [];
    const fields = flattenRecoveryFields(resolved, context, new Set([name]));
    if (
      [...names].some((fieldName) => !fields.some((field) => field.name === fieldName)) ||
      fields.some((field) => !field.optional && !names.has(field.name))
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
  if (matches.length === 0) return undefined;
  const bestScore = Math.min(...matches.map((match) => match.missing));
  const best = matches.filter((match) => match.missing === bestScore);
  const identities = new Map(
    best.map((match) => [recoveryTypeKey(resolveRecoveryType(match.type, context) ?? match.type), match.type]),
  );
  return identities.size === 1 ? [...identities.values()][0] : undefined;
}

function objectLiteralRecoveryNames(
  object: ts.ObjectLiteralExpression,
  context: LoweringContext,
): ReadonlySet<string> | undefined {
  const names = new Set<string>();
  for (const member of object.properties) {
    if (ts.isSpreadAssignment(member) || ts.isComputedPropertyName(member.name)) return undefined;
    if (
      ts.isPropertyAssignment(member) ||
      ts.isShorthandPropertyAssignment(member) ||
      ts.isMethodDeclaration(member) ||
      ts.isGetAccessorDeclaration(member) ||
      ts.isSetAccessorDeclaration(member)
    ) {
      names.add(propertyName(member.name, context));
      continue;
    }
    return undefined;
  }
  return names;
}

function flattenRecoveryFields(
  type: Extract<IrType, { kind: 'anonymous' }>,
  context: LoweringContext,
  visited: Set<string>,
): IrTypeField[] {
  const fields = new Map(type.fields.map((field) => [field.name, field]));
  for (const base of type.extends) {
    if (base.kind === 'named' && visited.has(base.name)) continue;
    const nextVisited = base.kind === 'named' ? new Set([...visited, base.name]) : new Set(visited);
    const resolved = resolveRecoveryType(base, context);
    if (resolved?.kind !== 'anonymous') continue;
    for (const field of flattenRecoveryFields(resolved, context, nextVisited)) {
      if (!fields.has(field.name)) fields.set(field.name, field);
    }
  }
  return [...fields.values()];
}

function recoveryCallParameterType(
  call: ts.CallExpression,
  index: number,
  context: LoweringContext,
): IrType | undefined {
  const signature = recoveryCallSignature(call.expression, context);
  if (signature?.kind !== 'function') return undefined;
  return signature.parameters[index] ?? signature.parameters.at(-1);
}

function recoveryCallSignature(
  expression: ts.Expression,
  context: LoweringContext,
): Extract<IrType, { kind: 'function' }> | undefined {
  if (ts.isIdentifier(expression)) {
    const local = findLocalFunctionDeclaration(expression.text, expression, context.sourceFile);
    if (local) {
      return {
        kind: 'function',
        parameters: local.parameters.map((parameter) => lowerParameter(parameter, context).type),
        returns: local.type ? lowerType(local.type, context) : { kind: 'dynamic' },
      };
    }
    const semantic = context.recoveryFunctions.get(expression.text);
    if (semantic) {
      return {
        kind: 'function',
        parameters: semantic.parameters.map(callbackParameterType),
        returns: semantic.returns,
      };
    }
  }
  const inferred = inferRecoveryExpressionType(expression, context, new Set());
  const resolved = inferred ? resolveRecoveryType(inferred, context) : undefined;
  return resolved?.kind === 'function' ? resolved : undefined;
}

function findLocalFunctionDeclaration(
  name: string,
  from: ts.Node,
  sourceFile: ts.SourceFile,
): ts.FunctionDeclaration | undefined {
  for (let scope: ts.Node | undefined = from.parent; scope; scope = scope.parent) {
    const statements = ts.isSourceFile(scope) || ts.isBlock(scope) ? scope.statements : undefined;
    const declaration = statements?.find(
      (statement): statement is ts.FunctionDeclaration =>
        ts.isFunctionDeclaration(statement) && statement.name?.text === name,
    );
    if (declaration) return declaration;
    if (scope === sourceFile) break;
  }
  return undefined;
}

function inferAsyncTaskOutput(
  node: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrType | undefined {
  const expressions: Array<ts.Expression | undefined> = [];
  if (ts.isArrowFunction(node) && !ts.isBlock(node.body)) {
    expressions.push(node.body);
  } else if (node.body) {
    const visit = (candidate: ts.Node): void => {
      if (candidate !== node.body && ts.isFunctionLike(candidate)) return;
      if (ts.isReturnStatement(candidate)) {
        expressions.push(candidate.expression);
        return;
      }
      ts.forEachChild(candidate, visit);
    };
    visit(node.body);
  }
  if (expressions.length === 0) return { kind: 'primitive', name: 'Void' };
  if (expressions.some((expression) => !expression)) return undefined;
  const types = expressions.flatMap((expression) => {
    const inferred = expression ? inferRecoveryExpressionType(expression, context, new Set()) : undefined;
    const output = inferred?.kind === 'task' ? inferred.output : inferred;
    return output && !recoveryTypeContainsDynamic(output) ? [output] : [];
  });
  return types.length === expressions.length ? commonType(types) : undefined;
}

function inferRecoveryExpressionType(
  node: ts.Expression,
  context: LoweringContext,
  visited: Set<ts.Node>,
): IrType | undefined {
  if (visited.has(node)) return undefined;
  visited.add(node);
  if (ts.isParenthesizedExpression(node) || ts.isNonNullExpression(node)) {
    return inferRecoveryExpressionType(node.expression, context, visited);
  }
  if (ts.isAsExpression(node) || ts.isTypeAssertionExpression(node)) return lowerType(node.type, context);
  if (ts.isSatisfiesExpression(node)) return lowerType(node.type, context);
  if (ts.isAwaitExpression(node)) {
    const awaited = inferRecoveryExpressionType(node.expression, context, visited);
    return awaited?.kind === 'task' ? awaited.output : awaited;
  }
  if (ts.isNumericLiteral(node) || ts.isPostfixUnaryExpression(node)) {
    return { kind: 'primitive', name: 'Float' };
  }
  if (ts.isPrefixUnaryExpression(node)) {
    if (node.operator === ts.SyntaxKind.ExclamationToken) return { kind: 'primitive', name: 'Bool' };
    return { kind: 'primitive', name: 'Float' };
  }
  if (ts.isTypeOfExpression(node)) return { kind: 'primitive', name: 'String' };
  if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node) || ts.isTemplateExpression(node)) {
    return { kind: 'primitive', name: 'String' };
  }
  if (node.kind === ts.SyntaxKind.TrueKeyword || node.kind === ts.SyntaxKind.FalseKeyword) {
    return { kind: 'primitive', name: 'Bool' };
  }
  if (ts.isIdentifier(node)) return inferRecoveryIdentifierType(node, context, visited);
  if (ts.isObjectLiteralExpression(node)) {
    const semantic = inferCatalogObjectType(node, context);
    if (semantic) return semantic;
    const fields = node.properties.flatMap((member) => {
      if (ts.isSpreadAssignment(member)) {
        const spread = inferRecoveryExpressionType(member.expression, context, new Set(visited));
        const resolved = spread ? resolveRecoveryType(spread, context) : undefined;
        return resolved?.kind === 'anonymous' ? flattenRecoveryFields(resolved, context, new Set()) : [];
      }
      if (!ts.isPropertyAssignment(member) && !ts.isShorthandPropertyAssignment(member)) return [];
      if (ts.isComputedPropertyName(member.name)) return [];
      const value = ts.isShorthandPropertyAssignment(member) ? member.name : member.initializer;
      const type = inferRecoveryExpressionType(value, context, new Set(visited));
      return type && !recoveryTypeContainsDynamic(type)
        ? [{ name: propertyName(member.name, context), optional: false, type }]
        : [];
    });
    if (fields.length !== node.properties.length) return undefined;
    return {
      extends: [],
      fields: fields.sort((left, right) => left.name.localeCompare(right.name)),
      kind: 'anonymous',
    };
  }
  if (ts.isArrayLiteralExpression(node)) {
    if (node.elements.length === 0) return undefined;
    const elements = node.elements.flatMap((element) => {
      const type = inferRecoveryExpressionType(element, context, new Set(visited));
      return type && !recoveryTypeContainsDynamic(type) ? [type] : [];
    });
    return elements.length === node.elements.length ? { element: commonType(elements), kind: 'array' } : undefined;
  }
  if (ts.isConditionalExpression(node)) {
    const whenTrue = inferRecoveryExpressionType(node.whenTrue, context, new Set(visited));
    const whenFalse = inferRecoveryExpressionType(node.whenFalse, context, new Set(visited));
    if (node.whenTrue.kind === ts.SyntaxKind.NullKeyword && whenFalse) return { inner: whenFalse, kind: 'nullable' };
    if (node.whenFalse.kind === ts.SyntaxKind.NullKeyword && whenTrue) return { inner: whenTrue, kind: 'nullable' };
    return whenTrue && whenFalse ? commonType([whenTrue, whenFalse]) : undefined;
  }
  if (ts.isBinaryExpression(node)) {
    const operator = node.operatorToken.kind;
    if (
      operator === ts.SyntaxKind.EqualsEqualsToken ||
      operator === ts.SyntaxKind.EqualsEqualsEqualsToken ||
      operator === ts.SyntaxKind.ExclamationEqualsToken ||
      operator === ts.SyntaxKind.ExclamationEqualsEqualsToken ||
      operator === ts.SyntaxKind.LessThanToken ||
      operator === ts.SyntaxKind.LessThanEqualsToken ||
      operator === ts.SyntaxKind.GreaterThanToken ||
      operator === ts.SyntaxKind.GreaterThanEqualsToken
    ) {
      return { kind: 'primitive', name: 'Bool' };
    }
    const left = inferRecoveryExpressionType(node.left, context, new Set(visited));
    const right = inferRecoveryExpressionType(node.right, context, new Set(visited));
    return left && right ? commonType([left, right]) : (left ?? right);
  }
  if (ts.isCallExpression(node)) {
    if (
      ts.isPropertyAccessExpression(node.expression) &&
      ts.isIdentifier(node.expression.expression) &&
      node.expression.expression.text === 'Promise' &&
      node.expression.name.text === 'resolve'
    ) {
      const value = node.arguments[0]
        ? inferRecoveryExpressionType(node.arguments[0], context, new Set(visited))
        : ({ kind: 'primitive', name: 'Void' } as const);
      return value ? { kind: 'task', output: value } : undefined;
    }
    return recoveryCallSignature(node.expression, context)?.returns;
  }
  if (ts.isPropertyAccessExpression(node)) {
    const object = inferRecoveryExpressionType(node.expression, context, new Set(visited));
    return object ? recoveryFieldType(object, node.name.text, context, new Set()) : undefined;
  }
  if (ts.isNewExpression(node) && ts.isIdentifier(node.expression)) {
    const lowered = lowerType(
      ts.factory.createTypeReferenceNode(
        node.expression.text,
        node.typeArguments ? [...node.typeArguments] : undefined,
      ),
      context,
    );
    return recoveryTypeContainsDynamic(lowered) ? undefined : lowered;
  }
  return undefined;
}

function inferRecoveryIdentifierType(
  identifier: ts.Identifier,
  context: LoweringContext,
  visited: Set<ts.Node>,
): IrType | undefined {
  if (identifier.text === 'undefined') return { kind: 'primitive', name: 'Void' };
  for (let current: ts.Node | undefined = identifier.parent; current; current = current.parent) {
    if (ts.isFunctionLike(current)) {
      const parameter = current.parameters.find(
        (candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === identifier.text,
      );
      if (parameter) {
        return parameter.type
          ? lowerType(parameter.type, context)
          : parameter.initializer
            ? inferRecoveryExpressionType(parameter.initializer, context, new Set(visited))
            : contextualParameterType(parameter, context);
      }
    }
  }
  let best: ts.VariableDeclaration | undefined;
  const visit = (node: ts.Node): void => {
    if (node !== context.sourceFile && ts.isFunctionLike(node)) {
      if (!(node.pos <= identifier.pos && identifier.end <= node.end)) return;
    }
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.name.text === identifier.text &&
      node.pos < identifier.pos &&
      (!best || node.pos > best.pos)
    ) {
      best = node;
    }
    ts.forEachChild(node, visit);
  };
  visit(context.sourceFile);
  if (best) {
    if (best.type) return lowerType(best.type, context);
    if (best.initializer) return inferRecoveryExpressionType(best.initializer, context, new Set(visited));
  }
  const functionDeclaration = findLocalFunctionDeclaration(identifier.text, identifier, context.sourceFile);
  if (functionDeclaration) {
    return {
      kind: 'function',
      parameters: functionDeclaration.parameters.map((parameter) => lowerParameter(parameter, context).type),
      returns: functionDeclaration.type ? lowerType(functionDeclaration.type, context) : { kind: 'dynamic' },
    };
  }
  return undefined;
}

function findEnclosingFunction(
  node: ts.Node,
): ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration | undefined {
  for (let current: ts.Node | undefined = node.parent; current; current = current.parent) {
    if (
      ts.isArrowFunction(current) ||
      ts.isFunctionDeclaration(current) ||
      ts.isFunctionExpression(current) ||
      ts.isMethodDeclaration(current)
    ) {
      return current;
    }
  }
  return undefined;
}

function recoveryTypeContainsDynamic(type: IrType): boolean {
  switch (type.kind) {
    case 'dynamic':
      return !type.portable;
    case 'anonymous':
      return (
        type.extends.some(recoveryTypeContainsDynamic) ||
        type.fields.some((field) => recoveryTypeContainsDynamic(field.type))
      );
    case 'array':
      return recoveryTypeContainsDynamic(type.element);
    case 'function':
      return type.parameters.some(recoveryTypeContainsDynamic) || recoveryTypeContainsDynamic(type.returns);
    case 'named':
      return type.arguments.some(recoveryTypeContainsDynamic);
    case 'nullable':
      return recoveryTypeContainsDynamic(type.inner);
    case 'task':
      return recoveryTypeContainsDynamic(type.output);
    case 'union':
      return type.variants.some(recoveryTypeContainsDynamic);
    case 'primitive':
      return false;
  }
}

function recoveryTypeKey(type: IrType): string {
  return JSON.stringify(type);
}

function isAsyncFunctionLike(
  node: ts.Node,
): node is ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration {
  return (
    (ts.isArrowFunction(node) ||
      ts.isFunctionDeclaration(node) ||
      ts.isFunctionExpression(node) ||
      ts.isMethodDeclaration(node)) &&
    hasModifier(node, ts.SyntaxKind.AsyncKeyword)
  );
}

function functionExecution(node: ts.Node, context: LoweringContext): IrFunctionExecution {
  return context.asyncTaskExecutions.get(node) ?? { kind: 'sync' };
}

function asyncTaskLexicalPath(node: ts.Node, sourceFile: ts.SourceFile, fingerprint: string): string {
  const labels: string[] = [];
  let current: ts.Node | undefined = node;
  while (current && !ts.isSourceFile(current)) {
    const label = lexicalNodeLabel(current, sourceFile);
    if (label && labels[0] !== label) labels.unshift(label);
    current = current.parent;
  }
  if (!asyncTaskHasOwnLexicalLabel(node)) {
    labels.push(`anonymous:${fingerprint.slice('sha256:'.length, 'sha256:'.length + 12)}`);
  }
  return labels.join('.');
}

function asyncTaskHasOwnLexicalLabel(node: ts.Node): boolean {
  if ((ts.isFunctionDeclaration(node) || ts.isFunctionExpression(node)) && node.name) {
    return true;
  }
  if (ts.isMethodDeclaration(node)) return true;
  const parent = node.parent;
  return (
    (ts.isVariableDeclaration(parent) && parent.initializer === node && ts.isIdentifier(parent.name)) ||
    (ts.isPropertyAssignment(parent) && parent.initializer === node)
  );
}

function lexicalNodeLabel(node: ts.Node, sourceFile: ts.SourceFile): string | undefined {
  if (ts.isClassDeclaration(node) || ts.isFunctionDeclaration(node) || ts.isFunctionExpression(node)) {
    return node.name?.text;
  }
  if (ts.isMethodDeclaration(node) || ts.isPropertyAssignment(node)) {
    return lexicalPropertyName(node.name, sourceFile);
  }
  if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name)) return node.name.text;
  return undefined;
}

function lexicalPropertyName(node: ts.PropertyName, sourceFile: ts.SourceFile): string {
  if (ts.isIdentifier(node) || ts.isStringLiteral(node) || ts.isNumericLiteral(node)) return node.text;
  return node.getText(sourceFile).replace(/[^A-Za-z0-9_]/gu, '_');
}

function collectAsyncTaskOperations(
  scope: ts.ArrowFunction | ts.FunctionDeclaration | ts.FunctionExpression | ts.MethodDeclaration,
  context: LoweringContext,
): IrAsyncTaskOperations {
  const operations: IrAsyncTaskOperations = {
    asyncIterations: 0,
    awaits: 0,
    promiseAll: 0,
    promiseAllSettled: 0,
    promiseCatch: 0,
    promiseFinally: 0,
    promiseReject: 0,
    promiseResolve: 0,
    promiseThen: 0,
    voidExpressions: 0,
  };
  const visit = (node: ts.Node): void => {
    if (node !== scope && isAsyncFunctionLike(node)) return;
    if (ts.isAwaitExpression(node)) operations.awaits++;
    if (ts.isForOfStatement(node) && node.awaitModifier) operations.asyncIterations++;
    if (ts.isVoidExpression(node)) operations.voidExpressions++;
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression)) {
      const method = node.expression.name.text;
      const owner = node.expression.expression;
      if (ts.isIdentifier(owner) && owner.text === 'Promise' && !isLexicallyBound(owner, context)) {
        if (method === 'all') operations.promiseAll++;
        if (method === 'allSettled') operations.promiseAllSettled++;
        if (method === 'reject') operations.promiseReject++;
        if (method === 'resolve') operations.promiseResolve++;
      } else {
        if (method === 'then') operations.promiseThen++;
        if (method === 'catch') operations.promiseCatch++;
        if (method === 'finally') operations.promiseFinally++;
      }
    }
    ts.forEachChild(node, visit);
  };
  visit(scope);
  return operations;
}

function collectPlatformBindingNames(
  sourceFile: ts.SourceFile,
  typeName: string,
  isBindingValue: (node: ts.Expression, names: ReadonlySet<string>) => boolean,
): ReadonlySet<string> {
  const names = new Set<string>();
  const factories = new Set<string>();
  const collectFactories = (node: ts.Node): void => {
    if (ts.isFunctionDeclaration(node) && node.name && node.type?.getText(sourceFile).includes(typeName)) {
      factories.add(node.name.text);
    }
    ts.forEachChild(node, collectFactories);
  };
  collectFactories(sourceFile);
  const visit = (node: ts.Node): void => {
    if (
      (ts.isParameter(node) || ts.isVariableDeclaration(node) || ts.isPropertyDeclaration(node)) &&
      ts.isIdentifier(node.name)
    ) {
      const declaredType = node.type?.getText(sourceFile);
      if (
        declaredType?.includes(typeName) ||
        (typeName === 'WebGL2RenderingContext' && node.name.text === 'gl') ||
        (node.initializer &&
          ts.isCallExpression(node.initializer) &&
          ts.isIdentifier(node.initializer.expression) &&
          factories.has(node.initializer.expression.text)) ||
        (node.initializer && isBindingValue(node.initializer, names))
      ) {
        names.add(node.name.text);
      }
    }
    ts.forEachChild(node, visit);
  };
  // Repeat once so a simple alias can refer to a binding declared later in the file.
  visit(sourceFile);
  visit(sourceFile);
  return names;
}

function collectGlobalRootNames(sourceFile: ts.SourceFile, root: string): ReadonlySet<string> {
  const names = new Set([root]);
  const isRootValue = (node: ts.Expression): boolean => {
    if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
      return isRootValue(node.expression);
    }
    return ts.isIdentifier(node) && names.has(node.text);
  };
  const visit = (node: ts.Node): void => {
    if (
      ts.isVariableDeclaration(node) &&
      ts.isIdentifier(node.name) &&
      node.initializer &&
      isRootValue(node.initializer)
    ) {
      names.add(node.name.text);
    }
    ts.forEachChild(node, visit);
  };
  visit(sourceFile);
  visit(sourceFile);
  return names;
}

function isBoundGlobalRootExpression(
  node: ts.Expression,
  context: LoweringContext,
  root: string,
  names: ReadonlySet<string>,
): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isBoundGlobalRootExpression(node.expression, context, root, names);
  }
  if (!ts.isIdentifier(node) || !names.has(node.text)) return false;
  return node.text !== root || !isLexicallyBound(node, context);
}

function isWebGlValueExpression(node: ts.Expression, names: ReadonlySet<string>): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isWebGlValueExpression(node.expression, names);
  }
  if (ts.isIdentifier(node)) return names.has(node.text);
  return ts.isPropertyAccessExpression(node) && node.name.text === 'gl';
}

function isCanvasValueExpression(node: ts.Expression, names: ReadonlySet<string>): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isCanvasValueExpression(node.expression, names);
  }
  if (ts.isIdentifier(node)) return names.has(node.text);
  if (ts.isPropertyAccessExpression(node) && node.name.text === 'ctx') return true;
  return (
    ts.isCallExpression(node) &&
    ts.isPropertyAccessExpression(node.expression) &&
    node.expression.name.text === 'getContext' &&
    node.arguments[0] !== undefined &&
    ts.isStringLiteral(node.arguments[0]) &&
    node.arguments[0].text === '2d'
  );
}

function isCanvasElementValueExpression(node: ts.Expression, names: ReadonlySet<string>): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isCanvasElementValueExpression(node.expression, names);
  }
  if (ts.isIdentifier(node)) return names.has(node.text);
  if (ts.isPropertyAccessExpression(node)) return node.name.text === 'canvas';
  if (ts.isNewExpression(node) && ts.isIdentifier(node.expression)) {
    return node.expression.text === 'OffscreenCanvas';
  }
  return (
    ts.isCallExpression(node) &&
    ts.isPropertyAccessExpression(node.expression) &&
    node.expression.name.text === 'createElement' &&
    node.arguments[0] !== undefined &&
    ts.isStringLiteral(node.arguments[0]) &&
    node.arguments[0].text === 'canvas'
  );
}

function isBoundCanvasElementExpression(node: ts.Expression, context: LoweringContext): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isBoundCanvasElementExpression(node.expression, context);
  }
  if (ts.isIdentifier(node)) {
    const parameter = findEnclosingParameter(node);
    if (parameter?.type) {
      const type = parameter.type.getText(context.sourceFile);
      if (type.includes('HTMLCanvasElement') || type.includes('OffscreenCanvas')) return true;
    }
  }
  return isCanvasElementValueExpression(node, context.canvasElementBindingNames);
}

function isNamedPlatformValueExpression(
  node: ts.Expression,
  names: ReadonlySet<string>,
  propertyName: string,
): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isNamedPlatformValueExpression(node.expression, names, propertyName);
  }
  if (ts.isIdentifier(node)) return names.has(node.text);
  return ts.isPropertyAccessExpression(node) && node.name.text === propertyName;
}

function isBoundNamedPlatformExpression(
  node: ts.Expression,
  context: LoweringContext,
  typeName: string,
  names: ReadonlySet<string>,
  propertyName: string,
): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isBoundNamedPlatformExpression(node.expression, context, typeName, names, propertyName);
  }
  if (ts.isIdentifier(node)) {
    const parameter = findEnclosingParameter(node);
    if (parameter?.type?.getText(context.sourceFile).includes(typeName)) return true;
    if (context.packageName.toLowerCase().includes('wgpu') && node.text === propertyName) return true;
  }
  return isNamedPlatformValueExpression(node, names, propertyName);
}

function isBoundPlatformExpression(
  node: ts.Expression,
  context: LoweringContext,
  typeName: 'CanvasRenderingContext2D' | 'WebGL2RenderingContext',
): boolean {
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isNonNullExpression(node)) {
    return isBoundPlatformExpression(node.expression, context, typeName);
  }
  if (ts.isIdentifier(node)) {
    const parameter = findEnclosingParameter(node);
    if (parameter) {
      if (parameter.type?.getText(context.sourceFile).includes(typeName)) return true;
      return (
        typeName === 'WebGL2RenderingContext' &&
        ts.isIdentifier(parameter.name) &&
        parameter.name.text === 'gl' &&
        context.packageName.toLowerCase().includes('-gl')
      );
    }
    const names = typeName === 'CanvasRenderingContext2D' ? context.canvasBindingNames : context.webGlBindingNames;
    return names.has(node.text);
  }
  return typeName === 'CanvasRenderingContext2D'
    ? isCanvasValueExpression(node, context.canvasBindingNames)
    : isWebGlValueExpression(node, context.webGlBindingNames);
}

function findEnclosingParameter(identifier: ts.Identifier): ts.ParameterDeclaration | undefined {
  let current: ts.Node | undefined = identifier.parent;
  while (current) {
    if (ts.isFunctionLike(current)) {
      const parameter = current.parameters.find(
        (candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === identifier.text,
      );
      if (parameter) return parameter;
    }
    current = current.parent;
  }
  return undefined;
}

interface LoweringContext {
  asyncTaskExecutions: WeakMap<ts.Node, Extract<IrFunctionExecution, { kind: 'portableTask' }>>;
  canvasBindingNames: ReadonlySet<string>;
  canvasElementBindingNames: ReadonlySet<string>;
  classThis: boolean;
  diagnostics: LoweringDiagnostic[];
  domDocumentBindingNames: ReadonlySet<string>;
  domNavigatorBindingNames: ReadonlySet<string>;
  domWindowBindingNames: ReadonlySet<string>;
  externalTypes: ReadonlySet<string>;
  externalValues: ReadonlyMap<string, { imported: string; specifier: string }>;
  erasedLocalTypes: ReadonlySet<string>;
  packageName: string;
  recoveryFunctions: ReadonlyMap<string, IrFunctionDeclaration>;
  recoveryTypes: Map<string, IrType>;
  scopeBindings: WeakMap<ts.Node, ReadonlySet<string>>;
  sourceFile: ts.SourceFile;
  typeScopeBindings: WeakMap<ts.Node, ReadonlySet<string>>;
  temporaryIndex: number;
  taskConstructions: IrTaskConstruction[];
  webGpuCanvasContextBindingNames: ReadonlySet<string>;
  webGpuDeviceBindingNames: ReadonlySet<string>;
  webGpuLimitsBindingNames: ReadonlySet<string>;
  webGpuQueueBindingNames: ReadonlySet<string>;
  webGlBindingNames: ReadonlySet<string>;
  workspaceDirectory: string;
}

class UnsupportedSyntaxError extends Error {}

function lowerClass(node: ts.ClassDeclaration, context: LoweringContext) {
  if (!node.name) throw new Error('Expected named class');
  const constructor = node.members.find(ts.isConstructorDeclaration);
  const fields = node.members.filter(ts.isPropertyDeclaration).map((field) => {
    return {
      initializer: field.initializer ? lowerExpression(field.initializer, context) : undefined,
      mutable: !hasModifier(field, ts.SyntaxKind.ReadonlyKeyword),
      name: propertyName(field.name, context),
      public: !hasModifier(field, ts.SyntaxKind.PrivateKeyword) && !hasModifier(field, ts.SyntaxKind.ProtectedKeyword),
      static: hasModifier(field, ts.SyntaxKind.StaticKeyword),
      type: field.type ? lowerType(field.type, context) : { kind: 'dynamic' as const },
    };
  });
  const heritage = node.heritageClauses?.find((clause) => clause.token === ts.SyntaxKind.ExtendsKeyword)?.types.at(0);
  const extendsType = heritage ? lowerExpressionWithTypeArguments(heritage, context) : undefined;
  if (extendsType?.kind === 'named' && extendsType.name === 'Error') {
    extendsType.name = 'PortError';
    fields.push({
      initializer: { kind: 'literal', value: 'Error' },
      mutable: true,
      name: 'name',
      public: true,
      static: false,
      type: { kind: 'primitive', name: 'String' },
    });
  }
  const loweredConstructor = constructor ? lowerParameterList(constructor.parameters, context) : undefined;
  return {
    constructorBody: [
      ...(loweredConstructor?.prefix ?? []),
      ...(constructor?.body?.statements.map((statement) => lowerStatement(statement, context)) ?? []),
    ],
    constructorParameters: loweredConstructor?.parameters ?? [],
    exported: hasModifier(node, ts.SyntaxKind.ExportKeyword),
    extends: extendsType,
    fields,
    kind: 'class' as const,
    methods: node.members.filter(ts.isMethodDeclaration).map((method) => {
      if (!method.body) unsupported(method, context, 'method without a body');
      const loweredParameters = lowerParameterList(method.parameters, context);
      return {
        body: [
          ...loweredParameters.prefix,
          ...method.body.statements.map((statement) => lowerStatement(statement, context)),
        ],
        execution: functionExecution(method, context),
        name: propertyName(method.name, context),
        parameters: loweredParameters.parameters,
        public:
          !hasModifier(method, ts.SyntaxKind.PrivateKeyword) && !hasModifier(method, ts.SyntaxKind.ProtectedKeyword),
        returns: hasModifier(method, ts.SyntaxKind.AsyncKeyword)
          ? asyncTaskType(method, context)
          : method.type
            ? lowerType(method.type, context)
            : hasReturnValue(method.body)
              ? ({ kind: 'dynamic' } satisfies IrType)
              : ({ kind: 'primitive', name: 'Void' } satisfies IrType),
        static: hasModifier(method, ts.SyntaxKind.StaticKeyword),
        typeParameters: method.typeParameters?.map((parameter) => parameter.name.text) ?? [],
      };
    }),
    name: node.name.text,
    origin: origin(node, context),
    typeParameters: node.typeParameters?.map((parameter) => parameter.name.text) ?? [],
  };
}

function mergeNamespace(node: ts.ModuleDeclaration, declarations: IrDeclaration[], context: LoweringContext): boolean {
  if (!ts.isIdentifier(node.name) || !node.body || !ts.isModuleBlock(node.body)) return false;
  const target = declarations.find(
    (declaration) => declaration.kind === 'enum' && declaration.name === node.name.getText(context.sourceFile),
  );
  if (!target || target.kind !== 'enum') return false;
  for (const statement of node.body.statements) {
    if (!ts.isFunctionDeclaration(statement) || !statement.name || !statement.body) return false;
    target.methods.push(lowerFunction(statement, context));
  }
  return true;
}

function lowerFunction(node: ts.FunctionDeclaration, context: LoweringContext): IrFunctionDeclaration {
  if (!node.name || !node.body) throw new Error('Expected named function with a body');
  const loweredParameters = lowerParameterList(node.parameters, context);
  const body = [
    ...loweredParameters.prefix,
    ...node.body.statements.map((statement) => lowerStatement(statement, context)),
  ];
  return {
    body,
    execution: functionExecution(node, context),
    exported: hasModifier(node, ts.SyntaxKind.ExportKeyword),
    kind: 'function',
    name: node.name.text,
    origin: origin(node, context),
    parameters: loweredParameters.parameters,
    returns: hasModifier(node, ts.SyntaxKind.AsyncKeyword)
      ? asyncTaskType(node, context)
      : node.type
        ? lowerType(node.type, context)
        : hasReturnValue(node.body)
          ? (inferNativeHostFunctionReturnType(body) ?? { kind: 'dynamic' })
          : { kind: 'primitive', name: 'Void' },
    typeParameters: node.typeParameters?.map((parameter) => parameter.name.text) ?? [],
  };
}

function lowerParameter(node: ts.ParameterDeclaration, context: LoweringContext): IrParameter {
  if (!ts.isIdentifier(node.name)) unsupported(node.name, context, 'binding pattern parameter');
  return {
    initializer: node.initializer ? lowerExpression(node.initializer, context) : undefined,
    name: node.name.text,
    optional: Boolean(node.questionToken),
    rest: Boolean(node.dotDotDotToken),
    type: node.type
      ? lowerType(node.type, context)
      : (contextualParameterType(node, context) ??
        inferParameterTypeFromInitializer(node.initializer) ?? { kind: 'dynamic' }),
  };
}

function contextualParameterType(node: ts.ParameterDeclaration, context: LoweringContext): IrType | undefined {
  const owner = node.parent;
  if (!ts.isArrowFunction(owner) && !ts.isFunctionExpression(owner) && !ts.isMethodDeclaration(owner)) {
    return undefined;
  }
  const contextual = contextualFunctionType(owner, context);
  const resolved = contextual ? resolveRecoveryType(contextual, context) : undefined;
  const index = owner.parameters.indexOf(node);
  return resolved?.kind === 'function' && index >= 0 ? resolved.parameters[index] : undefined;
}

function inferParameterTypeFromInitializer(node: ts.Expression | undefined): IrType | undefined {
  if (!node) return undefined;
  if (ts.isNumericLiteral(node) || ts.isPrefixUnaryExpression(node)) {
    return { kind: 'primitive', name: 'Float' };
  }
  if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) {
    return { kind: 'primitive', name: 'String' };
  }
  if (node.kind === ts.SyntaxKind.TrueKeyword || node.kind === ts.SyntaxKind.FalseKeyword) {
    return { kind: 'primitive', name: 'Bool' };
  }
  return undefined;
}

function lowerParameterList(
  nodes: readonly ts.ParameterDeclaration[],
  context: LoweringContext,
): { parameters: IrParameter[]; prefix: IrStatement[] } {
  const parameters: IrParameter[] = [];
  const prefix: IrStatement[] = [];
  for (const node of nodes) {
    if (isThisParameter(node)) continue;
    if (ts.isIdentifier(node.name)) {
      parameters.push(lowerParameter(node, context));
      continue;
    }
    const name = `__parameter${String(context.temporaryIndex++)}`;
    parameters.push({
      initializer: node.initializer ? lowerExpression(node.initializer, context) : undefined,
      name,
      optional: Boolean(node.questionToken),
      rest: Boolean(node.dotDotDotToken),
      type: node.type ? lowerType(node.type, context) : { kind: 'dynamic' },
    });
    const declarations: IrVariable[] = [];
    lowerBindingPattern(node.name, { kind: 'identifier', name }, false, declarations, context);
    prefix.push({ declarations, kind: 'variable' });
  }
  return { parameters, prefix };
}

function lowerType(node: ts.TypeNode, context: LoweringContext): IrType {
  switch (node.kind) {
    case ts.SyntaxKind.AnyKeyword:
    case ts.SyntaxKind.NeverKeyword:
      return { kind: 'dynamic' };
    case ts.SyntaxKind.UnknownKeyword:
      return { kind: 'dynamic', portable: true };
    case ts.SyntaxKind.UndefinedKeyword:
      return { kind: 'dynamic' };
    case ts.SyntaxKind.ObjectKeyword:
      return { kind: 'dynamic' };
    case ts.SyntaxKind.BooleanKeyword:
      return { kind: 'primitive', name: 'Bool' };
    case ts.SyntaxKind.NumberKeyword:
      return { kind: 'primitive', name: 'Float' };
    case ts.SyntaxKind.SymbolKeyword:
      return { arguments: [], kind: 'named', name: 'FlightSymbol' };
    case ts.SyntaxKind.StringKeyword:
      return { kind: 'primitive', name: 'String' };
    case ts.SyntaxKind.VoidKeyword:
      return { kind: 'primitive', name: 'Void' };
  }
  if (ts.isArrayTypeNode(node)) return { element: lowerType(node.elementType, context), kind: 'array' };
  if (ts.isTypeOperatorNode(node)) return lowerType(node.type, context);
  if (ts.isTypeQueryNode(node)) {
    const value = constObjectPropertyLiteralValue(node, context);
    if (typeof value === 'boolean') return { kind: 'primitive', name: 'Bool' };
    if (typeof value === 'number') return { kind: 'primitive', name: 'Float' };
    if (typeof value === 'string') return { kind: 'primitive', name: 'String' };
    return { kind: 'dynamic' };
  }
  // Template-literal types constrain which JavaScript strings are accepted statically, but they do
  // not introduce a distinct runtime representation. Preserve the value category without pretending
  // Rust's `String` enforces the TypeScript pattern (for example `${string}.${string}`).
  if (ts.isTemplateLiteralTypeNode(node)) return { kind: 'primitive', name: 'String' };
  if (ts.isIndexedAccessTypeNode(node)) {
    const parameterType = inferIndexedParameterType(node, context);
    if (parameterType) return parameterType;
    const namespaceType = inferValueNamespaceType(node, context);
    if (namespaceType) return namespaceType;
    const propertyType = inferIndexedPropertyType(node, context);
    if (propertyType) return propertyType;
    return { kind: 'dynamic' };
  }
  if (ts.isTypeLiteralNode(node)) {
    return {
      extends: [],
      fields: lowerTypeMembers(node.members, context),
      kind: 'anonymous',
    };
  }
  if (ts.isTupleTypeNode(node)) {
    const elements = node.elements.map((element) =>
      lowerType(ts.isNamedTupleMember(element) ? element.type : element, context),
    );
    return { element: commonType(elements), kind: 'array' };
  }
  if (ts.isParenthesizedTypeNode(node)) return lowerType(node.type, context);
  if (ts.isFunctionTypeNode(node)) {
    const parameters = lowerParameterList(node.parameters, context).parameters;
    return {
      kind: 'function',
      parameters: parameters.map(callbackParameterType),
      returns: lowerType(node.type, context),
    };
  }
  if (ts.isConstructorTypeNode(node)) return { kind: 'dynamic' };
  if (ts.isTypePredicateNode(node)) return { kind: 'primitive', name: 'Bool' };
  if (ts.isTypeReferenceNode(node)) {
    const name = node.typeName.getText(context.sourceFile);
    const arguments_ = node.typeArguments?.map((argument) => lowerType(argument, context)) ?? [];
    if (context.erasedLocalTypes.has(name)) return { kind: 'dynamic' };
    if (
      (name === 'Promise' && ts.isIdentifier(node.typeName) && !isTypeNameLexicallyBound(node.typeName, context)) ||
      name === 'globalThis.Promise'
    ) {
      return { kind: 'task', output: arguments_[0] ?? { kind: 'dynamic' } };
    }
    if (name === 'Error') return { arguments: [], kind: 'named', name: 'PortError' };
    const portableType = portableTypeReferenceMap[name];
    if (portableType) return { arguments: [], kind: 'named', name: portableType };
    if (name === 'Map' || name === 'ReadonlyMap' || name === 'WeakMap') {
      return { arguments: arguments_, kind: 'named', name: 'RustMap' };
    }
    if (name === 'Set' || name === 'ReadonlySet' || name === 'WeakSet') {
      return { arguments: arguments_, kind: 'named', name: 'RustSet' };
    }
    if (name === 'RegExpExecArray') {
      return {
        element: { inner: { kind: 'primitive', name: 'String' }, kind: 'nullable' },
        kind: 'array',
      };
    }
    if (context.externalTypes.has(name.split('.')[0]!)) return { kind: 'dynamic' };
    const nativeHostConstructor = portConfig.typeLowering.nativeHostConstructors.find(
      (constructor) => constructor.global === name,
    );
    if (nativeHostConstructor) {
      return { arguments: [], kind: 'named', name: nativeHostConstructor.resultType };
    }
    const platformType =
      platformDynamicTypes.has(name) ||
      name.startsWith('GPU') ||
      name.startsWith('HTML') ||
      name.startsWith('Intl.') ||
      name.startsWith('SVG') ||
      name.startsWith('WebGL') ||
      name.startsWith('globalThis.') ||
      name.startsWith('Canvas') ||
      name.startsWith('FileSystem') ||
      name.startsWith('Offscreen') ||
      name.startsWith('Performance') ||
      name.endsWith('Event') ||
      name.endsWith('EventListener') ||
      ['BodyInit', 'CSSStyleDeclaration', 'TextEncoder', 'WindowEventMap'].includes(name);
    if (platformType && (!ts.isIdentifier(node.typeName) || !isTypeNameLexicallyBound(node.typeName, context))) {
      return { kind: 'dynamic' };
    }
    const utilityArgument = arguments_[0];
    if (name === 'Partial' && utilityArgument && utilityArgument.kind !== 'dynamic') {
      return { arguments: [utilityArgument], kind: 'named', name: 'FlightPartial' };
    }
    if (name === 'Partial' && utilityArgument) return utilityArgument;
    if (name === 'Omit' && utilityArgument && node.typeArguments?.[1]) {
      const omitted = literalStringTypeValues(node.typeArguments[1]);
      if (omitted.length > 0) {
        return {
          arguments: [utilityArgument],
          kind: 'named',
          name: `FlightOmit:${JSON.stringify(omitted)}`,
        };
      }
    }
    if (['MethodsOf', 'Omit', 'PartialNode', 'Pick'].includes(name) && arguments_[0]) {
      return arguments_[0];
    }
    if (['Awaited', 'Exclude', 'Extract', 'NonNullable', 'Readonly', 'Required'].includes(name) && arguments_[0]) {
      return arguments_[0];
    }
    if (name === 'Parameters' && arguments_[0]) {
      return { arguments: [arguments_[0]], kind: 'named', name: 'FlightCallbackArgs' };
    }
    if (
      name === 'ReturnType' &&
      /\bset(?:Interval|Timeout)\b/u.test(node.typeArguments?.[0]?.getText(context.sourceFile) ?? '')
    ) {
      return { arguments: [], kind: 'named', name: 'FlightTimeout' };
    }
    if (['InstanceType', 'PropertyKey', 'ReturnType', 'ThisParameterType'].includes(name)) {
      return { kind: 'dynamic' };
    }
    if (name === 'ArrayLike') {
      return { element: arguments_[0] ?? { kind: 'dynamic' }, kind: 'array' };
    }
    if (name === 'Array' || name === 'ReadonlyArray') {
      return { element: arguments_[0] ?? { kind: 'dynamic' }, kind: 'array' };
    }
    if (portConfig.typeLowering.transparentTypeWrappers.some((wrapper) => wrapper.name === name) && arguments_[0]) {
      return arguments_[0];
    }
    if (name === 'Record') {
      return {
        arguments: [arguments_[0] ?? { kind: 'primitive', name: 'String' }, arguments_[1] ?? { kind: 'dynamic' }],
        kind: 'named',
        name: 'RustMap',
      };
    }
    return { arguments: arguments_, kind: 'named', name };
  }
  if (ts.isUnionTypeNode(node)) {
    const concrete = node.types.filter((item) => !isNullishType(item));
    const nullable = concrete.length !== node.types.length;
    const inner =
      concrete.length === 1
        ? lowerType(concrete[0]!, context)
        : commonType(concrete.map((item) => lowerType(item, context)));
    return nullable ? { inner, kind: 'nullable' } : inner;
  }
  if (ts.isIntersectionTypeNode(node)) {
    const types = node.types.map((item) => lowerType(item, context));
    const stringType = types.find((item) => item.kind === 'primitive' && item.name === 'String');
    if (stringType) return stringType;
    const genericIndex = node.types.findIndex((item) => isTypeParameterReference(item));
    const genericType = genericIndex < 0 ? undefined : types[genericIndex];
    const configuredBase = types.find(
      (item) =>
        item.kind === 'named' &&
        portConfig.typeLowering.genericIntersectionBaseOverrides.some((override) => override.name === item.name),
    );
    if (types.length === 2 && configuredBase && genericType) return configuredBase;
    if (genericType) return genericType;
    const concrete = types.filter((item) => item.kind !== 'dynamic');
    if (concrete.length === 0) return { kind: 'dynamic' };
    if (concrete.length === 1) return concrete[0]!;
    return {
      extends: concrete.flatMap((item) => (item.kind === 'anonymous' ? item.extends : [item])),
      fields: concrete.flatMap((item) => (item.kind === 'anonymous' ? item.fields : [])),
      kind: 'anonymous',
    };
  }
  if (ts.isConditionalTypeNode(node) || ts.isMappedTypeNode(node)) {
    return { kind: 'dynamic' };
  }
  if (ts.isLiteralTypeNode(node)) {
    if (node.literal.kind === ts.SyntaxKind.NullKeyword) return { kind: 'dynamic' };
    if (ts.isStringLiteral(node.literal)) return { kind: 'primitive', name: 'String' };
    if (ts.isNumericLiteral(node.literal)) return { kind: 'primitive', name: 'Float' };
    if (
      ts.isPrefixUnaryExpression(node.literal) &&
      node.literal.operator === ts.SyntaxKind.MinusToken &&
      ts.isNumericLiteral(node.literal.operand)
    ) {
      return { kind: 'primitive', name: 'Float' };
    }
    if (node.literal.kind === ts.SyntaxKind.TrueKeyword || node.literal.kind === ts.SyntaxKind.FalseKeyword) {
      return { kind: 'primitive', name: 'Bool' };
    }
  }
  return unsupported(node, context, `type ${ts.SyntaxKind[node.kind] ?? node.kind}`);
}

function inferIndexedParameterType(node: ts.IndexedAccessTypeNode, context: LoweringContext): IrType | undefined {
  let objectType = node.objectType;
  while (ts.isParenthesizedTypeNode(objectType)) objectType = objectType.type;
  let indexType = node.indexType;
  while (ts.isParenthesizedTypeNode(indexType)) indexType = indexType.type;
  if (
    !ts.isTypeReferenceNode(objectType) ||
    !ts.isIdentifier(objectType.typeName) ||
    objectType.typeName.text !== 'Parameters' ||
    !objectType.typeArguments?.[0] ||
    !ts.isLiteralTypeNode(indexType) ||
    !ts.isNumericLiteral(indexType.literal)
  ) {
    return undefined;
  }
  const index = Number(indexType.literal.text);
  if (!Number.isSafeInteger(index) || index < 0) return undefined;
  const callback = resolveCallbackTypeNode(objectType.typeArguments[0], context, new Set());
  return callback?.parameters[index];
}

function resolveCallbackTypeNode(
  node: ts.TypeNode,
  context: LoweringContext,
  visited: Set<string>,
): Extract<IrType, { kind: 'function' }> | undefined {
  while (ts.isParenthesizedTypeNode(node)) node = node.type;
  if (ts.isUnionTypeNode(node)) {
    for (const member of node.types.filter((candidate) => !isNullishType(candidate))) {
      const resolved = resolveCallbackTypeNode(member, context, visited);
      if (resolved) return resolved;
    }
    return undefined;
  }
  if (
    ts.isTypeReferenceNode(node) &&
    ts.isIdentifier(node.typeName) &&
    node.typeName.text === 'NonNullable' &&
    node.typeArguments?.[0]
  ) {
    return resolveCallbackTypeNode(node.typeArguments[0], context, visited);
  }
  if (ts.isTypeQueryNode(node) && ts.isIdentifier(node.exprName)) {
    const valueType = resolveValueTypeNode(node.exprName.text, context);
    return valueType ? resolveCallbackTypeNode(valueType.node, valueType.context, visited) : undefined;
  }
  if (ts.isTypeReferenceNode(node) && ts.isIdentifier(node.typeName) && !node.typeArguments?.length) {
    const resolved = resolveTypeDeclaration(node.typeName.text, context);
    if (!resolved || !ts.isTypeAliasDeclaration(resolved.declaration)) return undefined;
    const key = `${resolved.context.sourceFile.fileName}\0${node.typeName.text}`;
    if (visited.has(key)) return undefined;
    visited.add(key);
    return resolveCallbackTypeNode(resolved.declaration.type, resolved.context, visited);
  }
  const lowered = lowerType(node, context);
  return lowered.kind === 'function' ? lowered : undefined;
}

function resolveValueTypeNode(
  name: string,
  context: LoweringContext,
): { context: LoweringContext; node: ts.TypeNode } | undefined {
  const find = (sourceFile: ts.SourceFile): ts.TypeNode | undefined =>
    sourceFile.statements
      .filter(ts.isVariableStatement)
      .flatMap((statement) => [...statement.declarationList.declarations])
      .find((declaration) => ts.isIdentifier(declaration.name) && declaration.name.text === name)?.type;
  const local = find(context.sourceFile);
  if (local) return { context, node: local };
  const imported = resolveImportedTypeSource(name, context);
  if (!imported) return undefined;
  const sourceFile = ts.createSourceFile(
    imported.source,
    readFileSync(imported.source, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    imported.source.endsWith('.tsx') ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
  );
  const nextContext = { ...context, packageName: imported.packageName, sourceFile };
  const importedType = sourceFile.statements
    .filter(ts.isVariableStatement)
    .flatMap((statement) => [...statement.declarationList.declarations])
    .find((declaration) => ts.isIdentifier(declaration.name) && declaration.name.text === imported.imported)?.type;
  return importedType ? { context: nextContext, node: importedType } : undefined;
}

function isTypeParameterReference(node: ts.TypeNode): boolean {
  if (!ts.isTypeReferenceNode(node) || !ts.isIdentifier(node.typeName) || node.typeArguments?.length) return false;
  const name = node.typeName.text;
  for (let current: ts.Node | undefined = node.parent; current; current = current.parent) {
    const typeParameters =
      ts.isFunctionLike(current) ||
      ts.isClassDeclaration(current) ||
      ts.isInterfaceDeclaration(current) ||
      ts.isTypeAliasDeclaration(current)
        ? current.typeParameters
        : undefined;
    if (typeParameters?.some((parameter) => parameter.name.text === name)) return true;
  }
  return false;
}

function inferValueNamespaceType(node: ts.IndexedAccessTypeNode, context: LoweringContext): IrType | undefined {
  let objectType = node.objectType;
  while (ts.isParenthesizedTypeNode(objectType)) objectType = objectType.type;
  if (!ts.isTypeQueryNode(objectType) || !ts.isIdentifier(objectType.exprName)) return undefined;
  const namespace = objectType.exprName.text;
  const declaration = context.sourceFile.statements
    .filter(ts.isVariableStatement)
    .flatMap((statement) => [...statement.declarationList.declarations])
    .find((candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === namespace);
  if (!declaration?.initializer) return undefined;
  let initializer = declaration.initializer;
  while (
    ts.isParenthesizedExpression(initializer) ||
    ts.isAsExpression(initializer) ||
    ts.isTypeAssertionExpression(initializer) ||
    ts.isSatisfiesExpression(initializer)
  ) {
    initializer = initializer.expression;
  }
  if (!ts.isObjectLiteralExpression(initializer)) return undefined;
  const types = initializer.properties.flatMap((property): IrType[] => {
    if (!ts.isPropertyAssignment(property)) return [];
    const inferred = inferParameterTypeFromInitializer(property.initializer);
    return inferred ? [inferred] : [];
  });
  if (types.length !== initializer.properties.length) return undefined;
  return commonType(types);
}

function inferIndexedPropertyType(node: ts.IndexedAccessTypeNode, context: LoweringContext): IrType | undefined {
  let objectType = node.objectType;
  while (ts.isParenthesizedTypeNode(objectType)) objectType = objectType.type;
  let indexType = node.indexType;
  while (ts.isParenthesizedTypeNode(indexType)) indexType = indexType.type;
  if (
    !ts.isTypeReferenceNode(objectType) ||
    !ts.isIdentifier(objectType.typeName) ||
    !ts.isLiteralTypeNode(indexType) ||
    !ts.isStringLiteral(indexType.literal)
  ) {
    return undefined;
  }
  return resolveIndexedPropertyType(objectType.typeName.text, indexType.literal.text, context, new Set<string>());
}

function resolveIndexedPropertyType(
  typeName: string,
  property: string,
  context: LoweringContext,
  visited: Set<string>,
): IrType | undefined {
  const resolved = resolveTypeDeclaration(typeName, context);
  if (!resolved) return undefined;
  const key = `${resolved.context.sourceFile.fileName}\0${typeName}\0${property}`;
  if (visited.has(key)) return undefined;
  visited.add(key);
  const declaration = resolved.declaration;
  const members =
    ts.isInterfaceDeclaration(declaration) || ts.isClassDeclaration(declaration)
      ? declaration.members
      : ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type)
        ? declaration.type.members
        : undefined;
  if (!members) return undefined;
  const member = members.find(
    (candidate) =>
      (ts.isPropertySignature(candidate) || ts.isPropertyDeclaration(candidate)) &&
      candidate.type &&
      propertyName(candidate.name, resolved.context) === property,
  );
  if (!member || (!ts.isPropertySignature(member) && !ts.isPropertyDeclaration(member)) || !member.type) {
    return undefined;
  }
  return lowerResolvedPropertyType(member.type, resolved.context, visited);
}

function lowerResolvedPropertyType(
  node: ts.TypeNode,
  context: LoweringContext,
  visited: Set<string>,
): IrType | undefined {
  if (ts.isTypeReferenceNode(node) && ts.isIdentifier(node.typeName) && !node.typeArguments?.length) {
    const resolved = resolveTypeDeclaration(node.typeName.text, context);
    if (!resolved || !ts.isTypeAliasDeclaration(resolved.declaration)) return undefined;
    const key = `${resolved.context.sourceFile.fileName}\0${node.typeName.text}`;
    if (visited.has(key)) return undefined;
    visited.add(key);
    return lowerResolvedPropertyType(resolved.declaration.type, resolved.context, visited);
  }
  return lowerType(node, context);
}

function resolveTypeDeclaration(
  name: string,
  context: LoweringContext,
):
  | {
      context: LoweringContext;
      declaration: ts.ClassDeclaration | ts.InterfaceDeclaration | ts.TypeAliasDeclaration;
    }
  | undefined {
  const local = context.sourceFile.statements.find(
    (statement): statement is ts.ClassDeclaration | ts.InterfaceDeclaration | ts.TypeAliasDeclaration =>
      (ts.isClassDeclaration(statement) ||
        ts.isInterfaceDeclaration(statement) ||
        ts.isTypeAliasDeclaration(statement)) &&
      statement.name?.text === name,
  );
  if (local) return { context, declaration: local };
  const imported = resolveImportedTypeSource(name, context);
  if (!imported) return undefined;
  const sourceFile = ts.createSourceFile(
    imported.source,
    readFileSync(imported.source, 'utf8'),
    ts.ScriptTarget.Latest,
    true,
    imported.source.endsWith('.tsx') ? ts.ScriptKind.TSX : ts.ScriptKind.TS,
  );
  const nextContext = { ...context, packageName: imported.packageName, sourceFile };
  const declaration = sourceFile.statements.find(
    (statement): statement is ts.ClassDeclaration | ts.InterfaceDeclaration | ts.TypeAliasDeclaration =>
      (ts.isClassDeclaration(statement) ||
        ts.isInterfaceDeclaration(statement) ||
        ts.isTypeAliasDeclaration(statement)) &&
      statement.name?.text === imported.imported,
  );
  return declaration ? { context: nextContext, declaration } : undefined;
}

function resolveImportedTypeSource(
  localName: string,
  context: LoweringContext,
): { imported: string; packageName: string; source: string } | undefined {
  for (const statement of context.sourceFile.statements) {
    if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier)) continue;
    const bindings = statement.importClause?.namedBindings;
    if (!bindings || !ts.isNamedImports(bindings)) continue;
    const binding = bindings.elements.find((element) => element.name.text === localName);
    if (!binding) continue;
    const imported = binding.propertyName?.text ?? binding.name.text;
    const specifier = statement.moduleSpecifier.text;
    const flightPackage = /^(@flighthq\/[^/]+)/u.exec(specifier)?.[1];
    const base = specifier.startsWith('@flighthq/')
      ? path.join(
          context.workspaceDirectory,
          'upstream',
          'packages',
          flightPackage!.slice('@flighthq/'.length),
          'src',
          imported,
        )
      : specifier.startsWith('.')
        ? path.resolve(path.dirname(context.sourceFile.fileName), specifier)
        : undefined;
    if (!base) return undefined;
    const source = [`${base}.ts`, `${base}.tsx`, path.join(base, 'index.ts'), path.join(base, 'index.tsx')].find(
      (candidate) => existsSync(candidate),
    );
    if (!source) return undefined;
    return {
      imported,
      packageName: flightPackage ?? context.packageName,
      source,
    };
  }
  return undefined;
}

function isNullishType(node: ts.TypeNode): boolean {
  return (
    node.kind === ts.SyntaxKind.UndefinedKeyword ||
    node.kind === ts.SyntaxKind.NullKeyword ||
    (ts.isLiteralTypeNode(node) && node.literal.kind === ts.SyntaxKind.NullKeyword)
  );
}

function lowerTypeMembers(members: ts.NodeArray<ts.TypeElement>, context: LoweringContext) {
  const lowered = members.flatMap((member) => {
    try {
      const field = lowerTypeMember(member, context);
      return field ? [field] : [];
    } catch (error) {
      if (error instanceof UnsupportedSyntaxError) return [];
      throw error;
    }
  });
  return [...new Map(lowered.map((field) => [field.name, field])).values()];
}

function lowerExpressionWithTypeArguments(node: ts.ExpressionWithTypeArguments, context: LoweringContext): IrType {
  const name = node.expression.getText(context.sourceFile);
  const arguments_ = node.typeArguments?.map((argument) => lowerType(argument, context)) ?? [];
  if (platformDynamicTypes.has(name) || context.externalTypes.has(name.split('.')[0]!)) return { kind: 'dynamic' };
  if (name === 'Omit' || name === 'Partial' || name === 'Pick') return { kind: 'dynamic' };
  if (name === 'Readonly' && arguments_[0]) {
    return arguments_[0];
  }
  return {
    arguments: arguments_,
    kind: 'named',
    name,
  };
}

function lowerTypeMember(node: ts.TypeElement, context: LoweringContext) {
  if (ts.isPropertySignature(node) && node.type) {
    if (ts.isComputedPropertyName(node.name)) return undefined;
    return {
      contextualParameters: ts.isFunctionTypeNode(node.type)
        ? lowerParameterList(node.type.parameters, context).parameters
        : undefined,
      discriminantValue: literalTypeValue(node.type, context),
      name: propertyName(node.name, context),
      optional: Boolean(node.questionToken),
      type: lowerType(node.type, context),
    };
  }
  if (ts.isMethodSignature(node)) {
    const parameters = lowerParameterList(node.parameters, context).parameters;
    return {
      contextualParameters: parameters,
      name: propertyName(node.name, context),
      optional: Boolean(node.questionToken),
      type: {
        kind: 'function' as const,
        parameters: parameters.map(callbackParameterType),
        returns: node.type ? lowerType(node.type, context) : { kind: 'primitive' as const, name: 'Void' as const },
      },
    };
  }
  if (ts.isIndexSignatureDeclaration(node)) return undefined;
  if (ts.isConstructSignatureDeclaration(node)) {
    return {
      name: '__construct',
      optional: true,
      type: { kind: 'dynamic' as const },
    };
  }
  return unsupported(node, context, `type member ${ts.SyntaxKind[node.kind] ?? node.kind}`);
}

function literalTypeValue(node: ts.TypeNode, context: LoweringContext): boolean | number | string | undefined {
  const constant = constObjectPropertyLiteralValue(node, context);
  if (constant !== undefined) return constant;
  if (!ts.isLiteralTypeNode(node)) return undefined;
  if (ts.isStringLiteral(node.literal) || ts.isNumericLiteral(node.literal)) {
    return ts.isStringLiteral(node.literal) ? node.literal.text : Number(node.literal.text);
  }
  if (node.literal.kind === ts.SyntaxKind.TrueKeyword) return true;
  if (node.literal.kind === ts.SyntaxKind.FalseKeyword) return false;
  if (
    ts.isPrefixUnaryExpression(node.literal) &&
    node.literal.operator === ts.SyntaxKind.MinusToken &&
    ts.isNumericLiteral(node.literal.operand)
  ) {
    return -Number(node.literal.operand.text);
  }
  return undefined;
}

function constObjectPropertyLiteralValue(
  node: ts.TypeNode,
  context: LoweringContext,
): boolean | number | string | undefined {
  if (
    !ts.isTypeQueryNode(node) ||
    !ts.isQualifiedName(node.exprName) ||
    !ts.isIdentifier(node.exprName.left)
  ) {
    return undefined;
  }
  const namespace = node.exprName.left.text;
  const member = node.exprName.right.text;
  const declaration = context.sourceFile.statements
    .filter(ts.isVariableStatement)
    .flatMap((statement) => [...statement.declarationList.declarations])
    .find((candidate) => ts.isIdentifier(candidate.name) && candidate.name.text === namespace);
  if (!declaration?.initializer) return undefined;
  let initializer = declaration.initializer;
  while (
    ts.isParenthesizedExpression(initializer) ||
    ts.isAsExpression(initializer) ||
    ts.isTypeAssertionExpression(initializer) ||
    ts.isSatisfiesExpression(initializer)
  ) {
    initializer = initializer.expression;
  }
  if (!ts.isObjectLiteralExpression(initializer)) return undefined;
  const property = initializer.properties.find(
    (candidate): candidate is ts.PropertyAssignment =>
      ts.isPropertyAssignment(candidate) && propertyName(candidate.name, context) === member,
  );
  if (!property) return undefined;
  const value = property.initializer;
  if (ts.isStringLiteral(value) || ts.isNoSubstitutionTemplateLiteral(value)) return value.text;
  if (ts.isNumericLiteral(value)) return Number(value.text);
  if (value.kind === ts.SyntaxKind.TrueKeyword) return true;
  if (value.kind === ts.SyntaxKind.FalseKeyword) return false;
  if (
    ts.isPrefixUnaryExpression(value) &&
    value.operator === ts.SyntaxKind.MinusToken &&
    ts.isNumericLiteral(value.operand)
  ) {
    return -Number(value.operand.text);
  }
  return undefined;
}

function literalStringTypeValues(node: ts.TypeNode): string[] {
  if (ts.isParenthesizedTypeNode(node)) return literalStringTypeValues(node.type);
  if (ts.isUnionTypeNode(node)) return node.types.flatMap(literalStringTypeValues);
  return ts.isLiteralTypeNode(node) && ts.isStringLiteral(node.literal) ? [node.literal.text] : [];
}

function commonType(types: IrType[]): IrType {
  const first = types[0];
  if (!first) return { kind: 'dynamic' };
  if (types.every((item) => JSON.stringify(item) === JSON.stringify(first))) return first;
  if (types.every((item) => item.kind === 'anonymous')) {
    const anonymous = types as Array<Extract<IrType, { kind: 'anonymous' }>>;
    const discriminated = anonymous[0]!.fields.some((field) => {
      if (field.discriminantValue === undefined) return false;
      const values = anonymous.map((item) =>
        item.fields.find((candidate) => candidate.name === field.name)?.discriminantValue,
      );
      return values.every((value) => value !== undefined) && new Set(values).size === anonymous.length;
    });
    if (discriminated) return { kind: 'union', variants: anonymous };
    const fieldNames = new Set(anonymous.flatMap((item) => item.fields.map((field) => field.name)));
    return {
      extends: [],
      fields: [...fieldNames].map((name) => {
        const fields = anonymous.flatMap((item) => {
          const field = item.fields.find((candidate) => candidate.name === name);
          return field ? [field] : [];
        });
        return {
          name,
          optional: fields.length !== anonymous.length || fields.some((field) => field.optional),
          type: commonType(fields.map((field) => field.type)),
        };
      }),
      kind: 'anonymous',
    };
  }
  if (first.kind === 'named') {
    const storage = portableTypedArrayStorage[first.name];
    if (storage && types.every((item) => item.kind === 'named' && portableTypedArrayStorage[item.name] === storage)) {
      return first;
    }
    const typedArrayFamily = first.name.startsWith('Uint')
      ? 'Uint'
      : first.name.startsWith('Int')
        ? 'Int'
        : first.name.startsWith('Float')
          ? 'Float'
          : undefined;
    if (
      typedArrayFamily &&
      types.every(
        (item) =>
          item.kind === 'named' &&
          item.name.startsWith(typedArrayFamily) &&
          portableTypedArrayRank[item.name] !== undefined,
      )
    ) {
      return types.reduce((widest, item) =>
        item.kind === 'named' &&
        portableTypedArrayRank[item.name]! >
          portableTypedArrayRank[(widest as Extract<IrType, { kind: 'named' }>).name]!
          ? item
          : widest,
      ) as IrType;
    }
  }
  return {
    kind: 'union',
    variants: [
      ...new Map(
        types
          .flatMap((item) => (item.kind === 'union' ? item.variants : [item]))
          .map((item) => [JSON.stringify(item), item]),
      ).values(),
    ],
  };
}

function hasReturnValue(body: ts.Block): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (found) return;
    if (ts.isReturnStatement(node) && node.expression) {
      found = true;
      return;
    }
    if (node !== body && ts.isFunctionLike(node)) return;
    ts.forEachChild(node, visit);
  };
  visit(body);
  return found;
}

function inferNativeHostFunctionReturnType(statements: readonly IrStatement[]): IrType | undefined {
  const expressions = statements.flatMap(collectFunctionReturnExpressions);
  if (expressions.length === 0 || expressions.some((expression) => expression === undefined)) return undefined;
  const types = expressions.map((expression) => (expression ? inferNativeHostExpressionType(expression) : undefined));
  const first = types[0];
  return first && types.every((type) => type && JSON.stringify(type) === JSON.stringify(first)) ? first : undefined;
}

function collectFunctionReturnExpressions(statement: IrStatement): Array<IrExpression | undefined> {
  switch (statement.kind) {
    case 'return':
      return [statement.expression];
    case 'block':
      return statement.statements.flatMap(collectFunctionReturnExpressions);
    case 'do':
    case 'while':
      return collectFunctionReturnExpressions(statement.body);
    case 'for':
    case 'forOf':
      return collectFunctionReturnExpressions(statement.body);
    case 'if':
      return [
        ...collectFunctionReturnExpressions(statement.consequent),
        ...(statement.otherwise ? collectFunctionReturnExpressions(statement.otherwise) : []),
      ];
    case 'switch':
      return statement.cases.flatMap((switchCase) => switchCase.statements.flatMap(collectFunctionReturnExpressions));
    case 'try':
      return [
        ...collectFunctionReturnExpressions(statement.tryBody),
        ...(statement.catchBody ? collectFunctionReturnExpressions(statement.catchBody) : []),
        ...(statement.finallyBody ? collectFunctionReturnExpressions(statement.finallyBody) : []),
      ];
    default:
      return [];
  }
}

function inferNativeHostExpressionType(expression: IrExpression): IrType | undefined {
  if (expression.kind === 'hostConstruct') {
    return { arguments: [], kind: 'named', name: expression.resultType };
  }
  if (expression.kind === 'cast') return inferNativeHostExpressionType(expression.expression);
  if (expression.kind !== 'conditional') return undefined;
  const whenTrue = inferNativeHostExpressionType(expression.whenTrue);
  const whenFalse = inferNativeHostExpressionType(expression.whenFalse);
  return whenTrue && whenFalse && JSON.stringify(whenTrue) === JSON.stringify(whenFalse) ? whenTrue : undefined;
}

function lowerStatement(node: ts.Statement, context: LoweringContext): IrStatement {
  if (ts.isBlock(node))
    return {
      kind: 'block',
      statements: node.statements.map((item) => lowerStatement(item, context)),
    };
  if (ts.isVariableStatement(node)) {
    const mutable = (node.declarationList.flags & ts.NodeFlags.Const) === 0;
    return {
      kind: 'variable',
      declarations: lowerVariables(node.declarationList, mutable, context),
    };
  }
  if (ts.isExpressionStatement(node))
    return {
      expression: lowerExpression(node.expression, context),
      kind: 'expression',
    };
  if (ts.isReturnStatement(node)) {
    return {
      expression: node.expression ? lowerExpression(node.expression, context) : undefined,
      kind: 'return',
    };
  }
  if (ts.isIfStatement(node)) {
    return {
      condition: lowerExpression(node.expression, context),
      consequent: lowerStatement(node.thenStatement, context),
      kind: 'if',
      otherwise: node.elseStatement ? lowerStatement(node.elseStatement, context) : undefined,
    };
  }
  if (ts.isWhileStatement(node)) {
    return {
      body: lowerStatement(node.statement, context),
      condition: lowerExpression(node.expression, context),
      kind: 'while',
    };
  }
  if (ts.isDoStatement(node)) {
    return {
      body: lowerStatement(node.statement, context),
      condition: lowerExpression(node.expression, context),
      kind: 'do',
    };
  }
  if (ts.isForStatement(node)) {
    let initializer: IrExpression | IrVariable[] | undefined;
    if (node.initializer) {
      initializer = ts.isVariableDeclarationList(node.initializer)
        ? lowerVariables(node.initializer, (node.initializer.flags & ts.NodeFlags.Const) === 0, context)
        : lowerExpression(node.initializer, context);
    }
    return {
      body: lowerStatement(node.statement, context),
      condition: node.condition ? lowerExpression(node.condition, context) : undefined,
      increment: node.incrementor ? lowerExpression(node.incrementor, context) : undefined,
      initializer,
      kind: 'for',
    };
  }
  if (ts.isForOfStatement(node)) {
    if (!ts.isVariableDeclarationList(node.initializer) || node.initializer.declarations.length !== 1) {
      return unsupported(node.initializer, context, 'for-of initializer');
    }
    const declaration = node.initializer.declarations[0]!;
    const mutable = (node.initializer.flags & ts.NodeFlags.Const) === 0;
    const bindings: IrVariable[] = [];
    const variable = ts.isIdentifier(declaration.name)
      ? declaration.name.text
      : `__iteration${String(context.temporaryIndex++)}`;
    if (!ts.isIdentifier(declaration.name)) {
      lowerBindingPattern(declaration.name, { kind: 'identifier', name: variable }, mutable, bindings, context);
    }
    return {
      async: Boolean(node.awaitModifier),
      bindings,
      body: lowerStatement(node.statement, context),
      iterable: lowerExpression(node.expression, context),
      kind: 'forOf',
      variable,
    };
  }
  if (ts.isForInStatement(node)) {
    if (!ts.isVariableDeclarationList(node.initializer) || node.initializer.declarations.length !== 1) {
      return unsupported(node.initializer, context, 'for-in initializer');
    }
    const declaration = node.initializer.declarations[0]!;
    if (!ts.isIdentifier(declaration.name)) {
      return unsupported(declaration.name, context, 'for-in initializer');
    }
    return {
      body: lowerStatement(node.statement, context),
      enumeration: isStringIndexedRecordExpression(node.expression, context) ? 'direct-record' : 'runtime',
      kind: 'forIn',
      object: lowerExpression(node.expression, context),
      variable: declaration.name.text,
    };
  }
  if (ts.isTypeAliasDeclaration(node)) return { kind: 'block', statements: [] };
  if (ts.isThrowStatement(node))
    return {
      expression: lowerExpression(node.expression, context),
      kind: 'throw',
    };
  if (ts.isSwitchStatement(node)) {
    return {
      cases: node.caseBlock.clauses.map((clause) => ({
        expression: ts.isCaseClause(clause) ? lowerExpression(clause.expression, context) : undefined,
        statements: clause.statements.map((statement) => lowerStatement(statement, context)),
      })),
      expression: lowerExpression(node.expression, context),
      kind: 'switch',
    };
  }
  if (ts.isBreakStatement(node)) return { kind: 'break' };
  if (ts.isContinueStatement(node)) return { kind: 'continue' };
  if (ts.isTryStatement(node)) {
    const catchName = node.catchClause?.variableDeclaration?.name;
    if (catchName && !ts.isIdentifier(catchName)) unsupported(catchName, context, 'catch binding pattern');
    const owner = findEnclosingFunction(node);
    return {
      catchBody: node.catchClause ? lowerStatement(node.catchClause.block, context) : undefined,
      catchName: catchName?.text,
      execution: owner && functionExecution(owner, context).kind === 'portableTask' ? 'portableTask' : 'sync',
      finallyBody: node.finallyBlock ? lowerStatement(node.finallyBlock, context) : undefined,
      kind: 'try',
      origin: origin(node, context),
      tryBody: lowerStatement(node.tryBlock, context),
    };
  }
  if (ts.isFunctionDeclaration(node) && node.name && node.body) {
    const loweredParameters = lowerParameterList(node.parameters, context);
    const parameters = loweredParameters.parameters;
    return {
      declarations: [
        {
          initializer: {
            body: [
              ...loweredParameters.prefix,
              ...node.body.statements.map((statement) => lowerStatement(statement, context)),
            ],
            execution: functionExecution(node, context),
            kind: 'function',
            name: node.name.text,
            parameters,
            returns: hasModifier(node, ts.SyntaxKind.AsyncKeyword)
              ? asyncTaskType(node, context)
              : node.type
                ? lowerType(node.type, context)
                : hasReturnValue(node.body)
                  ? { kind: 'dynamic' }
                  : { kind: 'primitive', name: 'Void' },
          },
          mutable: false,
          name: node.name.text,
          type: {
            kind: 'function',
            parameters: parameters.map(callbackParameterType),
            returns: hasModifier(node, ts.SyntaxKind.AsyncKeyword)
              ? asyncTaskType(node, context)
              : node.type
                ? lowerType(node.type, context)
                : { kind: 'dynamic' },
          },
        },
      ],
      kind: 'variable',
    };
  }
  if (ts.isEmptyStatement(node)) return { kind: 'block', statements: [] };
  return unsupported(node, context, `statement ${ts.SyntaxKind[node.kind] ?? node.kind}`);
}

function isStringIndexedRecordExpression(node: ts.Expression, context: LoweringContext): boolean {
  if (!ts.isIdentifier(node)) return false;
  const parameter = findEnclosingParameter(node);
  if (parameter?.type && isStringIndexedRecordType(parameter.type, context, new Set())) return true;
  let current: ts.Node | undefined = node;
  while (current) {
    for (const child of current.getChildren(context.sourceFile)) {
      if (
        ts.isVariableDeclaration(child) &&
        ts.isIdentifier(child.name) &&
        child.name.text === node.text &&
        child.type &&
        isStringIndexedRecordType(child.type, context, new Set())
      ) {
        return true;
      }
    }
    current = current.parent;
  }
  return false;
}

function isStringIndexedRecordType(node: ts.TypeNode, context: LoweringContext, visited: Set<string>): boolean {
  if (ts.isParenthesizedTypeNode(node)) return isStringIndexedRecordType(node.type, context, visited);
  if (ts.isUnionTypeNode(node) || ts.isIntersectionTypeNode(node)) {
    return node.types.length > 0 && node.types.every((item) => isStringIndexedRecordType(item, context, visited));
  }
  if (ts.isTypeLiteralNode(node)) {
    return node.members.some(
      (member) =>
        ts.isIndexSignatureDeclaration(member) && member.parameters[0]?.type?.kind === ts.SyntaxKind.StringKeyword,
    );
  }
  if (!ts.isTypeReferenceNode(node) || !ts.isIdentifier(node.typeName)) return false;
  const name = node.typeName.text;
  if (name === 'Readonly' && node.typeArguments?.[0]) {
    return isStringIndexedRecordType(node.typeArguments[0], context, visited);
  }
  if (name === 'Record') {
    const key = node.typeArguments?.[0];
    return key?.kind === ts.SyntaxKind.StringKeyword;
  }
  return isStringIndexedRecordDeclaration(name, context, visited);
}

function isStringIndexedRecordDeclaration(name: string, context: LoweringContext, visited: Set<string>): boolean {
  if (visited.has(name)) return false;
  const declaration = context.sourceFile.statements.find(
    (statement): statement is ts.InterfaceDeclaration | ts.TypeAliasDeclaration =>
      (ts.isInterfaceDeclaration(statement) || ts.isTypeAliasDeclaration(statement)) && statement.name.text === name,
  );
  if (!declaration) return false;
  const nextVisited = new Set([...visited, name]);
  if (ts.isTypeAliasDeclaration(declaration)) {
    return isStringIndexedRecordType(declaration.type, context, nextVisited);
  }
  return (
    declaration.members.some(
      (member) =>
        ts.isIndexSignatureDeclaration(member) && member.parameters[0]?.type?.kind === ts.SyntaxKind.StringKeyword,
    ) ||
    declaration.heritageClauses?.some((clause) =>
      clause.types.some(
        (type) =>
          ts.isIdentifier(type.expression) &&
          isStringIndexedRecordDeclaration(type.expression.text, context, nextVisited),
      ),
    ) === true
  );
}

function callbackParameterType(parameter: IrParameter): IrType {
  return parameter.optional && parameter.type.kind !== 'nullable'
    ? { inner: parameter.type, kind: 'nullable' }
    : parameter.type;
}

function lowerVariables(node: ts.VariableDeclarationList, mutable: boolean, context: LoweringContext): IrVariable[] {
  return node.declarations.flatMap((declaration) => {
    if (ts.isIdentifier(declaration.name)) {
      return {
        initializer: declaration.initializer ? lowerExpression(declaration.initializer, context) : undefined,
        mutable,
        name: declaration.name.text,
        type: declaration.type ? lowerType(declaration.type, context) : undefined,
      };
    }
    if (!declaration.initializer) unsupported(declaration.name, context, 'uninitialized binding pattern variable');
    const temporaryName = `__destructure${String(context.temporaryIndex++)}`;
    const variables: IrVariable[] = [
      {
        initializer: lowerExpression(declaration.initializer, context),
        mutable: false,
        name: temporaryName,
      },
    ];
    lowerBindingPattern(declaration.name, { kind: 'identifier', name: temporaryName }, mutable, variables, context);
    return variables;
  });
}

function lowerBindingPattern(
  pattern: ts.BindingPattern,
  source: IrExpression,
  mutable: boolean,
  variables: IrVariable[],
  context: LoweringContext,
): void {
  if (ts.isObjectBindingPattern(pattern)) {
    for (const element of pattern.elements) {
      if (element.dotDotDotToken) unsupported(element, context, 'object rest binding');
      const name = element.propertyName
        ? propertyName(element.propertyName, context)
        : element.name.getText(context.sourceFile);
      let value: IrExpression = { kind: 'property', name, object: source };
      if (element.initializer) {
        value = {
          kind: 'binary',
          left: value,
          operator: '??undefined',
          right: lowerExpression(element.initializer, context),
        };
      }
      if (ts.isIdentifier(element.name)) {
        variables.push({
          initializer: value,
          mutable,
          name: element.name.text,
        });
      } else {
        lowerBindingPattern(element.name, value, mutable, variables, context);
      }
    }
    return;
  }
  pattern.elements.forEach((element, index) => {
    if (ts.isOmittedExpression(element)) return;
    if (element.dotDotDotToken) unsupported(element, context, 'array rest binding');
    let value: IrExpression = {
      index: { kind: 'literal', value: index },
      kind: 'element',
      object: source,
    };
    if (element.initializer) {
      value = {
        kind: 'binary',
        left: value,
        operator: '??undefined',
        right: lowerExpression(element.initializer, context),
      };
    }
    if (ts.isIdentifier(element.name)) {
      variables.push({ initializer: value, mutable, name: element.name.text });
    } else {
      lowerBindingPattern(element.name, value, mutable, variables, context);
    }
  });
}

function webGlComputedConstantDomain(node: ts.Expression): 'GlBlendEquation' | 'GlBlendFactor' | undefined {
  if (
    ts.isParenthesizedExpression(node) ||
    ts.isAsExpression(node) ||
    ts.isTypeAssertionExpression(node) ||
    ts.isNonNullExpression(node) ||
    ts.isSatisfiesExpression(node)
  ) {
    return webGlComputedConstantDomain(node.expression);
  }
  if (ts.isPropertyAccessExpression(node)) {
    if (node.name.text === 'equation') return 'GlBlendEquation';
    if (node.name.text === 'src' || node.name.text === 'dst') return 'GlBlendFactor';
    return undefined;
  }
  if (
    ts.isBinaryExpression(node) &&
    node.operatorToken.kind === ts.SyntaxKind.QuestionQuestionToken &&
    ts.isStringLiteral(node.right) &&
    node.right.text === 'FUNC_ADD'
  ) {
    return webGlComputedConstantDomain(node.left) === 'GlBlendEquation' ? 'GlBlendEquation' : undefined;
  }
  return undefined;
}

function lowerExpression(node: ts.Expression, context: LoweringContext): IrExpression {
  if (ts.isParenthesizedExpression(node)) return lowerExpression(node.expression, context);
  if (ts.isAsExpression(node) || ts.isTypeAssertionExpression(node)) {
    if (
      node.type.kind === ts.SyntaxKind.ConstKeyword ||
      (ts.isTypeReferenceNode(node.type) && node.type.typeName.getText(context.sourceFile) === 'const')
    ) {
      return lowerExpression(node.expression, context);
    }
    return {
      expression: lowerExpression(node.expression, context),
      kind: 'cast',
      type: lowerType(node.type, context),
    };
  }
  if (ts.isNonNullExpression(node)) {
    return lowerExpression(node.expression, context);
  }
  if (ts.isSatisfiesExpression(node)) return lowerExpression(node.expression, context);
  if (ts.isAwaitExpression(node))
    return {
      expression: lowerExpression(node.expression, context),
      kind: 'await',
      origin: taskExpressionOrigin(node, context, 'await'),
    };
  if (ts.isVoidExpression(node)) {
    return {
      kind: 'unary',
      operand: lowerExpression(node.expression, context),
      operator: 'void',
      postfix: false,
    };
  }
  if (ts.isRegularExpressionLiteral(node)) {
    const match = /^\/(.*)\/([a-z]*)$/su.exec(node.text);
    return {
      flags: match?.[2] ?? '',
      kind: 'regexp',
      pattern: match?.[1] ?? node.text,
    };
  }
  if (node.kind === ts.SyntaxKind.ImportKeyword) return { kind: 'identifier', name: '_Runtime.dynamicImport' };
  if (node.kind === ts.SyntaxKind.ThisKeyword) {
    return context.classThis
      ? { kind: 'identifier', name: 'this' }
      : {
          arguments: [],
          callee: {
            kind: 'property',
            name: 'thisValue',
            object: { kind: 'identifier', name: '_Runtime' },
          },
          kind: 'call',
          typeArguments: [],
        };
  }
  if (node.kind === ts.SyntaxKind.SuperKeyword) return { kind: 'identifier', name: 'super' };
  if (ts.isIdentifier(node)) return lowerIdentifier(node.text, context, isLexicallyBound(node, context));
  if (ts.isNumericLiteral(node)) return { kind: 'literal', value: Number(node.text) };
  if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node))
    return { kind: 'literal', value: node.text };
  if (ts.isTemplateExpression(node)) {
    return {
      kind: 'template',
      parts: [
        node.head.text,
        ...node.templateSpans.flatMap((span) => [lowerExpression(span.expression, context), span.literal.text]),
      ],
    };
  }
  if (node.kind === ts.SyntaxKind.TrueKeyword) return { kind: 'literal', value: true };
  if (node.kind === ts.SyntaxKind.FalseKeyword) return { kind: 'literal', value: false };
  if (node.kind === ts.SyntaxKind.NullKeyword) return { kind: 'literal', value: null };
  if (ts.isArrayLiteralExpression(node)) {
    return {
      elements: node.elements.map((element) => lowerExpression(element, context)),
      kind: 'array',
    };
  }
  if (ts.isObjectLiteralExpression(node)) {
    return {
      kind: 'object',
      properties: node.properties.map((property) => {
        if (ts.isSpreadAssignment(property)) {
          return {
            expression: lowerExpression(property.expression, context),
            kind: 'spread' as const,
          };
        }
        if (ts.isShorthandPropertyAssignment(property)) {
          return {
            kind: 'property' as const,
            name: property.name.text,
            value: lowerIdentifier(property.name.text, context, isLexicallyBound(property.name, context)),
          };
        }
        if (ts.isPropertyAssignment(property)) {
          const value = lowerExpression(property.initializer, context);
          if (ts.isComputedPropertyName(property.name)) {
            return {
              key: lowerExpression(property.name.expression, context),
              kind: 'computedProperty' as const,
              value,
            };
          }
          return {
            kind: 'property' as const,
            name: propertyName(property.name, context),
            value,
          };
        }
        if (ts.isMethodDeclaration(property) && property.body) {
          const previousClassThis = context.classThis;
          context.classThis = false;
          try {
            const value = {
              body: property.body.statements.map((statement) => lowerStatement(statement, context)),
              execution: functionExecution(property, context),
              kind: 'function' as const,
              parameters: property.parameters
                .filter((parameter) => !isThisParameter(parameter))
                .map((parameter) => lowerParameter(parameter, context)),
              returns: hasModifier(property, ts.SyntaxKind.AsyncKeyword) ? asyncTaskType(property, context) : undefined,
            };
            if (ts.isComputedPropertyName(property.name)) {
              return {
                key: lowerExpression(property.name.expression, context),
                kind: 'computedProperty' as const,
                value,
              };
            }
            return {
              kind: 'property' as const,
              name: propertyName(property.name, context),
              value,
            };
          } finally {
            context.classThis = previousClassThis;
          }
        }
        return unsupported(property, context, 'object literal member');
      }),
    };
  }
  if (ts.isPropertyAccessExpression(node)) {
    const webGpuConstantNamespace =
      ts.isIdentifier(node.expression) &&
      webGpuConstantNamespaces.has(node.expression.text) &&
      !isLexicallyBound(node.expression, context)
        ? node.expression.text
        : undefined;
    const objectIsGlobalObject =
      ts.isIdentifier(node.expression) &&
      node.expression.text === 'Object' &&
      !isLexicallyBound(node.expression, context);
    const objectIsCanvasElement =
      canvasElementMembers.has(node.name.text) && isBoundCanvasElementExpression(node.expression, context);
    const objectIsWebGpuDevice =
      webGpuDeviceMembers.has(node.name.text) &&
      isBoundNamedPlatformExpression(node.expression, context, 'GPUDevice', context.webGpuDeviceBindingNames, 'device');
    const objectIsWebGpuQueue =
      webGpuQueueMembers.has(node.name.text) &&
      isBoundNamedPlatformExpression(node.expression, context, 'GPUQueue', context.webGpuQueueBindingNames, 'queue');
    const objectIsWebGpuCanvasContext =
      webGpuCanvasContextMembers.has(node.name.text) &&
      isBoundNamedPlatformExpression(
        node.expression,
        context,
        'GPUCanvasContext',
        context.webGpuCanvasContextBindingNames,
        'context',
      );
    const objectIsWebGpuLimits =
      webGpuLimitsMembers.has(node.name.text) &&
      isBoundNamedPlatformExpression(
        node.expression,
        context,
        'GPUSupportedLimits',
        context.webGpuLimitsBindingNames,
        'limits',
      );
    const objectIsDomWindow = isBoundGlobalRootExpression(
      node.expression,
      context,
      'window',
      context.domWindowBindingNames,
    );
    const objectIsDomDocument = isBoundGlobalRootExpression(
      node.expression,
      context,
      'document',
      context.domDocumentBindingNames,
    );
    const objectIsDomNavigator = isBoundGlobalRootExpression(
      node.expression,
      context,
      'navigator',
      context.domNavigatorBindingNames,
    );
    return {
      binding: webGpuConstantNamespace
        ? 'WebGpuConstantsBackend'
        : objectIsCanvasElement
          ? 'CanvasElementBackend'
          : objectIsWebGpuDevice
            ? 'WebGpuDeviceBackend'
            : objectIsWebGpuQueue
              ? 'WebGpuQueueBackend'
              : objectIsWebGpuCanvasContext
                ? 'WebGpuCanvasContextBackend'
                : objectIsWebGpuLimits
                  ? 'WebGpuLimitsBackend'
                  : objectIsDomWindow
                    ? 'DomWindowBackend'
                    : objectIsDomDocument
                      ? 'DomDocumentBackend'
                      : objectIsDomNavigator
                        ? 'DomNavigatorBackend'
                        : objectIsGlobalObject
                          ? 'DynamicObject'
                          : isBoundPlatformExpression(node.expression, context, 'CanvasRenderingContext2D')
                            ? 'Canvas2dBackend'
                            : isBoundPlatformExpression(node.expression, context, 'WebGL2RenderingContext')
                              ? 'WebGl2Backend'
                              : undefined,
      kind: 'property',
      name: node.name.text,
      object: webGpuConstantNamespace
        ? { kind: 'literal', value: webGpuConstantNamespace }
        : lowerExpression(node.expression, context),
      optional: ts.isOptionalChain(node),
    };
  }
  if (ts.isElementAccessExpression(node) && node.argumentExpression) {
    const webGlBinding = isBoundPlatformExpression(node.expression, context, 'WebGL2RenderingContext');
    return {
      binding: webGlBinding ? 'WebGl2Backend' : undefined,
      index: lowerExpression(node.argumentExpression, context),
      kind: 'element',
      object: lowerExpression(node.expression, context),
      optional: ts.isOptionalChain(node),
      webGlComputedDomain: webGlBinding ? webGlComputedConstantDomain(node.argumentExpression) : undefined,
    };
  }
  if (ts.isCallExpression(node)) {
    const promiseMethod = globalPromiseMethod(node, context);
    if (promiseMethod === 'resolve') {
      const value = node.arguments[0] ? lowerExpression(node.arguments[0], context) : undefined;
      const output = taskFactoryOutput(node, value, context, true);
      const origin = taskExpressionOrigin(node, context, 'ready');
      context.taskConstructions.push({ kind: 'ready', origin, output });
      return { kind: 'taskReady', origin, output, value };
    }
    if (promiseMethod === 'reject') {
      const rejection = node.arguments[0];
      if (!rejection) unsupported(node, context, 'Promise.reject without a rejection value');
      const output = taskFactoryOutput(node, undefined, context, false);
      const origin = taskExpressionOrigin(node, context, 'reject');
      context.taskConstructions.push({ kind: 'reject', origin, output });
      return { kind: 'taskReject', origin, output, rejection: lowerExpression(rejection, context) };
    }
    if (promiseMethod === 'all') {
      const tasks = node.arguments[0];
      if (!tasks) unsupported(node, context, 'Promise.all without a task collection');
      const output = taskFactoryOutput(node, undefined, context, false);
      const origin = taskExpressionOrigin(node, context, 'join-all');
      context.taskConstructions.push({ kind: 'join-all', origin, output });
      return { kind: 'taskAll', origin, output, tasks: lowerExpression(tasks, context) };
    }
    if (promiseMethod === 'allSettled') {
      const output = taskFactoryOutput(node, undefined, context, false);
      context.taskConstructions.push({
        kind: 'join-all-settled',
        origin: taskExpressionOrigin(node, context, 'join-all-settled'),
        output,
      });
    } else if (
      ts.isPropertyAccessExpression(node.expression) &&
      ['then', 'catch', 'finally'].includes(node.expression.name.text)
    ) {
      const kind = node.expression.name.text as 'catch' | 'finally' | 'then';
      context.taskConstructions.push({
        kind,
        origin: taskExpressionOrigin(node, context, kind),
        output: taskFactoryOutput(node, undefined, context, false),
      });
    }
    return {
      arguments: node.arguments.map((argument) => lowerExpression(argument, context)),
      callee: lowerExpression(node.expression, context),
      kind: 'call',
      optional: Boolean(node.questionDotToken),
      typeArguments: node.typeArguments?.map((argument) => lowerType(argument, context)) ?? [],
    };
  }
  if (ts.isSpreadElement(node))
    return {
      expression: lowerExpression(node.expression, context),
      kind: 'spread',
    };
  if (ts.isTypeOfExpression(node)) {
    if (
      ts.isIdentifier(node.expression) &&
      !isLexicallyBound(node.expression, context) &&
      !context.externalValues.has(node.expression.text)
    ) {
      return {
        arguments: [{ kind: 'literal', value: node.expression.text }],
        callee: {
          kind: 'property',
          name: 'typeofGlobal',
          object: { kind: 'identifier', name: '_Runtime' },
        },
        kind: 'call',
        typeArguments: [],
      };
    }
    return {
      kind: 'unary',
      operand: lowerExpression(node.expression, context),
      operator: 'typeof',
      postfix: false,
    };
  }
  if (ts.isDeleteExpression(node)) {
    return {
      kind: 'unary',
      operand: lowerExpression(node.expression, context),
      operator: 'delete',
      postfix: false,
    };
  }
  if (ts.isNewExpression(node)) {
    const callee = node.expression;
    const nativeHostConstructorName =
      ts.isIdentifier(callee) && !isLexicallyBound(callee, context) && !context.externalValues.has(callee.text)
        ? callee.text
        : ts.isPropertyAccessExpression(callee) &&
            ts.isIdentifier(callee.expression) &&
            callee.expression.text === 'globalThis' &&
            !isLexicallyBound(callee.expression, context) &&
            !context.externalValues.has(callee.expression.text)
          ? callee.name.text
          : undefined;
    const nativeHostConstructor = portConfig.typeLowering.nativeHostConstructors.find(
      (constructor) => constructor.global === nativeHostConstructorName,
    );
    if (nativeHostConstructor) {
      return {
        arguments: node.arguments?.map((argument) => lowerExpression(argument, context)) ?? [],
        capability: nativeHostConstructor.capability,
        kind: 'hostConstruct',
        resultType: nativeHostConstructor.resultType,
      };
    }
    return {
      arguments: node.arguments?.map((argument) => lowerExpression(argument, context)) ?? [],
      callee: lowerExpression(node.expression, context),
      kind: 'new',
      typeArguments: node.typeArguments?.map((argument) => lowerType(argument, context)) ?? [],
    };
  }
  if (ts.isConditionalExpression(node)) {
    return {
      condition: lowerExpression(node.condition, context),
      kind: 'conditional',
      whenFalse: lowerExpression(node.whenFalse, context),
      whenTrue: lowerExpression(node.whenTrue, context),
    };
  }
  if (ts.isBinaryExpression(node)) {
    const operator = node.operatorToken.getText(context.sourceFile);
    const assignment =
      node.operatorToken.kind >= ts.SyntaxKind.FirstAssignment &&
      node.operatorToken.kind <= ts.SyntaxKind.LastAssignment;
    return {
      kind: assignment ? 'assignment' : 'binary',
      left: lowerExpression(node.left, context),
      operator,
      right: lowerExpression(node.right, context),
    };
  }
  if (ts.isPrefixUnaryExpression(node) || ts.isPostfixUnaryExpression(node)) {
    return {
      kind: 'unary',
      operand: lowerExpression(node.operand, context),
      operator: ts.tokenToString(node.operator) ?? unsupported(node, context, 'unary operator'),
      postfix: ts.isPostfixUnaryExpression(node),
    };
  }
  if (ts.isArrowFunction(node) || ts.isFunctionExpression(node)) {
    const previousClassThis = context.classThis;
    if (ts.isFunctionExpression(node)) context.classThis = false;
    try {
      const loweredParameters = lowerParameterList(node.parameters, context);
      const expression = ts.isBlock(node.body) ? undefined : lowerExpression(node.body, context);
      return {
        body: ts.isBlock(node.body)
          ? [
              ...loweredParameters.prefix,
              ...node.body.statements.map((statement) => lowerStatement(statement, context)),
            ]
          : loweredParameters.prefix.length > 0
            ? [...loweredParameters.prefix, { expression, kind: 'return' }]
            : [],
        expression: loweredParameters.prefix.length > 0 ? undefined : expression,
        execution: functionExecution(node, context),
        kind: 'function',
        name: ts.isFunctionExpression(node) ? node.name?.text : undefined,
        parameters: loweredParameters.parameters,
        returns: hasModifier(node, ts.SyntaxKind.AsyncKeyword)
          ? asyncTaskType(node, context)
          : node.type
            ? lowerType(node.type, context)
            : undefined,
      };
    } finally {
      context.classThis = previousClassThis;
    }
  }
  return unsupported(node, context, `expression ${ts.SyntaxKind[node.kind] ?? node.kind}`);
}

function globalPromiseMethod(
  node: ts.CallExpression,
  context: LoweringContext,
): 'all' | 'allSettled' | 'reject' | 'resolve' | undefined {
  if (!ts.isPropertyAccessExpression(node.expression)) return undefined;
  const method = node.expression.name.text;
  if (!['all', 'allSettled', 'reject', 'resolve'].includes(method)) return undefined;
  const receiver = node.expression.expression;
  if (ts.isIdentifier(receiver)) {
    return receiver.text === 'Promise' && !isLexicallyBound(receiver, context)
      ? (method as 'all' | 'allSettled' | 'reject' | 'resolve')
      : undefined;
  }
  return ts.isPropertyAccessExpression(receiver) &&
    ts.isIdentifier(receiver.expression) &&
    receiver.expression.text === 'globalThis' &&
    receiver.name.text === 'Promise'
    ? (method as 'all' | 'allSettled' | 'reject' | 'resolve')
    : undefined;
}

function taskFactoryOutput(
  node: ts.CallExpression,
  value: IrExpression | undefined,
  context: LoweringContext,
  voidWhenAbsent: boolean,
): IrType {
  const typeArgument = node.typeArguments?.[0];
  if (typeArgument) return lowerType(typeArgument, context);
  const contextual = contextualExpressionType(node, context, new Set());
  const contextualOutput =
    contextual?.kind === 'task'
      ? contextual.output
      : contextual && isAsyncReturnExpression(node)
        ? contextual
        : undefined;
  if (contextualOutput && !recoveryTypeContainsDynamic(contextualOutput)) return contextualOutput;
  if (voidWhenAbsent && node.arguments[0]) {
    const inferred = inferRecoveryExpressionType(node.arguments[0], context, new Set());
    const inferredOutput = inferred?.kind === 'task' ? inferred.output : inferred;
    if (inferredOutput && !recoveryTypeContainsDynamic(inferredOutput)) return inferredOutput;
  }
  if (!value) return voidWhenAbsent ? { kind: 'primitive', name: 'Void' } : { kind: 'dynamic' };
  if (value.kind === 'cast') return value.type;
  if (value.kind === 'literal') {
    if (typeof value.value === 'boolean') return { kind: 'primitive', name: 'Bool' };
    if (typeof value.value === 'number') return { kind: 'primitive', name: 'Float' };
    if (typeof value.value === 'string') return { kind: 'primitive', name: 'String' };
  }
  if (value.kind === 'identifier' && value.name === 'Undefined') return { kind: 'primitive', name: 'Void' };
  return { kind: 'dynamic' };
}

function isAsyncReturnExpression(node: ts.Expression): boolean {
  let current: ts.Expression = node;
  for (;;) {
    const parent = current.parent;
    if (
      (ts.isParenthesizedExpression(parent) ||
        ts.isAsExpression(parent) ||
        ts.isTypeAssertionExpression(parent) ||
        ts.isSatisfiesExpression(parent)) &&
      parent.expression === current
    ) {
      current = parent;
      continue;
    }
    if (ts.isReturnStatement(parent) && parent.expression === current) {
      const owner = findEnclosingFunction(parent);
      return Boolean(owner && hasModifier(owner, ts.SyntaxKind.AsyncKeyword));
    }
    return ts.isArrowFunction(parent) && parent.body === current && hasModifier(parent, ts.SyntaxKind.AsyncKeyword);
  }
}

function taskExpressionOrigin(node: ts.Node, context: LoweringContext, operation: string) {
  const sourceOrigin = origin(node, context);
  const labels: string[] = [];
  let current: ts.Node | undefined = node.parent;
  while (current && !ts.isSourceFile(current)) {
    const label = lexicalNodeLabel(current, context.sourceFile);
    if (label && labels[0] !== label) labels.unshift(label);
    current = current.parent;
  }
  const shortFingerprint = sourceOrigin.fingerprint.slice('sha256:'.length, 'sha256:'.length + 12);
  labels.push(`${operation}:${String(sourceOrigin.line)}:${String(sourceOrigin.column)}:${shortFingerprint}`);
  return { ...sourceOrigin, lexicalPath: labels.join('.') };
}

function isThisParameter(node: ts.ParameterDeclaration): boolean {
  return ts.isIdentifier(node.name) && node.name.text === 'this';
}

function lowerIdentifier(name: string, context: LoweringContext, locallyBound = false): IrExpression {
  if (name === 'Math' || name === 'Number' || name === 'Error') return { kind: 'identifier', name };
  if (name === 'undefined') {
    return { kind: 'identifier', name: 'Undefined' };
  }
  if (name === 'NaN')
    return {
      kind: 'property',
      name: 'NAN',
      object: { kind: 'identifier', name: 'Float' },
    };
  if (name === 'Infinity') {
    return {
      kind: 'property',
      name: 'INFINITY',
      object: { kind: 'identifier', name: 'Float' },
    };
  }
  const external = context.externalValues.get(name);
  if (external) {
    return {
      arguments: [
        { kind: 'literal', value: external.specifier },
        { kind: 'literal', value: external.imported },
      ],
      callee: {
        kind: 'property',
        name: 'externalValue',
        object: { kind: 'identifier', name: '_Runtime' },
      },
      kind: 'call',
      typeArguments: [],
    };
  }
  if (!locallyBound && (platformGlobalValues.has(name) || platformDynamicTypes.has(name) || name.startsWith('GPU'))) {
    return {
      arguments: [{ kind: 'literal', value: name }],
      callee: {
        kind: 'property',
        name: 'globalValue',
        object: { kind: 'identifier', name: '_Runtime' },
      },
      kind: 'call',
      typeArguments: [],
    };
  }
  return { kind: 'identifier', name };
}

function isLexicallyBound(identifier: ts.Identifier, context: LoweringContext): boolean {
  let current: ts.Node | undefined = identifier.parent;
  while (current) {
    if (ts.isFunctionLike(current) || ts.isSourceFile(current)) {
      let bindings = context.scopeBindings.get(current);
      if (!bindings) {
        const collected = new Set<string>();
        if (ts.isFunctionLike(current)) {
          for (const parameter of current.parameters) collectBindingNames(parameter.name, collected);
        }
        const root = ts.isSourceFile(current) ? current : 'body' in current ? current.body : undefined;
        if (root) {
          const visit = (node: ts.Node): void => {
            if (node !== root && ts.isFunctionLike(node)) {
              if (node.name && ts.isIdentifier(node.name)) collected.add(node.name.text);
              return;
            }
            if (ts.isVariableDeclaration(node)) collectBindingNames(node.name, collected);
            if ((ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node)) && node.name) {
              collected.add(node.name.text);
            }
            ts.forEachChild(node, visit);
          };
          visit(root);
        }
        bindings = collected;
        context.scopeBindings.set(current, bindings);
      }
      if (bindings.has(identifier.text)) return true;
    }
    current = current.parent;
  }
  return false;
}

function isTypeNameLexicallyBound(identifier: ts.Identifier, context: LoweringContext): boolean {
  let current: ts.Node | undefined = identifier.parent;
  while (current) {
    if (
      'typeParameters' in current &&
      Array.isArray(current.typeParameters) &&
      (current.typeParameters as readonly ts.TypeParameterDeclaration[]).some(
        (parameter) => parameter.name.text === identifier.text,
      )
    ) {
      return true;
    }
    if (ts.isSourceFile(current) || ts.isModuleBlock(current) || ts.isBlock(current)) {
      let bindings = context.typeScopeBindings.get(current);
      if (!bindings) {
        const collected = new Set<string>();
        for (const statement of current.statements) {
          if (
            (ts.isClassDeclaration(statement) ||
              ts.isEnumDeclaration(statement) ||
              ts.isInterfaceDeclaration(statement) ||
              ts.isTypeAliasDeclaration(statement)) &&
            statement.name
          ) {
            collected.add(statement.name.text);
          }
          if (ts.isModuleDeclaration(statement) && ts.isIdentifier(statement.name)) {
            collected.add(statement.name.text);
          }
          if (ts.isImportEqualsDeclaration(statement)) collected.add(statement.name.text);
          if (ts.isImportDeclaration(statement) && statement.importClause) {
            if (statement.importClause.name) collected.add(statement.importClause.name.text);
            const bindings_ = statement.importClause.namedBindings;
            if (bindings_ && ts.isNamespaceImport(bindings_)) collected.add(bindings_.name.text);
            if (bindings_ && ts.isNamedImports(bindings_)) {
              bindings_.elements.forEach((element) => collected.add(element.name.text));
            }
          }
        }
        bindings = collected;
        context.typeScopeBindings.set(current, bindings);
      }
      if (bindings.has(identifier.text)) return true;
    }
    current = current.parent;
  }
  return false;
}

function collectBindingNames(name: ts.BindingName, output: Set<string>): void {
  if (ts.isIdentifier(name)) {
    output.add(name.text);
    return;
  }
  for (const element of name.elements) {
    if (!ts.isOmittedExpression(element)) collectBindingNames(element.name, output);
  }
}

function propertyName(node: ts.PropertyName, context: LoweringContext): string {
  if (ts.isIdentifier(node) || ts.isStringLiteral(node) || ts.isNumericLiteral(node)) return node.text;
  if (ts.isComputedPropertyName(node)) {
    const sourceName = node.expression.getText(context.sourceFile).replace(/[^A-Za-z0-9_]/gu, '_');
    return `__${sourceName}`;
  }
  return unsupported(node, context, 'computed property name');
}

function origin(node: ts.Node, context: LoweringContext): SourceOrigin {
  const position = context.sourceFile.getLineAndCharacterOfPosition(node.getStart(context.sourceFile));
  return {
    column: position.character + 1,
    fingerprint: `sha256:${createHash('sha256')
      .update(
        fingerprintPrinter.printNode(ts.EmitHint.Unspecified, node, context.sourceFile).replace(/\s+/gu, ' ').trim(),
      )
      .digest('hex')}`,
    line: position.line + 1,
    packageName: context.packageName,
    source: path.relative(context.workspaceDirectory, context.sourceFile.fileName),
  };
}

function hasModifier(node: ts.Node, kind: ts.SyntaxKind): boolean {
  return ts.canHaveModifiers(node) && ts.getModifiers(node)?.some((modifier) => modifier.kind === kind) === true;
}

function unsupported(node: ts.Node, context: LoweringContext, description: string): never {
  const position = context.sourceFile.getLineAndCharacterOfPosition(node.getStart(context.sourceFile));
  const diagnostic = {
    column: position.character + 1,
    line: position.line + 1,
    message: `Unsupported TypeScript ${description}`,
    source: path.relative(context.workspaceDirectory, context.sourceFile.fileName),
  } satisfies LoweringDiagnostic;
  context.diagnostics.push(diagnostic);
  throw new UnsupportedSyntaxError(
    `${diagnostic.source}:${diagnostic.line}:${diagnostic.column}: ${diagnostic.message}`,
  );
}

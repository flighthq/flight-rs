export interface SourceOrigin {
  column: number;
  fingerprint: string;
  line: number;
  packageName: string;
  source: string;
}

export interface IrTaskOrigin extends SourceOrigin {
  lexicalPath: string;
}

export const PORTABLE_TASK_RUST_LOWERING_REASON = 'Portable task Rust lowering is not implemented.';

export type IrFunctionExecution =
  | { kind: 'sync' }
  | { kind: 'portableTask'; origin: IrTaskOrigin }
  | { capability: string; kind: 'hostTaskPlaceholder'; origin: IrTaskOrigin; reason: string };

export interface IrAsyncTaskOperations {
  asyncIterations: number;
  awaits: number;
  promiseAll: number;
  promiseAllSettled: number;
  promiseCatch: number;
  promiseFinally: number;
  promiseReject: number;
  promiseResolve: number;
  promiseThen: number;
  voidExpressions: number;
}

export interface IrAsyncTaskScope {
  execution: Exclude<IrFunctionExecution, { kind: 'sync' }>;
  matchesLegacyErasurePath: boolean;
  operations: IrAsyncTaskOperations;
}

export type IrType =
  | { kind: 'anonymous'; fields: IrTypeField[]; extends: IrType[] }
  | { kind: 'array'; element: IrType }
  | { kind: 'dynamic' }
  | { kind: 'function'; parameters: IrType[]; returns: IrType }
  | { kind: 'named'; arguments: IrType[]; name: string }
  | { kind: 'nullable'; inner: IrType }
  | { kind: 'primitive'; name: 'Bool' | 'Float' | 'Int' | 'String' | 'Void' }
  | { kind: 'task'; output: IrType }
  | { kind: 'union'; variants: IrType[] };

export interface IrTypeField {
  contextualParameters?: IrParameter[] | undefined;
  discriminantValue?: boolean | number | string | undefined;
  name: string;
  optional: boolean;
  type: IrType;
}

export interface IrParameter {
  initializer?: IrExpression | undefined;
  name: string;
  optional: boolean;
  rest: boolean;
  type: IrType;
}

export type IrHostConstructorCapability = 'ImageData' | 'OffscreenCanvas' | 'URL';

export type IrExpression =
  | { kind: 'array'; elements: IrExpression[] }
  | { kind: 'await'; expression: IrExpression }
  | { kind: 'assignment'; left: IrExpression; operator: string; right: IrExpression }
  | { kind: 'binary'; left: IrExpression; operator: string; right: IrExpression }
  | { kind: 'call'; arguments: IrExpression[]; callee: IrExpression; optional?: boolean; typeArguments: IrType[] }
  | { kind: 'cast'; expression: IrExpression; type: IrType }
  | { kind: 'conditional'; condition: IrExpression; whenFalse: IrExpression; whenTrue: IrExpression }
  | {
      binding?: 'WebGl2Backend' | undefined;
      kind: 'element';
      object: IrExpression;
      index: IrExpression;
      optional?: boolean | undefined;
      webGlComputedDomain?: 'GlBlendEquation' | 'GlBlendFactor' | undefined;
    }
  | {
      execution: IrFunctionExecution;
      kind: 'function';
      name?: string | undefined;
      parameters: IrParameter[];
      body: IrStatement[];
      expression?: IrExpression | undefined;
      returns?: IrType | undefined;
    }
  | {
      arguments: IrExpression[];
      capability: IrHostConstructorCapability;
      kind: 'hostConstruct';
      resultType: string;
    }
  | { kind: 'identifier'; name: string }
  | { kind: 'literal'; value: boolean | null | number | string }
  | { kind: 'new'; arguments: IrExpression[]; callee: IrExpression; typeArguments: IrType[] }
  | { kind: 'object'; properties: IrObjectMember[] }
  | {
      binding?:
        | 'Canvas2dBackend'
        | 'CanvasElementBackend'
        | 'DomDocumentBackend'
        | 'DomNavigatorBackend'
        | 'DomWindowBackend'
        | 'DynamicObject'
        | 'WebGl2Backend'
        | 'WebGpuCanvasContextBackend'
        | 'WebGpuConstantsBackend'
        | 'WebGpuDeviceBackend'
        | 'WebGpuLimitsBackend'
        | 'WebGpuQueueBackend'
        | undefined;
      kind: 'property';
      name: string;
      object: IrExpression;
      optional?: boolean | undefined;
    }
  | { flags: string; kind: 'regexp'; pattern: string }
  | { kind: 'template'; parts: Array<IrExpression | string> }
  | { kind: 'spread'; expression: IrExpression }
  | { kind: 'unary'; operand: IrExpression; operator: string; postfix: boolean };

export type IrObjectMember =
  | { key: IrExpression; kind: 'computedProperty'; value: IrExpression }
  | { kind: 'property'; name: string; value: IrExpression }
  | { kind: 'spread'; expression: IrExpression };

export type IrStatement =
  | { kind: 'block'; statements: IrStatement[] }
  | { kind: 'break' }
  | { kind: 'continue' }
  | { kind: 'do'; body: IrStatement; condition: IrExpression }
  | { kind: 'expression'; expression: IrExpression }
  | {
      kind: 'for';
      condition?: IrExpression | undefined;
      increment?: IrExpression | undefined;
      initializer?: IrExpression | IrVariable[] | undefined;
      body: IrStatement;
    }
  | {
      async: boolean;
      bindings: IrVariable[];
      body: IrStatement;
      iterable: IrExpression;
      kind: 'forOf';
      variable: string;
    }
  | { kind: 'if'; condition: IrExpression; consequent: IrStatement; otherwise?: IrStatement | undefined }
  | { kind: 'return'; expression?: IrExpression | undefined }
  | { kind: 'switch'; expression: IrExpression; cases: IrSwitchCase[] }
  | { kind: 'throw'; expression: IrExpression }
  | {
      catchBody?: IrStatement | undefined;
      catchName?: string | undefined;
      finallyBody?: IrStatement | undefined;
      kind: 'try';
      tryBody: IrStatement;
    }
  | { kind: 'variable'; declarations: IrVariable[] }
  | { kind: 'while'; body: IrStatement; condition: IrExpression };

export interface IrSwitchCase {
  expression?: IrExpression | undefined;
  statements: IrStatement[];
}

export interface IrVariable {
  initializer?: IrExpression | undefined;
  mutable: boolean;
  name: string;
  type?: IrType | undefined;
}

export interface IrFunctionDeclaration {
  body: IrStatement[];
  execution: IrFunctionExecution;
  exported: boolean;
  targetBody?: string | undefined;
  kind: 'function';
  name: string;
  origin: SourceOrigin;
  parameters: IrParameter[];
  returns: IrType;
  typeParameters: string[];
}

export interface IrVariableDeclaration extends IrVariable {
  exported: boolean;
  kind: 'variable';
  origin: SourceOrigin;
}

export interface IrTypeDeclaration {
  exported: boolean;
  kind: 'type';
  name: string;
  origin: SourceOrigin;
  // Emitted as a module-private type when its source identity is implementation-only.
  packagePrivate?: boolean;
  type: IrType;
  typeParameters: string[];
}

export interface IrEnumDeclaration {
  exported: boolean;
  kind: 'enum';
  members: Array<{ initializer?: IrExpression | undefined; name: string }>;
  methods: IrFunctionDeclaration[];
  name: string;
  origin: SourceOrigin;
  packagePrivate?: boolean;
}

export interface IrClassField {
  initializer?: IrExpression | undefined;
  mutable: boolean;
  name: string;
  public: boolean;
  static: boolean;
  type: IrType;
}

export interface IrClassMethod {
  body: IrStatement[];
  execution: IrFunctionExecution;
  name: string;
  parameters: IrParameter[];
  public: boolean;
  returns: IrType;
  static: boolean;
  typeParameters: string[];
}

export interface IrClassDeclaration {
  constructorBody: IrStatement[];
  constructorParameters: IrParameter[];
  exported: boolean;
  extends?: IrType | undefined;
  fields: IrClassField[];
  kind: 'class';
  methods: IrClassMethod[];
  name: string;
  origin: SourceOrigin;
  packagePrivate?: boolean;
  typeParameters: string[];
}

export type IrDeclaration =
  | IrClassDeclaration
  | IrEnumDeclaration
  | IrFunctionDeclaration
  | IrTypeDeclaration
  | IrVariableDeclaration;

export interface IrModule {
  declarations: IrDeclaration[];
  rustModule?: string;
  imports: string[];
  name: string;
  packageName: string;
  source?: string | undefined;
}

export interface LoweringDiagnostic {
  column: number;
  line: number;
  message: string;
  source: string;
}

export interface LoweringResult {
  accountedDeclarations: number;
  asyncTasks: IrAsyncTaskScope[];
  declarations: IrDeclaration[];
  diagnostics: LoweringDiagnostic[];
}

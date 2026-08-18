// @generated from upstream/packages/types/src/Abc.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub initializer: f64,
    pub traits: Vec<AbcTrait>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:9 (sha256:37795ca287195af002dc2ffce67b2c13f8bd2180ce31d09dbb3de78a3a0740ca)
#[derive(Clone, Default)]
pub struct AbcFile {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub classes: Vec<AbcClass>,
    pub constant_pool: AbcConstantPool,
    pub instances: Vec<AbcInstance>,
    pub major_version: f64,
    pub metadata: Vec<AbcMetadata>,
    pub method_bodies: Vec<AbcMethodBody>,
    pub methods: Vec<AbcMethod>,
    pub minor_version: f64,
    pub scripts: Vec<AbcScript>,
}
impl PartialEq for AbcFile {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:21 (sha256:32ae95dcc7d22d69461d4060c53eb50cdd377aa71bb215906736ec0c036da37f)
#[derive(Clone, Default)]
pub struct AbcConstantPool {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub doubles: Vec<f64>,
    pub integers: Vec<f64>,
    pub multinames: Vec<AbcMultiname>,
    pub namespace_sets: Vec<Vec<f64>>,
    pub namespaces: Vec<AbcNamespace>,
    pub strings: Vec<String>,
    pub unsigned_integers: Vec<f64>,
}
impl PartialEq for AbcConstantPool {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:31 (sha256:f2d532b17879cee596a07255b18e68602eed53931f0f98b31a2e3de565b0d076)
#[derive(Clone, Default)]
pub struct AbcNamespace {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: f64,
    pub name: f64,
}
impl PartialEq for AbcNamespace {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:41 (sha256:a530f1994767f0978b42abb9d0e32a5edfe81713180e2a2a86d594892bcf840c)
#[derive(Clone, Default)]
pub struct AbcMultiname {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: f64,
    pub name: f64,
    pub namespace: f64,
    pub namespace_set: f64,
    pub parameters: Vec<f64>,
    pub type_name: f64,
}
impl PartialEq for AbcMultiname {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:55 (sha256:8d2f73ec9b81a8a1ac5bb0722140d0fafbb067a24bfb32cac67a7ba0f1d2d653)
#[derive(Clone, Default)]
pub struct AbcMethod {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub flags: f64,
    pub name: f64,
    pub parameter_names: Vec<f64>,
    pub parameter_types: Vec<f64>,
    pub optional_values: Vec<AbcOptionalValue>,
    pub return_type: f64,
}
impl PartialEq for AbcMethod {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:69 (sha256:a9c73e221d48e582283957fdad84bc860388ea95f567474b32027eead21cd6f9)
#[derive(Clone, Default)]
pub struct AbcOptionalValue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: f64,
    pub value: f64,
}
impl PartialEq for AbcOptionalValue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:75 (sha256:ad7ad33079afef273be0ad0a7751c608f2f5b1bd4e60f07f627b6f4fcf0a1e6f)
#[derive(Clone, Default)]
pub struct AbcMetadata {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub items: Vec<AbcMetadataItem>,
    pub name: f64,
}
impl PartialEq for AbcMetadata {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:81 (sha256:f37603c793972d3f584b9aac0e2527834fb667fa556bbb9298804f03502661d9)
#[derive(Clone, Default)]
pub struct AbcMetadataItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub key: f64,
    pub value: f64,
}
impl PartialEq for AbcMetadataItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:87 (sha256:ae7540321295da56c929433c7d7d02b2bda7e1342c96510fe81b82a4af59a3d8)
#[derive(Clone, Default)]
pub struct AbcInstance {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub flags: f64,
    pub initializer: f64,
    pub interfaces: Vec<f64>,
    pub name: f64,
    pub protected_namespace: f64,
    pub super_name: f64,
    pub traits: Vec<AbcTrait>,
}
impl PartialEq for AbcInstance {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:100 (sha256:b91ac5356f278b751d9646f9420548fde9a99189a853bb651da26d79cdd6236a)
#[derive(Clone, Default)]
pub struct AbcClass {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub initializer: f64,
    pub traits: Vec<AbcTrait>,
}
impl PartialEq for AbcClass {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:106 (sha256:73969a7c1b39db7bd03fa0703f4d195874cd6359dd59688b7be53f117668a97f)
#[derive(Clone, Default)]
pub struct AbcScript {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub initializer: f64,
    pub traits: Vec<AbcTrait>,
}
impl PartialEq for AbcScript {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:115 (sha256:74c21326aec08c9f2f2e16e6d64e3300ccdb3fc5423d6bf9b2c1145b6def2a8a)
#[derive(Clone, Default)]
pub struct AbcTrait {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: f64,
    pub class_index: f64,
    pub dispatch_id: f64,
    pub method_index: f64,
    pub metadata: Vec<f64>,
    pub name: f64,
    pub kind: f64,
    pub slot_id: f64,
    pub type_name: f64,
    pub value_index: f64,
    pub value_kind: f64,
}
impl PartialEq for AbcTrait {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:137 (sha256:9cc72a5b8829a8547d37ce49de735b3816bbdb3fcd0688c9dff23895e0e0038b)
#[derive(Clone, Default)]
pub struct AbcMethodBody {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub code: Vec<u8>,
    pub exceptions: Vec<AbcException>,
    pub init_scope_depth: f64,
    pub local_count: f64,
    pub max_scope_depth: f64,
    pub max_stack: f64,
    pub method: f64,
    pub traits: Vec<AbcTrait>,
}
impl PartialEq for AbcMethodBody {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:151 (sha256:9dc77231b244962771bd41afab0c54c85d7adff8c6927eb6b5919cc9339d4c94)
#[derive(Clone, Default)]
pub struct AbcException {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub from: f64,
    pub target: f64,
    pub to: f64,
    pub exception_type: f64,
    pub variable_name: f64,
}
impl PartialEq for AbcException {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:161 (sha256:eae8c98dbead8901226b3ecc2451227f78b4f4ce06107da1f7b0f1d5d0ad1763)
pub struct AbcMultinameKind;
impl AbcMultinameKind {
    pub const Multiname: f64 = 9.0_f64;
    pub const MultinameA: f64 = 14.0_f64;
    pub const MultinameL: f64 = 27.0_f64;
    pub const MultinameLA: f64 = 28.0_f64;
    pub const QName: f64 = 7.0_f64;
    pub const QNameA: f64 = 13.0_f64;
    pub const RtqName: f64 = 15.0_f64;
    pub const RtqNameA: f64 = 16.0_f64;
    pub const RtqNameL: f64 = 17.0_f64;
    pub const RtqNameLA: f64 = 18.0_f64;
    pub const TypeName: f64 = 29.0_f64;
}

// Source: upstream/packages/types/src/Abc.ts:175 (sha256:ea86a0edff22927eed0a710faf54e4904f25b6f00b31b2d8027cb486921737ad)
// TypeScript numeric namespace AbcMultinameKind is represented by its generated Rust constants.

// Source: upstream/packages/types/src/Abc.ts:177 (sha256:3a7eee2b0c3a072552f3efc59e8b98be9e5cbd36dcc5fe0eb63a11522048b87a)
pub struct AbcTraitKind;
impl AbcTraitKind {
    pub const Class: f64 = 4.0_f64;
    pub const Const: f64 = 6.0_f64;
    pub const Function: f64 = 5.0_f64;
    pub const Getter: f64 = 2.0_f64;
    pub const Method: f64 = 1.0_f64;
    pub const Setter: f64 = 3.0_f64;
    pub const Slot: f64 = 0.0_f64;
}

// Source: upstream/packages/types/src/Abc.ts:187 (sha256:523c8c7b056eff8e040c9ec2b1af0b05ae71505df46aedf8aa0eacc8b9f295a2)
// TypeScript numeric namespace AbcTraitKind is represented by its generated Rust constants.

// Source: upstream/packages/types/src/Abc.ts:193 (sha256:f9b45a313f2614d54a52c279b9226bcc9c33ba384453b6dca79b0f5c1338c57d)
#[derive(Clone, Default)]
pub struct AbcInstruction {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub offset: f64,
    pub opcode: f64,
    pub operands: Vec<f64>,
}
impl PartialEq for AbcInstruction {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Abc.ts:202 (sha256:42c71aa80c9f192bd5d6c9becab7c9a7aeb0804208fb919a99fa136c6b145c3a)
pub struct AbcOpcode;
impl AbcOpcode {
    pub const Add: f64 = 160.0_f64;
    pub const AddInt: f64 = 197.0_f64;
    pub const ApplyType: f64 = 83.0_f64;
    pub const AsType: f64 = 134.0_f64;
    pub const AsTypeLate: f64 = 135.0_f64;
    pub const BitAnd: f64 = 168.0_f64;
    pub const BitNot: f64 = 151.0_f64;
    pub const BitOr: f64 = 169.0_f64;
    pub const BitXor: f64 = 170.0_f64;
    pub const Breakpoint: f64 = 1.0_f64;
    pub const BreakpointLine: f64 = 242.0_f64;
    pub const Call: f64 = 65.0_f64;
    pub const CallMethod: f64 = 67.0_f64;
    pub const CallProperty: f64 = 70.0_f64;
    pub const CallPropLex: f64 = 76.0_f64;
    pub const CallPropVoid: f64 = 79.0_f64;
    pub const CallStatic: f64 = 68.0_f64;
    pub const CallSuper: f64 = 69.0_f64;
    pub const CallSuperVoid: f64 = 78.0_f64;
    pub const CheckFilter: f64 = 120.0_f64;
    pub const Coerce: f64 = 128.0_f64;
    pub const CoerceAny: f64 = 130.0_f64;
    pub const CoerceBoolean: f64 = 129.0_f64;
    pub const CoerceDouble: f64 = 132.0_f64;
    pub const CoerceInt: f64 = 131.0_f64;
    pub const CoerceObject: f64 = 137.0_f64;
    pub const CoerceString: f64 = 133.0_f64;
    pub const CoerceUint: f64 = 136.0_f64;
    pub const Construct: f64 = 66.0_f64;
    pub const ConstructProp: f64 = 74.0_f64;
    pub const ConstructSuper: f64 = 73.0_f64;
    pub const ConvertBoolean: f64 = 118.0_f64;
    pub const ConvertDouble: f64 = 117.0_f64;
    pub const ConvertInt: f64 = 115.0_f64;
    pub const ConvertObject: f64 = 119.0_f64;
    pub const ConvertString: f64 = 112.0_f64;
    pub const ConvertUint: f64 = 116.0_f64;
    pub const Debug: f64 = 239.0_f64;
    pub const DebugFile: f64 = 241.0_f64;
    pub const DebugLine: f64 = 240.0_f64;
    pub const DecLocal: f64 = 148.0_f64;
    pub const DecLocalInt: f64 = 195.0_f64;
    pub const Decrement: f64 = 147.0_f64;
    pub const DecrementInt: f64 = 193.0_f64;
    pub const DeleteProperty: f64 = 106.0_f64;
    pub const Divide: f64 = 163.0_f64;
    pub const Dup: f64 = 42.0_f64;
    pub const Dxns: f64 = 6.0_f64;
    pub const DxnsLate: f64 = 7.0_f64;
    pub const Equals: f64 = 171.0_f64;
    pub const FindDef: f64 = 95.0_f64;
    pub const FindProperty: f64 = 94.0_f64;
    pub const FindPropStrict: f64 = 93.0_f64;
    pub const GetDescendants: f64 = 89.0_f64;
    pub const GetGlobalScope: f64 = 100.0_f64;
    pub const GetGlobalSlot: f64 = 110.0_f64;
    pub const GetLex: f64 = 96.0_f64;
    pub const GetLocal: f64 = 98.0_f64;
    pub const GetLocal0: f64 = 208.0_f64;
    pub const GetLocal1: f64 = 209.0_f64;
    pub const GetLocal2: f64 = 210.0_f64;
    pub const GetLocal3: f64 = 211.0_f64;
    pub const GetProperty: f64 = 102.0_f64;
    pub const GetScopeObject: f64 = 101.0_f64;
    pub const GetSlot: f64 = 108.0_f64;
    pub const GetSuper: f64 = 4.0_f64;
    pub const GreaterEquals: f64 = 176.0_f64;
    pub const GreaterThan: f64 = 175.0_f64;
    pub const HasNext: f64 = 31.0_f64;
    pub const HasNext2: f64 = 50.0_f64;
    pub const IfEq: f64 = 19.0_f64;
    pub const IfFalse: f64 = 18.0_f64;
    pub const IfGe: f64 = 24.0_f64;
    pub const IfGt: f64 = 23.0_f64;
    pub const IfLe: f64 = 22.0_f64;
    pub const IfLt: f64 = 21.0_f64;
    pub const IfNe: f64 = 20.0_f64;
    pub const IfNge: f64 = 15.0_f64;
    pub const IfNgt: f64 = 14.0_f64;
    pub const IfNle: f64 = 13.0_f64;
    pub const IfNlt: f64 = 12.0_f64;
    pub const IfStrictEq: f64 = 25.0_f64;
    pub const IfStrictNe: f64 = 26.0_f64;
    pub const IfTrue: f64 = 17.0_f64;
    pub const In: f64 = 180.0_f64;
    pub const IncLocal: f64 = 146.0_f64;
    pub const IncLocalInt: f64 = 194.0_f64;
    pub const Increment: f64 = 145.0_f64;
    pub const IncrementInt: f64 = 192.0_f64;
    pub const InitProperty: f64 = 104.0_f64;
    pub const InstanceOf: f64 = 177.0_f64;
    pub const IsType: f64 = 178.0_f64;
    pub const IsTypeLate: f64 = 179.0_f64;
    pub const Jump: f64 = 16.0_f64;
    pub const Kill: f64 = 8.0_f64;
    pub const Label: f64 = 9.0_f64;
    pub const LessEquals: f64 = 174.0_f64;
    pub const LessThan: f64 = 173.0_f64;
    pub const LoadFloat32: f64 = 56.0_f64;
    pub const LoadFloat64: f64 = 57.0_f64;
    pub const LoadInt16: f64 = 54.0_f64;
    pub const LoadInt32: f64 = 55.0_f64;
    pub const LoadInt8: f64 = 53.0_f64;
    pub const LookupSwitch: f64 = 27.0_f64;
    pub const LShift: f64 = 165.0_f64;
    pub const Modulo: f64 = 164.0_f64;
    pub const Multiply: f64 = 162.0_f64;
    pub const MultiplyInt: f64 = 199.0_f64;
    pub const Negate: f64 = 144.0_f64;
    pub const NegateInt: f64 = 196.0_f64;
    pub const NewActivation: f64 = 87.0_f64;
    pub const NewArray: f64 = 86.0_f64;
    pub const NewCatch: f64 = 90.0_f64;
    pub const NewClass: f64 = 88.0_f64;
    pub const NewFunction: f64 = 64.0_f64;
    pub const NewObject: f64 = 85.0_f64;
    pub const NextName: f64 = 30.0_f64;
    pub const NextValue: f64 = 35.0_f64;
    pub const Nop: f64 = 2.0_f64;
    pub const Not: f64 = 150.0_f64;
    pub const Pop: f64 = 41.0_f64;
    pub const PopScope: f64 = 29.0_f64;
    pub const PushByte: f64 = 36.0_f64;
    pub const PushDouble: f64 = 47.0_f64;
    pub const PushFalse: f64 = 39.0_f64;
    pub const PushInt: f64 = 45.0_f64;
    pub const PushNamespace: f64 = 49.0_f64;
    pub const PushNan: f64 = 40.0_f64;
    pub const PushNull: f64 = 32.0_f64;
    pub const PushScope: f64 = 48.0_f64;
    pub const PushShort: f64 = 37.0_f64;
    pub const PushString: f64 = 44.0_f64;
    pub const PushTrue: f64 = 38.0_f64;
    pub const PushUint: f64 = 46.0_f64;
    pub const PushUndefined: f64 = 33.0_f64;
    pub const PushWith: f64 = 28.0_f64;
    pub const ReturnValue: f64 = 72.0_f64;
    pub const ReturnVoid: f64 = 71.0_f64;
    pub const RShift: f64 = 166.0_f64;
    pub const SetGlobalSlot: f64 = 111.0_f64;
    pub const SetLocal: f64 = 99.0_f64;
    pub const SetLocal0: f64 = 212.0_f64;
    pub const SetLocal1: f64 = 213.0_f64;
    pub const SetLocal2: f64 = 214.0_f64;
    pub const SetLocal3: f64 = 215.0_f64;
    pub const SetProperty: f64 = 97.0_f64;
    pub const SetSlot: f64 = 109.0_f64;
    pub const SetSuper: f64 = 5.0_f64;
    pub const SignExtend1: f64 = 80.0_f64;
    pub const SignExtend16: f64 = 82.0_f64;
    pub const SignExtend8: f64 = 81.0_f64;
    pub const StoreFloat32: f64 = 61.0_f64;
    pub const StoreFloat64: f64 = 62.0_f64;
    pub const StoreInt16: f64 = 59.0_f64;
    pub const StoreInt32: f64 = 60.0_f64;
    pub const StoreInt8: f64 = 58.0_f64;
    pub const StrictEquals: f64 = 172.0_f64;
    pub const Subtract: f64 = 161.0_f64;
    pub const SubtractInt: f64 = 198.0_f64;
    pub const Swap: f64 = 43.0_f64;
    pub const Throw: f64 = 3.0_f64;
    pub const Timestamp: f64 = 243.0_f64;
    pub const TypeOf: f64 = 149.0_f64;
    pub const URShift: f64 = 167.0_f64;
}

// Source: upstream/packages/types/src/Abc.ts:371 (sha256:01fb80a91ed3cba9478ab32a9cdda7129acd032fffe39a2ea0254835edb8754a)
// TypeScript numeric namespace AbcOpcode is represented by its generated Rust constants.

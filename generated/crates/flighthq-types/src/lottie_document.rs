// @generated from upstream/packages/types/src/LottieDocument.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/LottieDocument.ts:7 (sha256:c18b725da8d81d6815f2491a289956b53bc53aaf293e12e3572adeb93b88b8c7)
#[derive(Clone)]
pub struct LottieBezierHandle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: crate::FlightUnion2<f64, Vec<f64>>,
    pub y: crate::FlightUnion2<f64, Vec<f64>>,
}
impl PartialEq for LottieBezierHandle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:12 (sha256:62295208edb23fbfba568845028bd4aed2cdbe7599cc9c81a4c03ce484fccc8d)
#[derive(Clone, Default)]
pub struct LottieKeyframe<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub e: Option<T>,
    pub h: Option<f64>,
    pub i: Option<LottieBezierHandle>,
    pub ti: Option<Vec<f64>>,
    pub o: Option<LottieBezierHandle>,
    pub s: Option<T>,
    pub t: f64,
    pub to: Option<Vec<f64>>,
}
impl<T> PartialEq for LottieKeyframe<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:31 (sha256:b2c0738bcde0017502b9eb7ebf1ca70ead5bbf734b1002d6b605f47f95275e72)
#[derive(Clone)]
pub struct LottieStaticProperty<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: Option<f64>,
    pub k: T,
    pub x: Option<String>,
}
impl<T> PartialEq for LottieStaticProperty<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:38 (sha256:f9af23ea9cb976eee80983290bfed64bfe927dbe60c4d3da359457ff0fd524b3)
#[derive(Clone, Default)]
pub struct LottieAnimatedProperty<T> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub k: Vec<LottieKeyframe<T>>,
    pub x: Option<String>,
}
impl<T> PartialEq for LottieAnimatedProperty<T> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:45 (sha256:22271f4737d2e38d40493c82040298c37bbe47798d86f691a24622b43e56df24)
pub type LottieAnimatable<T> =
    crate::FlightUnion2<LottieAnimatedProperty<T>, LottieStaticProperty<T>>;

// Source: upstream/packages/types/src/LottieDocument.ts:47 (sha256:f6022e59ec6b9c1b40b2cb050ba29bf58f0263cad3e9f221c547e6e371532fc4)
#[derive(Clone)]
pub struct LottieSeparatedPositionProperty {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub s: bool,
    pub x: LottieAnimatable<f64>,
    pub y: LottieAnimatable<f64>,
    pub z: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieSeparatedPositionProperty {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:54 (sha256:069a4526a5657ce3204d08e8ea5d1ca9939376a663d89bdd875504c65cba61d8)
pub type LottiePositionProperty =
    crate::FlightUnion2<LottieAnimatable<Vec<f64>>, LottieSeparatedPositionProperty>;

// Source: upstream/packages/types/src/LottieDocument.ts:56 (sha256:d21894626b9d2757d1bec65c881dbd34f84463a37d4eb1cebc327db28cbed122)
#[derive(Clone, Default)]
pub struct LottieTransform {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: Option<LottieAnimatable<Vec<f64>>>,
    pub o: Option<LottieAnimatable<f64>>,
    pub p: Option<LottiePositionProperty>,
    pub r: Option<LottieAnimatable<f64>>,
    pub rz: Option<LottieAnimatable<f64>>,
    pub s: Option<LottieAnimatable<Vec<f64>>>,
    pub sa: Option<LottieAnimatable<f64>>,
    pub sk: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieTransform {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:75 (sha256:ce922284ec58aebfe1997133806dde22e7754a0ac71bda478870249b6a938926)
#[derive(Clone, Default)]
pub struct LottieShapePath {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub c: bool,
    pub i: Vec<Vec<f64>>,
    pub o: Vec<Vec<f64>>,
    pub v: Vec<Vec<f64>>,
}
impl PartialEq for LottieShapePath {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:86 (sha256:7328fabaffdf88b497e22766c3d5a85ab59d0c4005f00ac04228e72920eb74c8)
#[derive(Clone, Default)]
pub struct LottieShapeItemBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
}
impl PartialEq for LottieShapeItemBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:96 (sha256:fff182942d483f27a71d71bede25baa71faa8639785e61872bd72606530bcf1e)
#[derive(Clone, Default)]
pub struct LottieShapeGroup {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub it: Vec<LottieShapeItem>,
    pub np: Option<f64>,
}
impl PartialEq for LottieShapeGroup {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:102 (sha256:d39d2f6063c555885f6d0d038df1fdecbb65ba1f8e60b11f8fc0361ee84a3846)
#[derive(Clone)]
pub struct LottieShapePathItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub ind: Option<f64>,
    pub ks: LottieAnimatable<LottieShapePath>,
}
impl PartialEq for LottieShapePathItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:108 (sha256:7491ca808de9f0829c8981dd48626501dbc8a52ce0ffc38c731938d88475411c)
#[derive(Clone)]
pub struct LottieRectangleShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub p: LottieAnimatable<Vec<f64>>,
    pub r: LottieAnimatable<f64>,
    pub s: LottieAnimatable<Vec<f64>>,
}
impl PartialEq for LottieRectangleShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:118 (sha256:5038e43ee51e5af9d416293bae8994a80f422dd39367b3e467a70bc7ae40a9b5)
#[derive(Clone)]
pub struct LottieEllipseShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub p: LottieAnimatable<Vec<f64>>,
    pub s: LottieAnimatable<Vec<f64>>,
}
impl PartialEq for LottieEllipseShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:124 (sha256:2d65ae3e408cabcee61875942d79d89249bbc8d5b58eb4d40ed408deece01a10)
#[derive(Clone)]
pub struct LottiePolystarShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub ir: Option<LottieAnimatable<f64>>,
    pub is: Option<LottieAnimatable<f64>>,
    pub or: LottieAnimatable<f64>,
    pub os: LottieAnimatable<f64>,
    pub p: LottieAnimatable<Vec<f64>>,
    pub pt: LottieAnimatable<f64>,
    pub r: LottieAnimatable<f64>,
    pub sy: f64,
}
impl PartialEq for LottiePolystarShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:140 (sha256:2b626bfd8b22a1c8d5d687c3191cc8e00ae4fa6fa946ee22feb8edd27d7da371)
#[derive(Clone)]
pub struct LottieFillShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub c: LottieAnimatable<Vec<f64>>,
    pub r: Option<f64>,
    pub o: LottieAnimatable<f64>,
}
impl PartialEq for LottieFillShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:150 (sha256:60ab6bdec3878f30bcaf79ab485dad342270d5c455ff2eb29b1d706a89b0638a)
#[derive(Clone)]
pub struct LottieDashEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub n: String,
    pub nm: Option<String>,
    pub v: LottieAnimatable<f64>,
}
impl PartialEq for LottieDashEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:157 (sha256:9074e9376c831f637386a2cb771f698c87159a9c3fe67cc99631003cfde701f6)
#[derive(Clone)]
pub struct LottieStrokeShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub c: LottieAnimatable<Vec<f64>>,
    pub d: Option<Vec<LottieDashEntry>>,
    pub lc: Option<f64>,
    pub lj: Option<f64>,
    pub ml: Option<f64>,
    pub o: LottieAnimatable<f64>,
    pub w: LottieAnimatable<f64>,
}
impl PartialEq for LottieStrokeShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:170 (sha256:2866dab7a1aa1bd39990ec1adb283859cafe087da82dbfe50bb53b0ccd853b69)
#[derive(Clone)]
pub struct LottieGradient {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub p: f64,
    pub k: LottieAnimatable<Vec<f64>>,
}
impl PartialEq for LottieGradient {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:177 (sha256:1ec1aa507566752e93bcb45581c665fa2fd51ed04521d4ad932f30922ae5fab7)
#[derive(Clone)]
pub struct LottieGradientShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub d: Option<Vec<LottieDashEntry>>,
    pub e: LottieAnimatable<Vec<f64>>,
    pub g: LottieGradient,
    pub a: Option<LottieAnimatable<f64>>,
    pub h: Option<LottieAnimatable<f64>>,
    pub lc: Option<f64>,
    pub lj: Option<f64>,
    pub ml: Option<f64>,
    pub o: Option<LottieAnimatable<f64>>,
    pub t: f64,
    pub s: LottieAnimatable<Vec<f64>>,
    pub w: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieGradientShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:200 (sha256:78c14b421de8ef427c6f79853ca96ad647e8dd6b162fa6b466259c64d49eb087)
#[derive(Clone, Default)]
pub struct LottieTransformShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub a: Option<LottieAnimatable<Vec<f64>>>,
    pub o: Option<LottieAnimatable<f64>>,
    pub p: Option<LottiePositionProperty>,
    pub r: Option<LottieAnimatable<f64>>,
    pub rz: Option<LottieAnimatable<f64>>,
    pub s: Option<LottieAnimatable<Vec<f64>>>,
    pub sa: Option<LottieAnimatable<f64>>,
    pub sk: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieTransformShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:204 (sha256:3922c3c729e943d2ffb6559f8aa3dbee0474c510061245593dbac3f390518091)
#[derive(Clone)]
pub struct LottieTrimPathShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub e: LottieAnimatable<f64>,
    pub m: f64,
    pub o: LottieAnimatable<f64>,
    pub s: LottieAnimatable<f64>,
}
impl PartialEq for LottieTrimPathShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:213 (sha256:b6cf1f9617e0986e5d073fb2c51e37fdb58819c4c48e0327400730ad849bb7b9)
#[derive(Clone, Default)]
pub struct LottieRepeaterShapeItemRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub a: Option<LottieAnimatable<Vec<f64>>>,
    pub o: Option<LottieAnimatable<f64>>,
    pub p: Option<LottiePositionProperty>,
    pub r: Option<LottieAnimatable<f64>>,
    pub rz: Option<LottieAnimatable<f64>>,
    pub s: Option<LottieAnimatable<Vec<f64>>>,
    pub sa: Option<LottieAnimatable<f64>>,
    pub sk: Option<LottieAnimatable<f64>>,
    pub eo: Option<LottieAnimatable<f64>>,
    pub so: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieRepeaterShapeItemRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct LottieRepeaterShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub c: LottieAnimatable<f64>,
    pub m: Option<f64>,
    pub o: LottieAnimatable<f64>,
    pub tr: LottieRepeaterShapeItemRecord1,
}
impl PartialEq for LottieRepeaterShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:225 (sha256:87399e5a8fcfcd08ef2a6e4835bc855f9ea29135da426c957da9a152f1a4de96)
#[derive(Clone, Default)]
pub struct LottieMergePathShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub mm: f64,
}
impl PartialEq for LottieMergePathShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:231 (sha256:57dab417d27f6c729c06afba587990db58f3099a0a90954758094263647bf49e)
#[derive(Clone)]
pub struct LottieRoundedCornersShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
    pub r: LottieAnimatable<f64>,
}
impl PartialEq for LottieRoundedCornersShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:236 (sha256:78cddb6e340653b2e0b5211ab714fd21f56b9d76a5fae41fd630470ffbd0998b)
#[derive(Clone, Default)]
pub struct LottieUnknownShapeItem {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hd: Option<bool>,
    pub ix: Option<f64>,
    pub nm: Option<String>,
    pub ty: String,
}
impl PartialEq for LottieUnknownShapeItem {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:240 (sha256:f6f03449ccfc2fdeb78b099586527a9553ee41832209bd62529c72fd8f2be97e)
pub type LottieShapeItem = crate::FlightUnion2<
    LottieEllipseShapeItem,
    crate::FlightUnion2<
        LottieFillShapeItem,
        crate::FlightUnion2<
            LottieGradientShapeItem,
            crate::FlightUnion2<
                LottieMergePathShapeItem,
                crate::FlightUnion2<
                    LottiePolystarShapeItem,
                    crate::FlightUnion2<
                        LottieRectangleShapeItem,
                        crate::FlightUnion2<
                            LottieRepeaterShapeItem,
                            crate::FlightUnion2<
                                LottieRoundedCornersShapeItem,
                                crate::FlightUnion2<
                                    LottieShapeGroup,
                                    crate::FlightUnion2<
                                        LottieShapePathItem,
                                        crate::FlightUnion2<
                                            LottieStrokeShapeItem,
                                            crate::FlightUnion2<
                                                LottieTransformShapeItem,
                                                crate::FlightUnion2<
                                                    LottieTrimPathShapeItem,
                                                    LottieUnknownShapeItem,
                                                >,
                                            >,
                                        >,
                                    >,
                                >,
                            >,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

// Source: upstream/packages/types/src/LottieDocument.ts:256 (sha256:a76719682a1f78059a61eca5708465e29b73e008f453a34756f735e830fb20d5)
#[derive(Clone)]
pub struct LottieMask {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub inv: Option<bool>,
    pub mode: String,
    pub nm: Option<String>,
    pub o: LottieAnimatable<f64>,
    pub pt: LottieAnimatable<LottieShapePath>,
    pub f: Option<LottieAnimatable<Vec<f64>>>,
    pub x: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieMask {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:268 (sha256:6fdb87c8296e70368f5ddd5a6699e7b6e050cb5e2d794d4382bd58fc5e83d4e2)
#[derive(Clone, Default)]
pub struct LottieTextDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fc: Option<Vec<f64>>,
    pub f: Option<String>,
    pub s: Option<f64>,
    pub sc: Option<Vec<f64>>,
    pub sw: Option<f64>,
    pub t: String,
    pub tr: Option<f64>,
    pub lh: Option<f64>,
    pub j: Option<f64>,
}
impl PartialEq for LottieTextDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:287 (sha256:9056f2886f78eb157be430599677f91037327b338e6abdb39fcdbf3c5576dc60)
#[derive(Clone, Default)]
pub struct LottieTextDataRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub k: Vec<LottieKeyframe<LottieTextDocument>>,
}
impl PartialEq for LottieTextDataRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LottieTextData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub d: LottieTextDataRecord1,
    pub a: Option<Vec<crate::OpaqueHostValue>>,
    pub m: Option<crate::OpaqueHostValue>,
    pub p: Option<crate::OpaqueHostValue>,
}
impl PartialEq for LottieTextData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:297 (sha256:f63fdcc269c7e158b73ba0ef5bb63dbba273608f3586cfcc534e986b9c62e6c7)
#[derive(Clone, Default)]
pub struct LottieEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ef: Option<Vec<LottieEffect>>,
    pub ix: Option<f64>,
    pub mn: Option<String>,
    pub nm: Option<String>,
    pub ty: Option<f64>,
    pub v: Option<LottieAnimatable<crate::FlightUnion2<f64, Vec<f64>>>>,
}
impl PartialEq for LottieEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:306 (sha256:ed2e39801a2b33fe8f903c119ababfaab77850d0f8749741e2e3a9a1a71ea5c2)
#[derive(Clone, Default)]
pub struct LottieLayer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bm: Option<f64>,
    pub ddd: Option<f64>,
    pub ef: Option<Vec<LottieEffect>>,
    pub ind: Option<f64>,
    pub parent: Option<f64>,
    pub ip: Option<f64>,
    pub op: Option<f64>,
    pub st: Option<f64>,
    pub sr: Option<f64>,
    pub ks: Option<LottieTransform>,
    pub masks_properties: Option<Vec<LottieMask>>,
    pub nm: Option<String>,
    pub ref_id: Option<String>,
    pub shapes: Option<Vec<LottieShapeItem>>,
    pub sc: Option<String>,
    pub sh: Option<f64>,
    pub sw: Option<f64>,
    pub t: Option<LottieTextData>,
    pub tt: Option<f64>,
    pub td: Option<f64>,
    pub ty: f64,
    pub tm: Option<LottieAnimatable<f64>>,
}
impl PartialEq for LottieLayer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:340 (sha256:30634eb0a7a59537f03ae3b33b6e6e8abda530c73ce511e2706162d48b9b3767)
#[derive(Clone, Default)]
pub struct LottieImageAsset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub e: Option<f64>,
    pub h: Option<f64>,
    pub id: String,
    pub p: String,
    pub u: Option<String>,
    pub w: Option<f64>,
}
impl PartialEq for LottieImageAsset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:351 (sha256:f272aaec54f3366e5bd2bc97284d40cd23c4617f89a714cdc5096c25412f3778)
#[derive(Clone, Default)]
pub struct LottiePrecompositionAsset {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub h: Option<f64>,
    pub id: String,
    pub layers: Vec<LottieLayer>,
    pub nm: Option<String>,
    pub w: Option<f64>,
}
impl PartialEq for LottiePrecompositionAsset {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:359 (sha256:7f7386f17ba55e90cc580d6a0742d7d515852231e3b6c6d1a3cb3b8e506ff355)
pub type LottieAsset = crate::FlightUnion2<LottieImageAsset, LottiePrecompositionAsset>;

// Source: upstream/packages/types/src/LottieDocument.ts:361 (sha256:f463045d641e1351fd59ee42ade43d7dd883c6c1db13f2a3f993fb060d4d125b)
#[derive(Clone, Default)]
pub struct LottieFont {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub f_family: Option<String>,
    pub f_name: String,
    pub f_style: Option<String>,
    pub ascent: Option<f64>,
}
impl PartialEq for LottieFont {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:368 (sha256:c29ff588cd6602d0879a65babfe9532e3f48ce003add4790ad105c4a758956b2)
#[derive(Clone, Default)]
pub struct LottieMarker {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cm: String,
    pub dr: f64,
    pub tm: f64,
}
impl PartialEq for LottieMarker {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/LottieDocument.ts:375 (sha256:bc1bd8fee72d0e49ff3cc90a7cac976377ee624b49519bc78226652352e72d31)
#[derive(Clone, Default)]
pub struct LottieDocumentRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub list: Vec<LottieFont>,
}
impl PartialEq for LottieDocumentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct LottieDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub assets: Option<Vec<LottieAsset>>,
    pub chars: Option<Vec<crate::OpaqueHostValue>>,
    pub ddd: Option<f64>,
    pub fonts: Option<LottieDocumentRecord1>,
    pub fr: f64,
    pub ip: f64,
    pub op: f64,
    pub h: f64,
    pub layers: Vec<LottieLayer>,
    pub markers: Option<Vec<LottieMarker>>,
    pub nm: Option<String>,
    pub v: Option<String>,
    pub w: f64,
}
impl PartialEq for LottieDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

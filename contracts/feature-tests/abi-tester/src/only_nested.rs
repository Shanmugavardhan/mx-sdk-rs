multiversx_sc::derive_imports!();

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct ConstructorArg {
    pub something: (),
}

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedUnit;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedEmptyArray {
    pub something: [u8; 0],
}

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedMultiResult();

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedVarArgs;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedMultiValueVec;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedOptionalArg;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedOptionalResult;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedEnumTuple;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedEnumField;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct NestedEsdtAttribute;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct MapperItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct VecItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct ArrayVecItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct ArrayItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct BoxItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct BoxedSliceItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct RefItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct SliceItem;

/// Tests that the ABI generator also fetches types that only appear as fields.
#[type_abi]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode)]
pub struct OptionItem;

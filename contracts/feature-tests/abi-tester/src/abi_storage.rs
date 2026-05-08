use multiversx_sc::imports::*;
use crate::abi_test_type::{AbiManagedVecItem, AbiManagedComplexVecItem};
use crate::only_nested::{
    MapperItem, VecItem, ArrayVecItem, ArrayItem, BoxItem, BoxedSliceItem, RefItem, SliceItem, OptionItem
};

#[multiversx_sc::module]
pub trait AbiStorageModule {
    #[view]
    #[storage_mapper("sample_storage_mapper")]
    fn sample_storage_mapper(&self) -> SingleValueMapper<MapperItem>;

    #[view]
    fn item_for_vec(&self) -> Vec<VecItem> {
        Vec::new()
    }

    #[view]
    fn item_for_array_vec(&self) -> ArrayVec<ArrayVecItem, 3> {
        ArrayVec::new()
    }

    #[view]
    fn item_for_managed_vec(&self) -> ManagedVec<AbiManagedVecItem> {
        ManagedVec::new()
    }

    #[view]
    fn item_for_managed_complex_vec(&self) -> ManagedVec<AbiManagedComplexVecItem<Self::Api>> {
        let mut result = ManagedVec::new();
        result.push(AbiManagedComplexVecItem {
            token_id: ManagedBuffer::from(b"CARBON-ab12cd"),
            holder: ManagedAddress::zero(),
            version: 7,
            body: ManagedBuffer::from(b"{\"ok\":true}"),
        });
        result
    }

    #[view]
    fn item_for_array(&self, _array: &[ArrayItem; 5]) {}

    #[view]
    fn item_for_box(&self) -> Box<BoxItem> {
        Box::new(BoxItem)
    }

    #[view]
    fn item_for_boxed_slice(&self) -> Box<[BoxedSliceItem]> {
        Vec::new().into_boxed_slice()
    }

    #[view]
    fn item_for_ref(&self, _ref: &RefItem) {}

    #[view]
    fn item_for_slice(&self, _ref: &[SliceItem]) {}

    #[view]
    fn item_for_option(&self) -> Option<OptionItem> {
        None
    }
}

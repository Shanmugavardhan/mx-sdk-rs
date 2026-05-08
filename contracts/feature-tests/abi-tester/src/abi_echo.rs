use multiversx_sc::imports::*;
use crate::abi_enum::AbiEnum;
use crate::abi_test_type::{AbiManagedType, AbiTestType, Permission};

#[multiversx_sc::module]
pub trait AbiEchoModule {
    /// Example endpoint docs.
    #[endpoint]
    #[output_name("single output")]
    #[output_name("this one doesn't show up")]
    fn echo_abi_test_type(&self, att: AbiTestType) -> AbiTestType {
        att
    }

    #[endpoint]
    #[only_owner]
    fn echo_enum(&self, e: AbiEnum) -> AbiEnum {
        e
    }

    #[endpoint]
    #[only_owner]
    fn take_managed_type(&self, _arg: AbiManagedType<Self::Api>) {}

    #[endpoint]
    fn address_vs_h256(&self, address: Address, h256: H256) -> MultiValue2<Address, H256> {
        self.address_h256_event(&address, &h256);
        (address, h256).into()
    }

    #[endpoint]
    fn managed_address_vs_byte_array(
        &self,
        address: ManagedAddress,
        byte_array: ManagedByteArray<Self::Api, 32>,
    ) -> MultiValue2<ManagedAddress, ManagedByteArray<Self::Api, 32>> {
        (address, byte_array).into()
    }

    #[endpoint]
    fn process_managed_decimal(
        &self,
        input: ManagedDecimal<Self::Api, ConstDecimals<U10>>,
    ) -> ManagedDecimal<Self::Api, usize> {
        input.into()
    }

    #[view]
    fn echo_permission(&self, p: Permission) -> Permission {
        p
    }

    #[event("address-h256-event")]
    fn address_h256_event(&self, #[indexed] address: &Address, #[indexed] h256: &H256);
}

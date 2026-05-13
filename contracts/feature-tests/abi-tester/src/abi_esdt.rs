use multiversx_sc::imports::*;

#[multiversx_sc::module]
#[esdt_attribute("TICKER1", BigUint)]
#[esdt_attribute("TICKER2", ManagedBuffer)]
#[esdt_attribute("TICKER3", u32)]
#[esdt_attribute("STRUCT1", crate::abi_enum::AbiEnum)]
#[esdt_attribute("STRUCT2", crate::abi_test_type::AbiManagedType<Self::Api>)]
#[esdt_attribute("OnlyInEsdt", crate::abi_test_type::EsdtAttribute)]
#[esdt_attribute("ExplicitDiscriminant", crate::abi_enum::ExplicitDiscriminant)]
#[esdt_attribute("ExplicitDiscriminantMixed", crate::abi_enum::ExplicitDiscriminantMixed)]
#[esdt_attribute("ManagedDecimalVar", ManagedDecimal<Self::Api, NumDecimals>)]
#[esdt_attribute("ManagedDecimalConst", crate::abi_test_type::ManagedDecimalWrapper<Self::Api>)]
pub trait AbiEsdtModule {
    #[endpoint]
    fn esdt_local_role(&self) -> EsdtLocalRole {
        EsdtLocalRole::None
    }

    #[endpoint]
    fn esdt_token_payment(&self) -> EsdtTokenPayment<Self::Api> {
        EsdtTokenPayment::new(
            EsdtTokenIdentifier::from(ManagedBuffer::from(b"TOKEN-000000")),
            0,
            BigUint::zero(),
        )
    }

    #[endpoint]
    fn esdt_token_data(&self) -> EsdtTokenData<Self::Api> {
        self.blockchain().get_esdt_token_data(
            &ManagedAddress::zero(),
            &EsdtTokenIdentifier::from(ManagedBuffer::new()),
            0,
        )
    }

    #[endpoint]
    #[payable("EGLD")]
    fn payable_egld(&self) {}

    #[endpoint]
    #[payable("TOKEN-FOR-ABI")]
    fn payable_some_token(&self) {
        let (token, payment) = self.call_value().single_fungible_esdt();
        self.payable_event(&token, &payment);
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_token(&self) {}

    #[event("payable-event")]
    fn payable_event(&self, #[indexed] token: &EsdtTokenIdentifier, amount: &BigUint);

    #[event]
    fn empty_identifier_event(&self);
}

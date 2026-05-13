#![no_std]

use multiversx_sc::imports::*;

mod abi_echo;
mod abi_enum;
pub mod abi_proxy;
mod abi_storage;
mod abi_multi_value;
mod abi_esdt;
mod abi_external;
mod abi_test_type;
mod only_nested;

pub use abi_test_type::{
    AbiEnvelope, AbiEnvelopeDomain, AbiManagedComplexVecItem, AbiManagedVecItem
};
pub use only_nested::ConstructorArg;

/// Contract whose sole purpose is to verify that
/// the ABI generation framework works as expected.
///
/// Note: any change in this contract must also be reflected in `abi_test_expected.abi.json`,
/// including Rust docs.
#[multiversx_sc::contract]
pub trait AbiTester:
    abi_echo::AbiEchoModule
    + abi_multi_value::AbiMultiValueModule
    + abi_storage::AbiStorageModule
    + abi_esdt::AbiEsdtModule
    + abi_external::AbiExternalModule
{
    /// Contract constructor.
    #[init]
    #[payable("EGLD")]
    fn init(&self, _constructor_arg_1: i32, _constructor_arg_2: ConstructorArg) {}

    /// Upgrade constructor.
    #[upgrade]
    fn upgrade(&self, _constructor_arg_1: i32, _constructor_arg_2: ConstructorArg) {
        self.init(_constructor_arg_1, _constructor_arg_2)
    }
}

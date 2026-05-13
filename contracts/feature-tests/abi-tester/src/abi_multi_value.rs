use multiversx_sc::imports::*;
use crate::only_nested::{NestedMultiResult, NestedVarArgs, NestedMultiValueVec, NestedOptionalArg, NestedOptionalResult};

#[multiversx_sc::module]
pub trait AbiMultiValueModule {
    #[endpoint]
    #[title("result-3")]
    #[output_name("multi-result-1")]
    #[output_name("multi-result-2")]
    #[output_name("multi-result-3")]
    #[output_name("multi-result-in-excess")]
    fn multi_result_3(&self) -> MultiValue3<i32, [u8; 3], BoxedBytes> {
        (1, [2; 3], BoxedBytes::empty()).into()
    }

    #[endpoint]
    #[output_name("multi-too-few-1")]
    #[output_name("multi-too-few-2")]
    fn multi_result_4(&self) -> MultiValue4<i32, [u8; 3], BoxedBytes, NestedMultiResult> {
        (1, [2; 3], BoxedBytes::empty(), NestedMultiResult()).into()
    }

    #[endpoint]
    fn var_args(
        &self,
        _simple_arg: u32,
        _var_args: MultiValueVec<MultiValue2<NestedVarArgs, i32>>,
    ) {
    }

    #[endpoint]
    fn multi_result_vec(&self) -> MultiValueVec<MultiValue3<NestedMultiValueVec, bool, ()>> {
        MultiValueVec::new()
    }

    #[endpoint]
    fn optional_arg(&self, _simple_arg: u32, _opt_args: OptionalValue<NestedOptionalArg>) {}

    #[endpoint]
    fn optional_result(&self) -> OptionalValue<NestedOptionalResult> {
        OptionalValue::None
    }
}

use multiversx_sc_scenario::imports::*;

use abi_tester::{
    abi_proxy::AbiTesterProxy, AbiEnvelope, AbiEnvelopeDomain, AbiManagedComplexVecItem,
    AbiManagedVecItem, ConstructorArg,
};

const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/abi-tester.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ABI_ADDRESS: TestSCAddress = TestSCAddress::new("abi-tester");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/abi-tester");
    blockchain.register_contract(CODE_PATH, abi_tester::ContractBuilder);
    blockchain
}

fn deploy(world: &mut ScenarioWorld) {
    world.account(OWNER_ADDRESS).nonce(1).balance(100);
    world.new_address(OWNER_ADDRESS, 1, ABI_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(ABI_ADDRESS)
        .argument(&0i32)
        .argument(&ConstructorArg { something: () })
        .run();
}

#[test]
fn abi_tester_experimental_managed_vec_return_smoke() {
    let mut world = world();
    deploy(&mut world);

    let value: ManagedVec<StaticApi, AbiManagedVecItem> = world
        .query()
        .to(ABI_ADDRESS)
        .typed(AbiTesterProxy)
        .item_for_managed_vec()
        .returns(ReturnsResult)
        .run();

    assert!(value.is_empty());
}

#[test]
fn abi_tester_complex_vec_return_check() {
    let mut world = world();
    deploy(&mut world);

    let value: ManagedVec<StaticApi, AbiManagedComplexVecItem<StaticApi>> = world
        .query()
        .to(ABI_ADDRESS)
        .typed(AbiTesterProxy)
        .item_for_managed_complex_vec()
        .returns(ReturnsResult)
        .run();

    assert_eq!(value.len(), 1);
    let item = value.get(0);
    assert_eq!(item.token_id, ManagedBuffer::from(b"CARBON-ab12cd"));
    assert_eq!(item.version, 7);
}

#[test]
fn abi_tester_envelope_check() {
    let mut world = world();
    deploy(&mut world);

    let value: AbiEnvelope<StaticApi> = world
        .query()
        .to(ABI_ADDRESS)
        .typed(AbiTesterProxy)
        .envelope_like_result()
        .returns(ReturnsResult)
        .run();

    assert_eq!(value.domain, AbiEnvelopeDomain::Alpha);
    assert_eq!(value.payload_hash, ManagedBuffer::from(&[9u8; 32]));
    assert_eq!(value.operations.len(), 1);
}

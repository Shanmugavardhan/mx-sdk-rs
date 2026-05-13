use multiversx_sc_scenario::imports::*;

use scenario_tester::scenario_tester_proxy;

const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/scenario-tester.mxsc.json");
const OWNER_ADDRESS: TestAddress = TestAddress::new("owner");
const ST_ADDRESS: TestSCAddress = TestSCAddress::new("scenario-tester");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new()
        .executor_config(ExecutorConfig::compiled_tests_or(ExecutorConfig::default()));

    blockchain.set_current_dir_from_workspace("contracts/feature-tests/scenario-tester");
    
    // Pass the MxscPath object directly to register_contract
    blockchain.register_contract(
        CODE_PATH,
        scenario_tester::ContractBuilder,
    );
    blockchain
}

#[test]
fn st_experimental_multi_return_smoke() {
    let mut world = world();

    world.account(OWNER_ADDRESS).nonce(1).balance(100);
    world.new_address(OWNER_ADDRESS, 1, ST_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .init(5u32)
        .code(CODE_PATH) // Use the same MxscPath object
        .new_address(ST_ADDRESS)
        .run();

    let value = world
        .tx()
        .from(OWNER_ADDRESS)
        .to(ST_ADDRESS)
        .typed(scenario_tester_proxy::ScenarioTesterProxy)
        .multi_return(1u32)
        .returns(ReturnsResultUnmanaged)
        .run();

    assert_eq!(
        value,
        MultiValue2((RustBigUint::from(1u32), RustBigUint::from(2u32)))
    );
}

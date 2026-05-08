use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/abi-tester");

    blockchain.register_contract(
        "mxsc:output/abi-tester.mxsc.json",
        abi_tester::ContractBuilder,
    );

    blockchain
}

#[test]
fn abi_tester_experimental_rs() {
    world().run("scenarios/abi_tester_experimental.scen.json");
}

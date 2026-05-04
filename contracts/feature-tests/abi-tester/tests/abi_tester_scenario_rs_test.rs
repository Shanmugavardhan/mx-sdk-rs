use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    // Setting current dir to the scenarios folder to align relative paths with the JSON files
    blockchain.set_current_dir_from_workspace("contracts/feature-tests/abi-tester/scenarios");
    
    blockchain.register_contract(
        "mxsc:../output/abi-tester.mxsc.json",
        abi_tester::ContractBuilder,
    );

    blockchain
}

#[test]
fn abi_tester_experimental_rs() {
    world().run("abi_tester_experimental.scen.json");
}

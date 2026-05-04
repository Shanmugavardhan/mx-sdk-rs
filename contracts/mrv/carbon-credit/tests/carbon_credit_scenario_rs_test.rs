use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    // Setting current dir to the scenarios folder to align relative paths with the JSON files
    blockchain.set_current_dir_from_workspace("contracts/mrv/carbon-credit/scenarios");
    
    blockchain.register_contract(
        "mxsc:../output/mrv-carbon-credit.mxsc.json",
        mrv_carbon_credit::ContractBuilder,
    );
    blockchain.register_contract(
        "mxsc:../governance/output/mrv-governance.mxsc.json",
        mrv_governance::ContractBuilder,
    );

    blockchain
}

#[test]
fn carbon_credit_init_rs() {
    world().run("carbon-credit-init.scen.json");
}

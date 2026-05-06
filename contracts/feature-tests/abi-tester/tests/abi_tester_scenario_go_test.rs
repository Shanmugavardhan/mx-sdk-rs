use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn abi_tester_experimental_go() {
    world().run("scenarios/abi_tester_experimental.scen.json");
}

use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::vm_go();
    world.set_current_dir_from_workspace("contracts/feature-tests/abi-tester");
    world
}

#[test]
fn abi_tester_experimental_go() {
    world().run("scenarios/abi_tester_experimental.scen.json");
}

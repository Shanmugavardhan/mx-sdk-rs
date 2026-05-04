use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::vm_go();
    world.set_current_dir_from_workspace("contracts/mrv/carbon-credit");
    world
}

#[test]
fn carbon_credit_init_go() {
    world().run("scenarios/carbon-credit-init.scen.json");
}

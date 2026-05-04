use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::vm_go();
    world.set_current_dir_from_workspace("contracts/mrv/reserve-proof-registry");
    world
}

#[test]
fn reserve_proof_lifecycle_go() {
    world().run("scenarios/reserve-proof-lifecycle.scen.json");
}

#[test]
fn reserve_proof_integration_go() {
    world().run("scenarios/reserve-proof-integration.scen.json");
}

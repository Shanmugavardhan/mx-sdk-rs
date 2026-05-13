use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn reserve_proof_lifecycle_go() {
    world().run("scenarios/reserve-proof-lifecycle.scen.json");
}

#[test]
fn reserve_proof_integration_go() {
    world().run("scenarios/reserve-proof-integration.scen.json");
}

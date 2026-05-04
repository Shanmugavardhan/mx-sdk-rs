use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::vm_go();
    world.set_current_dir_from_workspace("contracts/drwa/asset-manager");
    world
}

#[test]
fn asset_manager_init_go() {
    world().run("scenarios/asset-manager-init.scen.json");
}

#[test]
fn asset_manager_denial_signals_go() {
    world().run("scenarios/asset-manager-denial-signals.scen.json");
}

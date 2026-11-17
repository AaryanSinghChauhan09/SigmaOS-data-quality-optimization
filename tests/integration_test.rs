//! Integration tests for SigmaOS-data-quality-optimization.

use sigmaos_data_quality_optimization_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}

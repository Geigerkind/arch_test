use velcro::hash_set;

use crate::{Architecture, ModuleTree};
use crate::analyzer::domain_values::access_rules::{NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess};

#[test]
fn no_parent_access() {
    let architecture = Architecture::new(hash_set![])
        .with_access_rule(NoParentAccess);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_parent_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn no_module_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![])
        .with_access_rule(NoModuleCyclicDependencies);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_module_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn no_layer_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![])
        .with_access_rule(NoLayerCyclicDependencies);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_layer_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}
use velcro::hash_set;

use crate::{Architecture, ModuleTree};
use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess, NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess, MayOnlyBeAccessedBy};

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

#[test]
fn may_only_access_positive() {
    let layer_names = hash_set!["file_1".to_owned(), "file_2".to_owned()];
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayOnlyAccess::new(&layer_names, "file_1".to_owned(), hash_set!["file_2".to_owned()], false));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_access_negative() {
    let layer_names = hash_set!["file_1".to_owned(), "file_2".to_owned()];
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayOnlyAccess::new(&layer_names, "file_1".to_owned(), hash_set![], false));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_not_access() {
    let layer_names = hash_set!["file_1".to_owned(), "file_2".to_owned()];
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayNotAccess::new(&layer_names, "file_1".to_owned(), hash_set!["file_2".to_owned()], false));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_only_be_accessed_by() {
    let layer_names = hash_set!["file_1".to_owned(), "file_2".to_owned(), "file_3".to_owned()];
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayOnlyBeAccessedBy::new(&layer_names, "file_2".to_owned(), hash_set!["file_1".to_owned()], false));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn myself() {
    let layer_names = hash_set!["analyzer".to_owned(), "parser".to_owned(), "domain_values".to_owned(), "entities".to_owned(), "materials".to_owned(), "services".to_owned(), "tests".to_owned(), "utils".to_owned()];
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(NoParentAccess)
        .with_access_rule(NoModuleCyclicDependencies)
        .with_access_rule(NoLayerCyclicDependencies)
        .with_access_rule(MayNotAccess::new(&layer_names, "parser".to_owned(), hash_set!["analyzer".to_owned()], true))
        .with_access_rule(MayOnlyAccess::new(&layer_names, "analyzer".to_owned(), hash_set!["analyzer".to_owned(), "parser".to_owned()], true))
        .with_access_rule(MayOnlyAccess::new(&layer_names, "domain_values".to_owned(), hash_set!["domain_values".to_owned(), "utils".to_owned()], false))
        .with_access_rule(MayOnlyAccess::new(&layer_names, "entities".to_owned(), hash_set!["entities".to_owned(), "domain_values".to_owned()], false))
        .with_access_rule(MayOnlyAccess::new(&layer_names, "utils".to_owned(), hash_set!["utils".to_owned()], true))
        .with_access_rule(MayNotAccess::new(&layer_names, "services".to_owned(), hash_set!["materials".to_owned()], true))
        .with_access_rule(MayNotAccess::new(&layer_names, "materials".to_owned(), hash_set!["tests".to_owned()], true));
    let module_tree = ModuleTree::new("src/lib.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}
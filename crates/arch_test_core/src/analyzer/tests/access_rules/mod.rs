use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{
    MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess,
};
use crate::{Architecture, ModuleTree};

#[test]
fn no_parent_access() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoParentAccess);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_parent_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn no_module_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoModuleCyclicDependencies);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/no_module_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn no_layer_cyclic_dependencies() {
    let architecture = Architecture::new(hash_set![]).with_access_rule(NoLayerCyclicDependencies);
    let module_tree =
        ModuleTree::new("src/analyzer/tests/access_rules/no_layer_cyclic_dependencies/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_only_access_positive() {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("file_1".to_owned(), hash_set!["file_2".to_owned()], false),
        );
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_access_negative() {
    let architecture = Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()])
        .with_access_rule(MayOnlyAccess::new("file_1".to_owned(), hash_set![], false));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_only_access_when_same_parent_positive() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("layer_1".to_owned(), hash_set![], true),
        );
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

#[test]
fn may_only_access_when_same_parent_negative() {
    let architecture =
        Architecture::new(hash_set!["layer_1".to_owned(), "layer_2".to_owned()]).with_access_rule(
            MayOnlyAccess::new("layer_1".to_owned(), hash_set![], false),
        );
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access_same_parent/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_not_access() {
    let architecture =
        Architecture::new(hash_set!["file_1".to_owned(), "file_2".to_owned()]).with_access_rule(
            MayNotAccess::new("file_1".to_owned(), hash_set!["file_2".to_owned()], false),
        );
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_only_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_access_rule(MayOnlyBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_1".to_owned()],
        false,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn may_not_be_accessed_by() {
    let architecture = Architecture::new(hash_set![
        "file_1".to_owned(),
        "file_2".to_owned(),
        "file_3".to_owned()
    ])
    .with_access_rule(MayNotBeAccessedBy::new(
        "file_2".to_owned(),
        hash_set!["file_3".to_owned()],
        false,
    ));
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/may_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}

#[test]
fn myself() {
    let architecture = Architecture::new(hash_set![
        "analyzer".to_owned(),
        "parser".to_owned(),
        "domain_values".to_owned(),
        "entities".to_owned(),
        "materials".to_owned(),
        "services".to_owned(),
        "tests".to_owned(),
        "utils".to_owned()
    ])
    .with_access_rule(NoParentAccess)
    .with_access_rule(NoModuleCyclicDependencies)
    .with_access_rule(NoLayerCyclicDependencies)
    .with_access_rule(MayNotAccess::new(
        "parser".to_owned(),
        hash_set!["analyzer".to_owned()],
        true,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "analyzer".to_owned(),
        hash_set!["analyzer".to_owned(), "parser".to_owned()],
        true,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "domain_values".to_owned(),
        hash_set!["domain_values".to_owned(), "utils".to_owned()],
        false,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "entities".to_owned(),
        hash_set!["entities".to_owned(), "domain_values".to_owned()],
        false,
    ))
    .with_access_rule(MayOnlyAccess::new(
        "utils".to_owned(),
        hash_set!["utils".to_owned()],
        true,
    ))
    .with_access_rule(MayNotAccess::new(
        "services".to_owned(),
        hash_set!["materials".to_owned()],
        true,
    ))
    .with_access_rule(MayNotAccess::new(
        "materials".to_owned(),
        hash_set!["tests".to_owned()],
        true,
    ));
    let module_tree = ModuleTree::new("src/lib.rs");
    assert!(architecture.validate_access_rules().is_ok());
    assert!(architecture.check_access_rules(&module_tree).is_ok());
}

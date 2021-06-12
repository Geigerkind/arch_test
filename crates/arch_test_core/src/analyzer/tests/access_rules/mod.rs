use crate::{Architecture, ModuleTree};
use velcro::hash_set;
use crate::analyzer::domain_values::access_rules::NoParentAccess;

#[test]
fn no_parent_access() {
    let architecture = Architecture::new(hash_set!["crate".to_owned(), "child".to_owned()])
        .with_access_rule(NoParentAccess);
    let module_tree = ModuleTree::new("src/analyzer/tests/access_rules/no_parent_access/main.rs");
    assert!(architecture.check_access_rules(&module_tree).is_err());
}
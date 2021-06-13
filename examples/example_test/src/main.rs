#[cfg(test)]
mod test {
    extern crate arch_test_core;

    use arch_test_core::access_rules::{
        MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
        NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess,
    };
    use arch_test_core::{hash_set, Architecture, ModuleTree};

    #[test]
    fn test_architecture() {
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
        .with_access_rule(MayOnlyBeAccessedBy::new(
            "materials".to_owned(),
            hash_set!["tests".to_owned()],
            true,
        ))
        .with_access_rule(MayNotBeAccessedBy::new(
            "tests".to_owned(),
            hash_set!["materials".to_owned()],
            true,
        ));
        let module_tree = ModuleTree::new("../../crates/arch_test_core/src/lib.rs");
        assert!(architecture.validate_access_rules().is_ok());
        assert!(architecture.check_access_rules(&module_tree).is_ok());
    }
}

fn main() {}

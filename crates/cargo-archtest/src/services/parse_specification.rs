use std::fs::File;
use std::io::Read;
use std::path::Path;

use arch_test_core::access_rules::{
    MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy,
    NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess,
};
use arch_test_core::hash_set;
use arch_test_core::Architecture;

use crate::domain_values::{AccessRule, Failure, Specification};

pub fn parse_specification(specification_path: &Path) -> Result<Architecture, Failure> {
    let specification: Specification =
        serde_json::from_str(&read_file_content(specification_path)?)
            .map_err(|_| Failure::SpecificationCouldNotBeParsed)?;

    let mut architecture = Architecture::new(hash_set![..specification.clone().layer_names]);
    for access_rule in specification.access_rules {
        match access_rule {
            AccessRule::NoLayerCyclicDependencies => {
                architecture = architecture.with_access_rule(NoLayerCyclicDependencies)
            }
            AccessRule::NoModuleCyclicDependencies => {
                architecture = architecture.with_access_rule(NoModuleCyclicDependencies)
            }
            AccessRule::NoParentAccess => {
                architecture = architecture.with_access_rule(NoParentAccess)
            }
            AccessRule::MayOnlyAccess {
                accessor,
                accessed,
                when_same_parent,
            } => {
                architecture = architecture.with_access_rule(MayOnlyAccess::new(
                    accessor,
                    hash_set![..accessed],
                    when_same_parent,
                ))
            }
            AccessRule::MayNotAccess {
                accessor,
                accessed,
                when_same_parent,
            } => {
                architecture = architecture.with_access_rule(MayNotAccess::new(
                    accessor,
                    hash_set![..accessed],
                    when_same_parent,
                ))
            }
            AccessRule::MayOnlyBeAccessedBy {
                accessors,
                accessed,
                when_same_parent,
            } => {
                architecture = architecture.with_access_rule(MayOnlyBeAccessedBy::new(
                    accessed,
                    hash_set![..accessors],
                    when_same_parent,
                ))
            }
            AccessRule::MayNotBeAccessedBy {
                accessors,
                accessed,
                when_same_parent,
            } => {
                architecture = architecture.with_access_rule(MayNotBeAccessedBy::new(
                    accessed,
                    hash_set![..accessors],
                    when_same_parent,
                ))
            }
        }
    }
    Ok(architecture)
}

fn read_file_content(file_path: &Path) -> Result<String, Failure> {
    let mut file = File::open(file_path).map_err(|_| Failure::SpecificationFileCantBeOpened)?;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    Ok(content)
}

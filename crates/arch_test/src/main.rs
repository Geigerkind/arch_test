#[macro_use]
extern crate structopt;

use std::collections::HashSet;

use structopt::StructOpt;

use arch_test_core::{Architecture, ModuleTree};
use arch_test_core::access_rules::{MayOnlyAccess, NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess};
use arch_test_core::hash_set;

use crate::domain_values::Options;

mod domain_values;

fn main() {
    let opts = Options::from_args();


    /*
    let module_tree = ModuleTree::new("/home/shino/hacking/cyclic_dep_test/src/main.rs");
    let layer_names: HashSet<String> = hash_set!["dir_a", "dir_b", "dir_c", "dir_d"]
        .iter().map(|elem| elem.to_string()).collect();
    let architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayOnlyAccess::new(&layer_names, "dir_c", hash_set!["dir_c", "dir_d", "dir_b", "dir_a"]))
        //.with_access_rule(MayNotAccess::new(&layer_names, "dir_c", hash_set!["dir_d"]))
        .with_access_rule(NoParentAccess)
        .with_access_rule(NoModuleCyclicDependencies)
        .with_access_rule(NoLayerCyclicDependencies);
    println!("Check Access Rules: {:?}", architecture.check_access_rules(&module_tree));
    println!("Check missing layers: {:?}", architecture.check_complete_layer_specification(&module_tree));
     */
}
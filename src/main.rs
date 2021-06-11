#![feature(option_result_contains)]
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate derive_new;
extern crate syntax;
extern crate velcro;

use std::collections::HashSet;

pub use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess, NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess};
use crate::analyzer::materials::Architecture;
use crate::parser::materials::ModuleTree;

mod analyzer;
mod parser;

fn main() {
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
}

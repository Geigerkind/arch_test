#![feature(option_result_contains)]
#[macro_use]
extern crate derive_getters;
#[macro_use]
extern crate derive_new;
extern crate velcro;
// TODO Republsh hash_set macro

use std::collections::HashSet;

use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess, NoParentAccess};
use crate::analyzer::materials::Architecture;
use crate::analyzer::services::{contains_cyclic_dependency, contains_cyclic_dependency_on_any_level, contains_cyclic_dependency_on_level};
use crate::parser::materials::ModuleTree;

mod analyzer;
mod parser;

fn main() {
    let module_tree = ModuleTree::new("/home/shino/hacking/cyclic_dep_test/src/").unwrap();
    let layer_names: HashSet<String> = hash_set!["dir_a", "dir_b", "dir_c", "dir_d"]
        .iter().map(|elem| elem.to_string()).collect();
    let mut architecture = Architecture::new(layer_names.clone())
        .with_access_rule(MayOnlyAccess::new(&layer_names, "dir_c", hash_set!["dir_c", "dir_d", "dir_b", "dir_a"]))
        //.with_access_rule(MayNotAccess::new(&layer_names, "dir_c", hash_set!["dir_d"]))
        .with_access_rule(NoParentAccess);
    println!("{:?}", module_tree);
    println!("Check Access Rules: {:?}", architecture.check_access_rules(&module_tree));
    println!("Per module: {:?}", contains_cyclic_dependency(&module_tree));
    println!("Per any level: {:?}", contains_cyclic_dependency_on_any_level(&module_tree));
    println!("Per level 1: {:?}", contains_cyclic_dependency_on_level(&module_tree, 1));
    println!("Per level 2: {:?}", contains_cyclic_dependency_on_level(&module_tree, 2));
}

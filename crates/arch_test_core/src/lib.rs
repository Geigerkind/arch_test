#![feature(option_result_contains)]
#[macro_use]
extern crate derive_getters;
extern crate syntax;
extern crate velcro;
extern crate itertools;

pub use velcro::hash_set;

pub use crate::analyzer::domain_values::access_rules;
pub use crate::analyzer::materials::Architecture;
pub use crate::parser::materials::ModuleTree;

mod analyzer;
mod parser;
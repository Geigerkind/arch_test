extern crate itertools;
extern crate ra_ap_syntax;
extern crate velcro;

pub use velcro::hash_set;

pub use crate::analyzer::domain_values::access_rules;
pub use crate::analyzer::materials::Architecture;
pub use crate::parser::materials::ModuleTree;

mod analyzer;
mod parser;

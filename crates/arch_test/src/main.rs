#[macro_use]
extern crate structopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use structopt::StructOpt;

use arch_test_core::ModuleTree;

use crate::domain_values::Options;
use crate::services::parse_specification;
use std::path::Path;

mod domain_values;
mod services;

fn main() {
    let opts: Options = Options::from_args();
    let specification = parse_specification(Path::new(&opts.specification));
    if let Ok(architecture) = specification {
        let module_tree = ModuleTree::new(opts.input.to_str().unwrap());
        println!("{:?}", architecture.check_access_rules(&module_tree));
        if opts.check_for_complete_layer_specification {
            println!("{:?}", architecture.check_complete_layer_specification(&module_tree));
        }
    }
}
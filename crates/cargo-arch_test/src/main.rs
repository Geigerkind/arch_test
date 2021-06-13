extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use std::io::Read;
use std::path::Path;

use structopt::StructOpt;

use arch_test_core::ModuleTree;

use crate::domain_values::Options;
use crate::services::parse_specification;

mod domain_values;
mod services;

fn main() {
    let opts: Options = Options::from_args();
    let specification = parse_specification(Path::new(&opts.specification));
    if let Ok(architecture) = specification {
        let module_tree = ModuleTree::new(opts.input.to_str().unwrap());
        if let Err(err) = architecture.validate_access_rules() {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if let Err(err) = architecture.check_access_rules(&module_tree) {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if opts.check_for_complete_layer_specification {
            if let Err(err) = architecture.check_complete_layer_specification(&module_tree) {
                err.print(&module_tree.tree());
                std::process::exit(1);
            }
        }
    } else {
        println!("Specification file cant be opened!");
        std::process::exit(1);
    }
}

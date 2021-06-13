extern crate cargo_toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use std::path::Path;

use structopt::StructOpt;

use arch_test_core::ModuleTree;

use crate::domain_values::Command;
use crate::services::parse_specification;

mod domain_values;
mod services;

fn check_architecture(directory_path: &str, check_for_complete_layer_specification: bool) {
    let main_path_str = format!("{}/src/main.rs", directory_path);
    let main_path = Path::new(&main_path_str);
    let root_path = if main_path.exists() && main_path.is_file() {
        format!("{}/src/main.rs", directory_path)
    } else {
        format!("{}/src/lib.rs", directory_path)
    };
    let specification_path = format!("{}/architecture.json", directory_path);
    let specification = parse_specification(Path::new(&specification_path));

    if let Ok(architecture) = specification {
        let module_tree = ModuleTree::new(&root_path);
        if let Err(err) = architecture.validate_access_rules() {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if let Err(err) = architecture.check_access_rules(&module_tree) {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if check_for_complete_layer_specification {
            if let Err(err) = architecture.check_complete_layer_specification(&module_tree) {
                err.print(&module_tree.tree());
                std::process::exit(1);
            }
        }
    } else {
        println!(
            "Specification file cant be opened for '{}'.",
            directory_path
        );
        std::process::exit(1);
    }
}

fn main() {
    let Command::Archtest {
        check_for_complete_layer_specification,
        toml_path,
    } = Command::from_args();
    let toml_path = Path::new(&toml_path);
    if toml_path.exists() && toml_path.is_file() {
        if let Ok(toml) = cargo_toml::Manifest::from_path(toml_path) {
            if let Some(workspace) = toml.workspace {
                for member in workspace.members {
                    if member.contains('*') {
                        println!("Can not interpret paths with '*'");
                        std::process::exit(1);
                    } else {
                        check_architecture(&member, check_for_complete_layer_specification);
                    }
                }
            } else {
                check_architecture(".", check_for_complete_layer_specification);
            }
        } else {
            println!("Cargo.toml could not be parsed!");
            std::process::exit(1);
        }
    } else {
        println!("Cargo.toml not found in the specified path!");
        std::process::exit(1);
    }

    println!("[Ok]: No architecture rules were violated!");
}

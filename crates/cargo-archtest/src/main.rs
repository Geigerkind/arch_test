//! # ArchTest
//! ArchTest is a rule based architecture testing tool. It applies static analyses on the specified rust project to extract use relationships.
//!
//! For a through documentation on how to use it for tests, please consult the [arch_test_core](https://docs.rs/arch_test_core/0.1.2/arch_test_core/) crate.
//!
//! ## Install
//! ```sh
//! cargo install cargo-archtest --force
//! ```
//!
//! ## How to use it
//! Define in the cargo root path a file called `architecture.json`. Fill it according to the `Specification` struct.
//!
//! Example
//! ```json
//! let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned(), ...])
//! {
//!   "layer_names": ["analyzer", "parser", "domain_values", "entities", "materials", "services", "tests", "utils"],
//!   "access_rules": [
//!     "NoLayerCyclicDependencies",
//!     "NoModuleCyclicDependencies",
//!     "NoParentAccess",
//!     {
//!       "MayNotAccess": {
//!         "accessor": "parser",
//!         "accessed": ["analyzer"],
//!         "when_same_parent": true
//!       }
//!     },
//!     {
//!       "MayOnlyBeAccessedBy": {
//!         "accessors": ["materials", "tests"],
//!         "accessed": "services",
//!         "when_same_parent": false
//!       }
//!     },
//!     {
//!       "MayNotBeAccessedBy": {
//!         "accessors": ["services", "domain_values", "entities", "utils"],
//!         "accessed": "materials",
//!         "when_same_parent": true
//!       }
//!     }
//!   ]
//! }
//! ```
//! Then execute `cargo archtest` in your project directory.
//!
//! ## Contiinues integration
//! You can use it in continues integration by using either methods. If you decide to use the Cargo sub command on GitHub, the following snippet will allow you to test your project.
//! ```yml
//! arch_test:
//!    name: ArchTest
//!    runs-on: ubuntu-latest
//!    steps:
//!      - uses: actions/checkout@v2
//!      - uses: actions-rs/install@v0.1
//!        with:
//!          crate: cargo-archtest
//!          version: latest
//!      - run: cargo archtest
//! ```

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

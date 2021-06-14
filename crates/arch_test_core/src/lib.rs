//! # ArchTest
//! ArchTest is a rule based architecture testing tool. It applies static analyses on the specified rust project to extract use relationships.
//!
//! ## Features
//! * Detect cyclic dependencies level wise or module wise
//! * Prohibit parent access
//! * Define layer relationships like `MayNotAccess`, `MayOnlyAccess`, `MyNotBeAccessedBy`, `MayOnlyBeAccessedBy` etc.
//! * For more access rules consult `access_rules`.
//!
//! ## Install
//! ```toml
//! [dev-dependencies]
//! arch_test_core = "*"
//! ```
//!
//! ## How to use it
//! You can use the `Architecture` struct in order to define your architecture. Afterwards you check it for failures.
//! ```rust
//! let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned(), ...])
//! .with_access_rule(NoParentAccess)
//! .with_access_rule(NoModuleCyclicDependencies)
//! .with_access_rule(NoLayerCyclicDependencies)
//! ...
//! .with_access_rule(MayNotAccess::new(
//!     "materials".to_owned(),
//!     hash_set!["tests".to_owned()],
//!     true,
//! ));
//! let module_tree = ModuleTree::new("src/lib.rs");
//! assert!(architecture.validate_access_rules().is_ok());
//! assert!(architecture.check_access_rules(&module_tree).is_ok());
//! ```
//! If you are interested in the failure you can pretty print it like this:
//! ```rust
//! architecture.check_access_rules(&module_tree).err().unwrap().print(module_tree.tree());
//! ```

extern crate itertools;
extern crate ra_ap_syntax;
extern crate velcro;

/// `has_set![...]` macro exposed from the `velcro` crate for utility
pub use velcro::hash_set;

pub use crate::analyzer::domain_values::access_rules;
pub use crate::analyzer::materials::Architecture;
pub use crate::parser::materials::ModuleTree;

mod analyzer;
mod parser;

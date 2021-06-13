# ArchTest
ArchTest is a rule based architecture testing tool. 
It applies static analyses on the specified rust project to extract use relationships.

## Features
* Detect cyclic dependencies level wise or module wise
* Prohibit parent access
* Define layer relationships like `MayNotAccess`, `MayOnlyAccess`, `MyNotBeAccessedBy`, `MayOnlyBeAccessedBy`
* And more, please consult the documentation.

## Install
You can install it either as subcommand of Cargo or as a package in your developer dependencies.
```
# Subcommand
cargo install cargo-archtest --force

# Package
[dev-dependencies]
arch_test_core = "*"
```

## How to use it
### Using the Cargo subcommand
Define in the cargo root path a file called `architecture.json`. Fill it according to the `Specification` struct.
Example:
```json
{
  "layer_names": ["analyzer", "parser", "domain_values", "entities", "materials", "services", "tests", "utils"],
  "access_rules": [
    "NoLayerCyclicDependencies",
    "NoModuleCyclicDependencies",
    "NoParentAccess",
    {
      "MayNotAccess": {
        "accessor": "parser",
        "accessed": ["analyzer"],
        "when_same_parent": true
      }
    },
    {
      "MayOnlyBeAccessedBy": {
        "accessors": ["materials", "tests"],
        "accessed": "services",
        "when_same_parent": false
      }
    },
    {
      "MayNotBeAccessedBy": {
        "accessors": ["services", "domain_values", "entities", "utils"],
        "accessed": "materials",
        "when_same_parent": true
      }
    }
  ]
}
```

### Using a rust test
You can use the `Architecture` struct in order to define your architecture.
Afterwards you check it for failures.
```rust
let architecture = Architecture::new(hash_set!["analyzer".to_owned(), "parser".to_owned(), ...])
.with_access_rule(NoParentAccess)
.with_access_rule(NoModuleCyclicDependencies)
.with_access_rule(NoLayerCyclicDependencies)
...
.with_access_rule(MayNotAccess::new(
    "materials".to_owned(),
    hash_set!["tests".to_owned()],
    true,
));
let module_tree = ModuleTree::new("src/lib.rs");
assert!(architecture.validate_access_rules().is_ok());
assert!(architecture.check_access_rules(&module_tree).is_ok());
```
If you are interested in the failure you can pretty print it like this:
```rust
architecture.check_access_rules(&module_tree).err().unwrap().print(module_tree.tree());
```
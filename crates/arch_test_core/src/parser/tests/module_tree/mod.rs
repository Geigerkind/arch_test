use std::collections::HashMap;

use crate::parser::domain_values::ObjectUse;
use crate::parser::entities::ModuleNode;
use crate::ModuleTree;

#[test]
fn correct_fully_qualified_names() {
    // This also tests "correct_republish_paths"
    let module_tree =
        ModuleTree::new("src/parser/tests/module_tree/correct_fully_qualified_names/main.rs");
    let tree: &Vec<ModuleNode> = module_tree.tree();

    assert_eq!(tree[0].usable_objects[0].object_name, "test".to_owned());
    assert_eq!(
        tree[0].usable_objects[1].object_name,
        "crate::republish::wambo::WAMBO".to_owned()
    );

    assert_eq!(
        tree[1].usable_objects[0].object_name,
        "crate::republish::wambo::WAMBO".to_owned()
    );
    assert_eq!(
        tree[1].usable_objects[1].object_name,
        "crate::republish::testo::TESTO".to_owned()
    );
    assert_eq!(tree[1].usable_objects[2].object_name, "test".to_owned());
    assert_eq!(
        tree[1].usable_objects[3].object_name,
        "crate::republish::wambo::WAMBO".to_owned()
    );
    assert_eq!(
        tree[1].usable_objects[4].object_name,
        "crate::republish::wambo::WAMBO".to_owned()
    );
    assert_eq!(
        tree[1].usable_objects[5].object_name,
        "crate::republish::testo::TESTO".to_owned()
    );
    assert_eq!(
        tree[1].usable_objects[6].object_name,
        "crate::republish::testo::TESTO".to_owned()
    );

    assert_eq!(tree[2].usable_objects[0].object_name, "WAMBO".to_owned());
    assert_eq!(tree[3].usable_objects[0].object_name, "TESTO".to_owned());
}

#[test]
fn construct_possible_use_map() {
    let module_tree =
        ModuleTree::new("src/parser/tests/module_tree/correct_fully_qualified_names/main.rs");
    let use_map: &HashMap<String, ObjectUse> = module_tree.possible_uses();

    assert_eq!(use_map.len(), 4);
    assert!(use_map.contains_key("crate::republish::testo::TESTO"));
    assert!(use_map.contains_key("crate::republish::wambo::WAMBO"));
    assert!(use_map.contains_key("crate::test"));
    assert!(use_map.contains_key("crate::republish::test"));
}

#[test]
fn filter_primary_types() {
    let module_tree = ModuleTree::new("src/parser/tests/module_tree/filter_primary_types/main.rs");

    let tree = module_tree.tree();
    assert_eq!(tree[0].usable_objects.len(), 1);
    assert_eq!(tree[0].usable_objects[0].object_name, "main".to_owned());
}

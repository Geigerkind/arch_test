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

#[test]
fn path_wildcard() {
    let module_tree = ModuleTree::new("src/parser/tests/module_tree/path_wildcard/main.rs");

    let tree = module_tree.tree();
    assert_eq!(tree[0].usable_objects.len(), 2);
    assert_eq!(tree[0].usable_objects[0].object_name, "main".to_owned());
    assert_eq!(
        tree[0].usable_objects[1].object_name,
        "crate::ext_file::Test1".to_owned()
    );

    assert_eq!(tree[1].usable_objects.len(), 2);
    assert_eq!(tree[1].usable_objects[0].object_name, "Test1".to_owned());
    assert_eq!(tree[1].usable_objects[1].object_name, "Test2".to_owned());
}

#[test]
fn non_main_or_lib_root() {
    let module_tree =
        ModuleTree::new("src/parser/tests/module_tree/non_main_or_lib_root/file_1.rs");

    let tree = module_tree.tree();
    assert_eq!(tree.len(), 2);
    assert_eq!(tree[0].module_name(), "file_1");
    assert_eq!(tree[0].parent_index(), None);
    assert_eq!(tree[1].module_name(), "file_2");
    assert_eq!(tree[1].parent_index(), Some(0));
}

#[test]
fn path_wildcard_unknown() {
    let module_tree = ModuleTree::new("src/parser/tests/module_tree/path_wildcard_unknown.rs");

    assert_eq!(
        module_tree.tree()[0].usable_objects[0].object_name,
        "a::b::c".to_owned()
    );
}

use std::path::Path;

use crate::parser::domain_values::ObjectType;
use crate::parser::services::parse_main_or_mod_file_into_tree;

#[test]
fn return_type() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/functions/return_type.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree[0].usable_objects[0].object_type(), &ObjectType::Function);
    assert_eq!(node_tree[0].usable_objects[0].object_name, "test".to_owned());
    for i in 1..7 {
        assert_eq!(node_tree[0].usable_objects[i].object_type(), &ObjectType::ImplicitUse);
    }
    assert_eq!(node_tree[0].usable_objects[1].object_name, "a".to_owned());
    assert_eq!(node_tree[0].usable_objects[2].object_name, "b::c".to_owned());
    assert_eq!(node_tree[0].usable_objects[3].object_name, "d".to_owned());
    assert_eq!(node_tree[0].usable_objects[4].object_name, "e".to_owned());
    assert_eq!(node_tree[0].usable_objects[5].object_name, "f".to_owned());
    assert_eq!(node_tree[0].usable_objects[6].object_name, "g".to_owned());
}
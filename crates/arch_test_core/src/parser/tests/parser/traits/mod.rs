use std::path::Path;
use crate::parser::services::parse_main_or_mod_file_into_tree;
use crate::parser::domain_values::ObjectType;

#[test]
fn functions() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/traits/functions.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree[0].usable_objects[0].object_type(), &ObjectType::Trait);
    assert_eq!(node_tree[0].usable_objects[0].object_name, "TestTrait".to_owned());
    for i in 1..6 {
        assert_eq!(node_tree[0].usable_objects[i].object_type(), &ObjectType::ImplicitUse);
    }
    assert_eq!(node_tree[0].usable_objects[1].object_name, "b".to_owned());
    assert_eq!(node_tree[0].usable_objects[2].object_name, "d".to_owned());
    assert_eq!(node_tree[0].usable_objects[3].object_name, "a".to_owned());
    assert_eq!(node_tree[0].usable_objects[4].object_name, "c".to_owned());
    assert_eq!(node_tree[0].usable_objects[5].object_name, "e".to_owned());
}

#[test]
fn impl_normal() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/traits/impl_normal.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    for i in 0..3 {
        assert_eq!(node_tree[0].usable_objects[i].object_type(), &ObjectType::ImplicitUse);
    }
    assert_eq!(node_tree[0].usable_objects[0].object_name, "Bla".to_owned());
    assert_eq!(node_tree[0].usable_objects[1].object_name, "b".to_owned());
    assert_eq!(node_tree[0].usable_objects[2].object_name, "c".to_owned());
}
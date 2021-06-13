use std::path::Path;

use crate::parser::domain_values::ObjectType;
use crate::parser::services::parse_main_or_mod_file_into_tree;

#[test]
fn empty() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/struct_stmt/empty.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree.len(), 1);
    assert_eq!(node_tree[0].usable_objects.len(), 2);
    assert_eq!(
        node_tree[0].usable_objects[0].object_type(),
        ObjectType::Struct
    );
    assert_eq!(
        node_tree[0].usable_objects[0].object_name,
        "Test1".to_owned()
    );
    assert_eq!(
        node_tree[0].usable_objects[1].object_type(),
        ObjectType::Struct
    );
    assert_eq!(
        node_tree[0].usable_objects[1].object_name,
        "Test2".to_owned()
    );
}

#[test]
fn complex() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/struct_stmt/complex.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(
        node_tree[0].usable_objects[0].object_type(),
        ObjectType::Struct
    );
    assert_eq!(
        node_tree[0].usable_objects[0].object_name,
        "Complex".to_owned()
    );
    assert_eq!(
        node_tree[0].usable_objects[1].object_name,
        "a::b".to_owned()
    );
    assert_eq!(
        node_tree[0].usable_objects[2].object_name,
        "c::d".to_owned()
    );
    assert_eq!(
        node_tree[0].usable_objects[3].object_name,
        "e::f".to_owned()
    );
    assert_eq!(node_tree[0].usable_objects[4].object_name, "g".to_owned());
    assert_eq!(
        node_tree[0].usable_objects[5].object_name,
        "h::i".to_owned()
    );
    assert_eq!(node_tree[0].usable_objects[6].object_name, "j".to_owned());
    assert_eq!(node_tree[0].usable_objects[7].object_name, "k".to_owned());
    for i in 1..8 {
        assert_eq!(
            node_tree[0].usable_objects[i].object_type(),
            ObjectType::ImplicitUse
        );
    }
}

use crate::parser::domain_values::ObjectType;
use crate::parser::services::parse_main_or_mod_file_into_tree;
use std::path::Path;

#[test]
fn complex_tuple() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/types/complex_tuple.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(
        node_tree[0].usable_objects[0].object_type(),
        ObjectType::Function
    );
    assert_eq!(
        node_tree[0].usable_objects[0].object_name,
        "main".to_owned()
    );
    for i in 1..8 {
        assert_eq!(
            node_tree[0].usable_objects[i].object_type(),
            ObjectType::ImplicitUse
        );
    }
    assert_eq!(node_tree[0].usable_objects[1].object_name, "c".to_owned());
    assert_eq!(node_tree[0].usable_objects[2].object_name, "e".to_owned());
    assert_eq!(node_tree[0].usable_objects[3].object_name, "f".to_owned());
    assert_eq!(node_tree[0].usable_objects[4].object_name, "g".to_owned());
    assert_eq!(node_tree[0].usable_objects[5].object_name, "h".to_owned());
    assert_eq!(node_tree[0].usable_objects[6].object_name, "d".to_owned());
    assert_eq!(node_tree[0].usable_objects[7].object_name, "b".to_owned());
}

#[test]
fn complex_generic() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/types/complex_generic.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(
        node_tree[0].usable_objects[0].object_type(),
        ObjectType::Function
    );
    assert_eq!(
        node_tree[0].usable_objects[0].object_name,
        "main".to_owned()
    );
    for i in 1..11 {
        assert_eq!(
            node_tree[0].usable_objects[i].object_type(),
            ObjectType::ImplicitUse
        );
    }
    assert_eq!(
        node_tree[0].usable_objects[1].object_name,
        "a::b".to_owned()
    );
    assert_eq!(node_tree[0].usable_objects[2].object_name, "c".to_owned());
    assert_eq!(node_tree[0].usable_objects[3].object_name, "d".to_owned());
    assert_eq!(node_tree[0].usable_objects[4].object_name, "e".to_owned());
    assert_eq!(
        node_tree[0].usable_objects[5].object_name,
        "f::g".to_owned()
    );
    assert_eq!(node_tree[0].usable_objects[6].object_name, "h".to_owned());
    assert_eq!(node_tree[0].usable_objects[7].object_name, "i".to_owned());
    assert_eq!(node_tree[0].usable_objects[8].object_name, "k".to_owned());
    assert_eq!(node_tree[0].usable_objects[9].object_name, "l".to_owned());
    assert_eq!(node_tree[0].usable_objects[10].object_name, "j".to_owned());
}

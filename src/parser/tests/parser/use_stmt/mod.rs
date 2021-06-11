use std::path::Path;

use crate::parser::domain_values::ObjectType;
use crate::parser::services::parse_main_or_mod_file_into_tree;

#[test]
fn visibility() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/use_stmt/visibility.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree.len(), 1);
    assert_eq!(node_tree[0].usable_objects.len(), 2);
    assert_eq!(node_tree[0].usable_objects[0].object_type(), &ObjectType::RePublish);
    assert_eq!(node_tree[0].usable_objects[0].object_name, "a::b".to_owned());
    assert_eq!(node_tree[0].usable_objects[1].object_type(), &ObjectType::Use);
    assert_eq!(node_tree[0].usable_objects[1].object_name, "c::d".to_owned());
}

#[test]
fn nested() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/use_stmt/nested.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    let usable_objects = &node_tree[0].usable_objects;
    for obj in usable_objects {
        assert_eq!(obj.object_type(), &ObjectType::Use);
    }

    assert_eq!(usable_objects[0].object_name, "a::b::c".to_owned());
    assert_eq!(usable_objects[1].object_name, "a::d::e::f::g::h".to_owned());
    assert_eq!(usable_objects[2].object_name, "a::d::e::i::j::k".to_owned());
    assert_eq!(usable_objects[3].object_name, "a::d::e::i::j::l".to_owned());
    assert_eq!(usable_objects[4].object_name, "a::d::e::i::j::m::n::o::p".to_owned());
    assert_eq!(usable_objects[5].object_name, "a::d::e::q::r".to_owned());
    assert_eq!(usable_objects[6].object_name, "a::s::t::u".to_owned());
    assert_eq!(usable_objects[7].object_name, "a::v::w".to_owned());
    assert_eq!(usable_objects[8].object_name, "a::v::x".to_owned());
    assert_eq!(usable_objects[9].object_name, "a::v::y::z".to_owned());
}
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
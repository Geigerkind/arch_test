use std::path::Path;

use crate::parser::services::parse_main_or_mod_file_into_tree;

#[test]
fn simple() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/macros/simple.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree[0].usable_objects.len(), 2);
    assert_eq!(
        node_tree[0].usable_objects[0].object_name,
        "main".to_owned()
    );
    assert_eq!(
        node_tree[0].usable_objects[1].object_name,
        "q::r::vec".to_owned()
    );
}

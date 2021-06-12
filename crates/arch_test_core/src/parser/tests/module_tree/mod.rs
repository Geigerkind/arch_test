use crate::ModuleTree;
use crate::parser::entities::ModuleNode;

#[test]
fn correct_fully_qualified_names() {
    let module_tree = ModuleTree::new("src/parser/tests/module_tree/correct_fully_qualified_names/main.rs");
    let tree: &Vec<ModuleNode> = module_tree.tree();

    assert_eq!(tree[0].usable_objects[0].object_name, "test".to_owned());
    assert_eq!(tree[0].usable_objects[1].object_name, "crate::republish::wambo::WAMBO".to_owned());
    assert_eq!(tree[0].usable_objects[2].object_name, "crate::republish::testo::TESTO".to_owned());

    assert_eq!(tree[1].usable_objects[0].object_name, "crate::republish::wambo::WAMBO".to_owned());
    assert_eq!(tree[1].usable_objects[1].object_name, "crate::republish::testo::TESTO".to_owned());
    assert_eq!(tree[1].usable_objects[2].object_name, "test".to_owned());
    assert_eq!(tree[1].usable_objects[3].object_name, "crate::republish::wambo::WAMBO".to_owned());
    assert_eq!(tree[1].usable_objects[4].object_name, "crate::republish::wambo::WAMBO".to_owned());
    assert_eq!(tree[1].usable_objects[5].object_name, "crate::republish::testo::TESTO".to_owned());
    assert_eq!(tree[1].usable_objects[6].object_name, "crate::republish::testo::TESTO".to_owned());

    assert_eq!(tree[2].usable_objects[0].object_name, "WAMBO".to_owned());
    assert_eq!(tree[3].usable_objects[0].object_name, "TESTO".to_owned());
}
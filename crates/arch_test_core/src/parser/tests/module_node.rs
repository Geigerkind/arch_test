use crate::parser::entities::ModuleNode;

#[test]
fn register_child() {
    let mut module_node = ModuleNode::new("WAMBO".to_owned(), 0, None, "WAMBO".to_owned());

    assert!(module_node.children().is_empty());
    module_node.register_child(1);
    assert!(!module_node.children().is_empty());
    assert_eq!(module_node.children()[0], 1);
}
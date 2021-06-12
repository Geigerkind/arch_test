use crate::parser::entities::ModuleNode;

#[test]
fn register_child() {
    let mut module_node = ModuleNode::new("WAMBO".to_owned(), 0, None, "WAMBO".to_owned());

    assert!(module_node.children().is_empty());
    module_node.register_child(1);
    assert!(!module_node.children().is_empty());
    assert_eq!(module_node.children()[0], 1);
}

#[test]
fn included_nodes() {
    let mut node1 = ModuleNode::new("WAMBO".to_owned(), 0, None, "WAMBO".to_owned());
    let mut node2 = node1.clone();
    let mut node3 = node1.clone();
    let node4 = node1.clone();

    node1.register_child(1);
    node2.register_child(2);
    node3.register_child(3);

    let tree: Vec<ModuleNode> = vec![node1, node2, node3, node4];

    assert_eq!(tree[0].included_nodes(&tree), vec![1, 2, 3]);
    assert_eq!(tree[1].included_nodes(&tree), vec![2, 3]);
    assert_eq!(tree[2].included_nodes(&tree), vec![3]);
    assert_eq!(tree[3].included_nodes(&tree), vec![]);
}
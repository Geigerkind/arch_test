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

#[test]
fn get_fully_qualified_path() {
    let node1 = ModuleNode::new("WAMBO".to_owned(), 0, None, "WAMBO".to_owned());
    let node2 = ModuleNode::new("WAMBO1".to_owned(), 1, Some(0), "WAMBO1".to_owned());
    let node3 = ModuleNode::new("WAMBO2".to_owned(), 2, Some(1), "WAMBO2".to_owned());
    let node4 = ModuleNode::new("WAMBO3".to_owned(), 3, Some(2), "WAMBO3".to_owned());
    let tree: Vec<ModuleNode> = vec![node1, node2, node3, node4];

    assert_eq!(tree[0].get_fully_qualified_path(&tree), "WAMBO".to_owned());
    assert_eq!(tree[1].get_fully_qualified_path(&tree), "WAMBO::WAMBO1".to_owned());
    assert_eq!(tree[2].get_fully_qualified_path(&tree), "WAMBO::WAMBO1::WAMBO2".to_owned());
    assert_eq!(tree[3].get_fully_qualified_path(&tree), "WAMBO::WAMBO1::WAMBO2::WAMBO3".to_owned());
}
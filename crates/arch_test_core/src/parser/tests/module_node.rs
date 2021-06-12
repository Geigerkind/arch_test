use crate::ModuleTree;
use crate::parser::entities::ModuleNode;
use std::collections::HashMap;
use crate::parser::domain_values::ObjectUse;

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

#[test]
fn object_uses_with_children() {
    let module_tree = ModuleTree::new("src/parser/tests/module_tree/correct_fully_qualified_names/main.rs");
    let tree: &Vec<ModuleNode> = module_tree.tree();
    let use_map: &HashMap<String, ObjectUse> = module_tree.possible_uses();

    let node1_object_uses = tree[0].object_uses(tree, use_map, true);
    let node2_object_uses = tree[1].object_uses(tree, use_map, true);
    let node3_object_uses = tree[2].object_uses(tree, use_map, true);
    let node4_object_uses = tree[3].object_uses(tree, use_map, true);

    assert_eq!(node1_object_uses.iter().count(), 2);
    assert!(node1_object_uses.iter().any(|obj_use| obj_use.node_index() == &2 && obj_use.full_module_path() == "crate::republish::wambo::WAMBO"));
    assert!(node1_object_uses.iter().any(|obj_use| obj_use.node_index() == &3 && obj_use.full_module_path() == "crate::republish::testo::TESTO"));

    assert_eq!(node2_object_uses.iter().count(), 2);
    assert!(node2_object_uses.iter().any(|obj_use| obj_use.node_index() == &2 && obj_use.full_module_path() == "crate::republish::wambo::WAMBO"));
    assert!(node2_object_uses.iter().any(|obj_use| obj_use.node_index() == &3 && obj_use.full_module_path() == "crate::republish::testo::TESTO"));

    assert_eq!(node3_object_uses.iter().count(), 0);
    assert_eq!(node4_object_uses.iter().count(), 0);
}
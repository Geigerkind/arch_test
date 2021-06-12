use std::collections::{BTreeSet, HashMap};

use crate::parser::materials::ModuleTree;
use crate::parser::domain_values::ObjectUse;


pub fn contains_cyclic_dependency(module_tree: &ModuleTree) -> bool {
    module_tree.tree().iter().enumerate().any(|(index, node)| {
        // TODO: Not enough to just use node 0?
        node.object_uses(module_tree.tree(), module_tree.possible_uses(), false).iter().filter(|obj_use| *obj_use.node_index() != index)
            .any(|obj_use| {
                let mut visited_nodes = BTreeSet::new();
                visited_nodes.insert(0);
                find_traverse(&mut visited_nodes, *obj_use.node_index(), module_tree)
            })
    })
}

fn find_traverse(visited_nodes: &mut BTreeSet<usize>, current_index: usize, module_tree: &ModuleTree) -> bool {
    if visited_nodes.contains(&current_index) {
        return true;
    }
    visited_nodes.insert(current_index);
    for obj_use in module_tree.tree()[current_index].object_uses(module_tree.tree(), module_tree.possible_uses(), false).iter().filter(|obj_use| *obj_use.node_index() != current_index) {
        if find_traverse(visited_nodes, *obj_use.node_index(), module_tree) {
            return true;
        }
        visited_nodes.remove(obj_use.node_index());
    }
    visited_nodes.remove(&current_index);
    false
}

pub fn contains_cyclic_dependency_on_any_level(module_tree: &ModuleTree) -> bool {
    let mut current_level = 1;
    while module_tree.tree().iter().any(|node| *node.level() == current_level) {
        if contains_cyclic_dependency_on_level(module_tree, current_level) {
            return true;
        }
        current_level += 1;
    }
    false
}

pub fn contains_cyclic_dependency_on_level(module_tree: &ModuleTree, level: usize) -> bool {
    let mut node_mapping = HashMap::new();
    let mut obj_uses_per_level = HashMap::new();
    let current_tree = module_tree.tree();

    current_tree.iter().enumerate().filter(|(_, node)| *node.level() == level).for_each(|(index, node)| {
        let included_nodes: Vec<usize> = node.included_nodes(current_tree);
        node_mapping.insert(index, index);
        for node_index in included_nodes.iter() {
            node_mapping.insert(*node_index, index);
        }
        let level_uses: Vec<ObjectUse> = node.object_uses(current_tree, module_tree.possible_uses(), true)
            .into_iter().filter(|obj_use| !included_nodes.contains(obj_use.node_index()) && *obj_use.node_index() != index).collect();
        obj_uses_per_level.insert(index, level_uses);
    });

    obj_uses_per_level.iter().any(|(index, obj_uses)| {
        obj_uses.iter().filter(|obj_use| node_mapping.contains_key(obj_use.node_index())).any(|obj_use| {
            let mut visited_nodes = BTreeSet::new();
            visited_nodes.insert(*index);
            find_traverse_on_level(&mut visited_nodes, *node_mapping.get(obj_use.node_index()).unwrap(), &node_mapping, &obj_uses_per_level)
        })
    })
}

fn find_traverse_on_level(visited_nodes: &mut BTreeSet<usize>, current_index: usize, node_mapping: &HashMap<usize, usize>, obj_uses_per_level: &HashMap<usize, Vec<ObjectUse>>) -> bool {
    if visited_nodes.contains(&current_index) {
        return true;
    }
    visited_nodes.insert(current_index);
    for obj_use in obj_uses_per_level.get(&current_index).unwrap().iter() {
        let obj_use_index = *node_mapping.get(obj_use.node_index()).unwrap();
        if find_traverse_on_level(visited_nodes, obj_use_index, node_mapping, obj_uses_per_level) {
            return true;
        }
        visited_nodes.remove(&obj_use_index);
    }
    visited_nodes.remove(&current_index);
    false
}
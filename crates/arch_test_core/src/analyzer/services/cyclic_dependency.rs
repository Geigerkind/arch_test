use std::collections::HashMap;

use itertools::Itertools;

use crate::parser::domain_values::UseRelation;
use crate::parser::materials::ModuleTree;

pub fn contains_cyclic_dependency(module_tree: &ModuleTree) -> Option<Vec<UseRelation>> {
    let mut visited_nodes: Vec<UseRelation> = Vec::new();
    for (index, node) in module_tree.tree().iter().enumerate() {
        if node
            .use_relations(module_tree.tree(), module_tree.possible_uses(), false)
            .iter()
            .filter(|use_relation| use_relation.used_object().node_index() != index)
            .dedup_by(|left, right| {
                left.used_object().node_index() == right.used_object().node_index()
            })
            .any(|use_relation| {
                visited_nodes.clear();
                visited_nodes.push(use_relation.clone());
                find_traverse(
                    &mut visited_nodes,
                    use_relation.used_object().node_index(),
                    module_tree,
                )
            })
        {
            let last_index = visited_nodes.last().cloned().unwrap();
            let mut result = vec![last_index.clone()];
            for node in visited_nodes.into_iter().rev().skip(1) {
                result.push(node);
                if index == last_index.used_object().node_index() {
                    break;
                }
            }
            return Some(result);
        }
    }
    None
}

fn find_traverse(
    visited_nodes: &mut Vec<UseRelation>,
    current_index: usize,
    module_tree: &ModuleTree,
) -> bool {
    if visited_nodes
        .iter()
        .any(|use_relation| use_relation.using_object().node_index() == current_index)
    {
        return true;
    }
    for use_relation in module_tree.tree()[current_index]
        .use_relations(module_tree.tree(), module_tree.possible_uses(), false)
        .iter()
        .filter(|use_relation| use_relation.used_object().node_index() != current_index)
        .dedup_by(|left, right| left.used_object().node_index() == right.used_object().node_index())
    {
        visited_nodes.push(use_relation.clone());
        if find_traverse(
            visited_nodes,
            use_relation.used_object().node_index(),
            module_tree,
        ) {
            return true;
        }
        visited_nodes.pop();
    }
    false
}

pub fn contains_cyclic_dependency_on_any_level(
    module_tree: &ModuleTree,
) -> Option<Vec<UseRelation>> {
    let mut current_level = 1;
    while module_tree
        .tree()
        .iter()
        .any(|node| node.level() == current_level)
    {
        if let Some(involved) = contains_cyclic_dependency_on_level(module_tree, current_level) {
            return Some(involved);
        }
        current_level += 1;
    }
    None
}

pub fn contains_cyclic_dependency_on_level(
    module_tree: &ModuleTree,
    level: usize,
) -> Option<Vec<UseRelation>> {
    let mut node_mapping = HashMap::new();
    let mut use_relations_per_level = HashMap::new();
    let current_tree = module_tree.tree();

    current_tree
        .iter()
        .enumerate()
        .filter(|(_, node)| node.level() == level)
        .for_each(|(index, node)| {
            let included_nodes: Vec<usize> = node.included_nodes(current_tree);
            node_mapping.insert(index, index);
            for node_index in included_nodes.iter() {
                node_mapping.insert(*node_index, index);
            }
            let level_uses: Vec<UseRelation> = node
                .use_relations(current_tree, module_tree.possible_uses(), true)
                .into_iter()
                .filter(|use_relation| {
                    !included_nodes.contains(&use_relation.used_object().node_index())
                        && use_relation.used_object().node_index() != index
                })
                .collect();
            use_relations_per_level.insert(index, level_uses);
        });

    let mut visited_nodes: Vec<UseRelation> = Vec::new();
    for (index, use_relations) in use_relations_per_level.iter() {
        if use_relations
            .iter()
            .filter(|use_relation| {
                node_mapping.contains_key(&use_relation.used_object().node_index())
            })
            .dedup_by(|left, right| {
                node_mapping.get(&left.used_object().node_index()).unwrap()
                    == node_mapping.get(&right.used_object().node_index()).unwrap()
            })
            .any(|use_relation| {
                visited_nodes.clear();
                visited_nodes.push(use_relation.clone());
                find_traverse_on_level(
                    &mut visited_nodes,
                    *node_mapping
                        .get(&use_relation.used_object().node_index())
                        .unwrap(),
                    &node_mapping,
                    &use_relations_per_level,
                )
            })
        {
            let last_index = visited_nodes.last().cloned().unwrap();
            let mut result = vec![last_index.clone()];
            for node in visited_nodes.into_iter().rev().skip(1) {
                result.push(node);
                if *index == last_index.used_object().node_index() {
                    break;
                }
            }
            return Some(result);
        }
    }
    None
}

fn find_traverse_on_level(
    visited_nodes: &mut Vec<UseRelation>,
    current_index: usize,
    node_mapping: &HashMap<usize, usize>,
    use_relations_per_level: &HashMap<usize, Vec<UseRelation>>,
) -> bool {
    if visited_nodes
        .iter()
        .any(|use_relation| use_relation.using_object().node_index() == current_index)
    {
        return true;
    }
    for use_relation in use_relations_per_level
        .get(&current_index)
        .unwrap()
        .iter()
        .dedup_by(|left, right| {
            node_mapping.get(&left.used_object().node_index()).unwrap()
                == node_mapping.get(&right.used_object().node_index()).unwrap()
        })
    {
        let use_relation_index = *node_mapping
            .get(&use_relation.used_object().node_index())
            .unwrap();
        visited_nodes.push(use_relation.clone());
        if find_traverse_on_level(
            visited_nodes,
            use_relation_index,
            node_mapping,
            use_relations_per_level,
        ) {
            return true;
        }
        visited_nodes.pop();
    }
    false
}

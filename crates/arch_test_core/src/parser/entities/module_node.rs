use std::collections::{HashMap, HashSet};

use crate::parser::domain_values::{ObjectType, ObjectUse, UsableObject, UseRelation};

#[derive(Debug, Clone, Getters)]
pub struct ModuleNode {
    parent_index: Option<usize>,
    level: usize,
    file_path: String,
    module_name: String,
    children: Vec<usize>,
    pub usable_objects: Vec<UsableObject>,
}

impl ModuleNode {
    pub fn new(file_path: String, level: usize, parent_index: Option<usize>, module_name: String) -> Self {
        ModuleNode {
            parent_index,
            level,
            file_path,
            module_name,
            children: vec![],
            usable_objects: vec![],
        }
    }

    pub fn register_child(&mut self, child_index: usize) {
        self.children.push(child_index)
    }

    pub fn object_uses(&self, tree: &Vec<Self>, possible_use_map: &HashMap<String, ObjectUse>, include_children: bool) -> HashSet<UseRelation> {
        let mut obj_uses = HashSet::new();
        for obj in self.usable_objects.iter().filter(|obj| obj.object_type() == &ObjectType::RePublish
            || obj.object_type() == &ObjectType::Use || obj.object_type() == &ObjectType::ImplicitUse) {
            if let Some(obj_use) = possible_use_map.get(&obj.object_name) {
                obj_uses.insert(UseRelation::new(obj.clone(), obj_use.clone()));
            }
        }

        if include_children {
            for child in self.children.iter() {
                for obj_use in tree[*child].object_uses(tree, possible_use_map, true) {
                    obj_uses.insert(obj_use);
                }
            }
        }

        obj_uses
    }

    pub fn get_fully_qualified_path(&self, tree: &Vec<Self>) -> String {
        let mut name = self.module_name.clone();
        let mut parent_index = self.parent_index.clone();
        while let Some(index) = parent_index {
            name = format!("{}::{}", tree[index].module_name.clone(), name);
            parent_index = tree[index].parent_index.clone();
        }
        name
    }

    pub fn included_nodes(&self, tree: &Vec<Self>) -> Vec<usize> {
        let mut result = self.children.clone();
        for child in self.children.iter() {
            result.append(&mut tree[*child].included_nodes(tree));
        }
        result
    }
}
use std::collections::HashMap;
use std::path::Path;

use crate::parser::domain_values::{ObjectType, ObjectUse};
use crate::parser::entities::ModuleNode;
use crate::parser::services::parse_main_or_mod_file_into_tree;

#[derive(Debug)]
pub struct ModuleTree {
    tree: Vec<ModuleNode>,
    possible_uses: HashMap<String, ObjectUse>,
}

impl ModuleTree {
    pub fn new(root_directory: &str) -> Self {
        let path = Path::new(root_directory);
        assert!(path.exists(), "Expecting a valid path");
        assert!(path.is_file(), "Expecting path to be a file!");
        let file_name = path.file_name().and_then(|os_str| os_str.to_str()).unwrap();
        let module_name = if file_name == "main.rs" || file_name == "lib.rs" {
            "crate".to_owned()
        } else {
            path.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap()
                .trim_end_matches(".rs")
                .to_owned()
        };

        let mut module_tree = ModuleTree {
            tree: vec![],
            possible_uses: HashMap::default(),
        };
        parse_main_or_mod_file_into_tree(&mut module_tree.tree, path, 0, None, module_name);
        module_tree.correct_fully_qualified_names();
        module_tree.correct_republish_paths();
        module_tree.construct_possible_use_map();
        module_tree
    }

    fn correct_fully_qualified_names(&mut self) {
        let fully_qualified_names: Vec<String> = self
            .tree
            .iter()
            .map(|node| node.get_fully_qualified_path(&self.tree))
            .collect();
        let module_names: Vec<String> = self
            .tree
            .iter()
            .map(|node| node.module_name().clone())
            .collect();
        for (index, node) in self.tree.iter_mut().enumerate() {
            let node_children_module_names: Vec<String> = node
                .children()
                .iter()
                .map(|child_index| module_names[*child_index].clone())
                .collect();
            for uses in node.usable_objects.iter_mut().filter(|obj| {
                obj.object_type() == ObjectType::Use || obj.object_type() == ObjectType::RePublish
            }) {
                if uses.object_name.starts_with("self::") {
                    uses.object_name = uses
                        .object_name
                        .replace("self::", &format!("{}::", fully_qualified_names[index]));
                } else if !uses.object_name.starts_with("crate::") {
                    let has_mod_prefix = node_children_module_names
                        .iter()
                        .any(|module_name| uses.object_name.starts_with(module_name));
                    if has_mod_prefix {
                        uses.object_name =
                            format!("{}::{}", fully_qualified_names[index], uses.object_name);
                    }
                }
            }

            let use_paths: Vec<String> = node
                .usable_objects
                .iter()
                .filter(|obj| {
                    obj.object_type() == ObjectType::Use
                        || obj.object_type() == ObjectType::RePublish
                })
                .map(|obj| obj.object_name.clone())
                .collect();
            for uses in node
                .usable_objects
                .iter_mut()
                .filter(|obj| obj.object_type() == ObjectType::ImplicitUse)
            {
                let splits: Vec<&str> = uses.object_name.split("::").collect();
                if let Some(prefix) = use_paths.iter().find(|prefix| prefix.ends_with(&splits[0])) {
                    if splits.len() > 1 {
                        uses.object_name = format!("{}::{}", prefix, splits[1..].join("::"));
                    } else {
                        uses.object_name = prefix.clone();
                    }
                } else if let Some(prefix) = use_paths
                    .iter()
                    .find(|prefix| prefix.ends_with(&uses.object_name))
                {
                    uses.object_name = prefix.clone();
                } else if splits.len() > 1
                    && node_children_module_names
                        .iter()
                        .any(|name| name == splits[0])
                {
                    uses.object_name =
                        format!("{}::{}", fully_qualified_names[index], uses.object_name);
                }
            }
        }
    }

    fn correct_republish_paths(&mut self) {
        let mut republish_map = HashMap::new();
        let fully_qualified_names: Vec<String> = self
            .tree
            .iter()
            .map(|node| node.get_fully_qualified_path(&self.tree))
            .collect();

        for (index, node) in self.tree.iter().enumerate() {
            let prefix = fully_qualified_names[index].clone();
            for path_obj in node
                .usable_objects
                .iter()
                .filter(|obj| obj.object_type() == ObjectType::RePublish)
            {
                let split_vec = path_obj.object_name.split("::").collect::<Vec<&str>>();
                republish_map.insert(
                    format!("{}::{}", prefix, split_vec.last().unwrap()),
                    path_obj.object_name.clone(),
                );
            }
        }

        for node in self.tree.iter_mut() {
            for uses in node.usable_objects.iter_mut().filter(|obj| {
                (obj.object_type() == ObjectType::Use
                    || obj.object_type() == ObjectType::ImplicitUse
                    || obj.object_type() == ObjectType::RePublish)
            }) {
                uses.object_name = republish_map
                    .get(&uses.object_name)
                    .cloned()
                    .unwrap_or(uses.object_name.clone());
            }
        }
    }

    fn construct_possible_use_map(&mut self) {
        let fully_qualified_names: Vec<String> = self
            .tree
            .iter()
            .map(|node| node.get_fully_qualified_path(&self.tree))
            .collect();
        for (index, node) in self.tree.iter().enumerate() {
            let prefix = fully_qualified_names[index].clone();
            for path_obj in node.usable_objects.iter().filter(|obj| {
                obj.object_type() != ObjectType::RePublish
                    && obj.object_type() != ObjectType::Use
                    && obj.object_type() != ObjectType::ImplicitUse
            }) {
                let full_path = format!("{}::{}", prefix, path_obj.object_name);
                self.possible_uses.insert(
                    full_path.clone(),
                    ObjectUse::new(index, full_path, path_obj.clone()),
                );
            }
        }
    }

    pub fn tree(&self) -> &Vec<ModuleNode> {
        &self.tree
    }

    pub fn possible_uses(&self) -> &HashMap<String, ObjectUse> {
        &self.possible_uses
    }
}

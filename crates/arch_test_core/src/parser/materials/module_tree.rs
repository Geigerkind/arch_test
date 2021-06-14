use std::collections::HashMap;
use std::path::Path;

use crate::parser::domain_values::{ObjectType, ObjectUse, UsableObject};
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
        module_tree.replace_path_wildcard();
        module_tree.correct_republish_paths();
        module_tree.filter_primary_types();
        module_tree.filter_covered_implicit_uses();
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
                obj.object_type() == ObjectType::Use
                    || obj.object_type() == ObjectType::ImplicitUse
                    || obj.object_type() == ObjectType::RePublish
            }) {
                uses.object_name = republish_map
                    .get(&uses.object_name)
                    .cloned()
                    .unwrap_or_else(|| uses.object_name.clone());
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

    fn filter_primary_types(&mut self) {
        let primary_types = vec![
            "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "isize", "usize",
            "str", "char", "f32", "f64", "bool", "Self", "self",
        ];
        let object_primary_types = vec![
            "std::collections::HashMap",
            "std::collections::HashSet",
            "std::collections::VecDeque",
            "Vec",
            "vec",
            "String",
            "std::collections::LinkedList",
            "std::collections::BTreeMap",
            "std::collections::BTreeSet",
            "std::collections::BinaryHeap",
        ];

        for node in self.tree.iter_mut() {
            for i in (0..node.usable_objects.len()).rev() {
                if primary_types.contains(&node.usable_objects[i].object_name.as_str()) {
                    node.usable_objects.remove(i);
                } else {
                    let splits: Vec<&str> =
                        node.usable_objects[i].object_name.split("::").collect();
                    if object_primary_types.contains(&node.usable_objects[i].object_name.as_str())
                        || object_primary_types
                            .contains(&splits[..(splits.len() - 1)].join("::").as_str())
                    {
                        node.usable_objects.remove(i);
                    }
                }
            }
        }
    }

    fn replace_path_wildcard(&mut self) {
        loop {
            let mut wild_card_path: Option<(usize, usize, UsableObject)> = None;
            for node in self.tree.iter_mut() {
                for i in (0..node.usable_objects.len()).rev() {
                    if node.usable_objects[i].object_name.ends_with('*') {
                        wild_card_path = Some((node.index(), i, node.usable_objects[i].clone()));
                        break;
                    }
                }
                if wild_card_path.is_some() {
                    break;
                }
            }

            if let Some((node_index, usable_object_index, use_obj)) = wild_card_path {
                let path = use_obj.object_name.trim_end_matches("::*").to_string();
                if let Some(matching_node) = self
                    .tree
                    .iter()
                    .find(|node| node.get_fully_qualified_path(&self.tree) == path)
                    .cloned()
                {
                    for obj in matching_node
                        .usable_objects
                        .iter()
                        .filter(|obj| obj.is_public() && obj.object_name.split("::").count() == 1)
                    {
                        match obj.object_type() {
                            ObjectType::Struct
                            | ObjectType::Trait
                            | ObjectType::Enum
                            | ObjectType::Function => {
                                self.tree[node_index].usable_objects.push(UsableObject::new(
                                    use_obj.is_public(),
                                    use_obj.object_type(),
                                    format!("{}::{}", path, obj.object_name),
                                    *use_obj.text_range(),
                                ));
                            }
                            _ => continue,
                        };
                    }
                    self.tree[node_index]
                        .usable_objects
                        .remove(usable_object_index);
                } else {
                    self.tree[node_index].usable_objects[usable_object_index].object_name =
                        self.tree[node_index].usable_objects[usable_object_index]
                            .object_name
                            .trim_end_matches("::*")
                            .to_string();
                }
            } else {
                break;
            }
        }
    }

    fn filter_covered_implicit_uses(&mut self) {
        for node in self.tree.iter_mut() {
            for i in (0..node.usable_objects.len()).rev() {
                if node.usable_objects[i].object_name.starts_with("crate::") {
                    continue;
                }

                if node.usable_objects.iter().any(|obj| {
                    obj.object_name != node.usable_objects[i].object_name
                        && obj
                            .object_name
                            .ends_with(&node.usable_objects[i].object_name)
                }) {
                    node.usable_objects.remove(i);
                }
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

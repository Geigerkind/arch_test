use std::collections::HashMap;
use std::fs::DirEntry;
use std::io::Read;
use std::path::Path;
use std::str::Chars;

use regex::Regex;
use syntax::{AstNode, SourceFile, SyntaxKind, SyntaxNode};
use syntax::ast::ModuleItemOwner;

use crate::parser::domain_values::{ObjectType, ObjectUse, ParseFailure, UsableObject};
use crate::parser::entities::ModuleNode;
use crate::parser::utils::read_file_content;

#[derive(Debug, Getters)]
pub struct ModuleTree {
    tree: Vec<ModuleNode>,
    possible_uses: HashMap<String, ObjectUse>,
}

impl ModuleTree {
    pub fn new(root_directory: &str) -> Result<Self, ParseFailure> {
        let path = Path::new(root_directory);
        if !path.exists() {
            return Err(ParseFailure::NotAPath);
        }

        let mut module_tree = ModuleTree { tree: vec![], possible_uses: HashMap::default() };
        parse_file(read_file_content(Path::new("/home/shino/hacking/use_ex.rs")));
        if let Err(e) = module_tree.parse_DEPRECATED(path, None, None, 0, "crate".to_owned()) {
            return Err(e);
        }
        module_tree.correct_fully_qualified_names();
        module_tree.correct_republish_paths();
        module_tree.construct_possible_use_map();
        Ok(module_tree)
    }

    fn parse_DEPRECATED(&mut self, path: &Path, file_content: Option<String>, parent_index: Option<usize>, level: usize, module_name: String) -> Result<(), ParseFailure> {
        let current_index = self.tree.len();
        self.tree.push(ModuleNode::new(path.to_path_buf().into_os_string().into_string().unwrap(), level, parent_index, module_name));

        if let Some(parent_index) = parent_index {
            self.tree.get_mut(parent_index).unwrap().register_child(current_index);
        }

        if file_content.is_some() || path.is_file() {
            let file_content = file_content.unwrap_or(read_file_content(path));
            let (child_modules, pseudo_files, usable_objects) = parse_file_DEPRECATED(file_content);
            self.tree.get_mut(current_index).unwrap().usable_objects = usable_objects;
            if !child_modules.is_empty() {
                if let Some(parent) = path.parent() {
                    let dir_entries: Vec<DirEntry> = parent.read_dir().unwrap().filter_map(|entry| entry.ok()).collect();
                    for entry in dir_entries.into_iter().filter(|entry| child_modules.iter()
                        .any(|module| entry.file_name().to_str().unwrap().to_string().contains(module))) {
                        self.parse_DEPRECATED(&entry.path(), None, Some(current_index), level + 1,
                                              entry.file_name().to_str().unwrap().to_string().trim_end_matches(".rs").to_string())?;
                    }
                }
            }

            for (module_name, file_content) in pseudo_files {
                self.parse_DEPRECATED(path, Some(file_content), Some(current_index), level + 1, module_name)?;
            }
        } else {
            let dir_entries: Vec<DirEntry> = path.read_dir().unwrap().filter_map(|entry| entry.ok()).collect();
            if let Some(dir_entry) = dir_entries.iter().find(|i_path| i_path.path().is_file()
                && (i_path.file_name().to_str().contains(&"main.rs") || i_path.file_name().to_str().contains(&"mod.rs"))) {
                let (child_modules, pseudo_files, usable_objects) = parse_file_DEPRECATED(read_file_content(&dir_entry.path()));
                self.tree.get_mut(current_index).unwrap().usable_objects = usable_objects;

                for entry in dir_entries.iter().filter(|entry| child_modules.iter()
                    .any(|module| entry.file_name().to_str().unwrap().to_string().contains(module))) {
                    self.parse_DEPRECATED(&entry.path(), None, Some(current_index), level + 1,
                                          entry.file_name().to_str().unwrap().to_string().trim_end_matches(".rs").to_string())?;
                }

                for (module_name, file_content) in pseudo_files {
                    self.parse_DEPRECATED(&dir_entry.path(), Some(file_content), Some(current_index), level + 1, module_name)?;
                }
            } else {
                return Err(ParseFailure::PathIsNotARustDirectory);
            }
        }

        Ok(())
    }

    fn correct_fully_qualified_names(&mut self) {
        let fully_qualified_names: Vec<String> = self.tree.iter().map(|node| node.get_fully_qualified_path(&self.tree)).collect();
        let module_names: Vec<String> = self.tree.iter().map(|node| node.module_name().clone()).collect();
        for (index, node) in self.tree.iter_mut().enumerate() {
            let node_children_module_names: Vec<String> = node.children().iter().map(|child_index| module_names[*child_index].clone()).collect();
            for uses in node.usable_objects.iter_mut().filter(|obj| obj.object_type() == &ObjectType::Use || obj.object_type() == &ObjectType::RePublish) {
                if uses.object_name.starts_with("self::") {
                    uses.object_name = uses.object_name.replace("self::", &format!("{}::", fully_qualified_names[index]));
                } else if !uses.object_name.starts_with("crate::") {
                    let has_mod_prefix = node_children_module_names.iter().any(|module_name| uses.object_name.starts_with(module_name));
                    if has_mod_prefix {
                        uses.object_name = format!("{}::{}", fully_qualified_names[index], uses.object_name);
                    }
                }
            }

            let use_paths: Vec<String> = node.usable_objects.iter().filter(|obj| obj.object_type() == &ObjectType::Use || obj.object_type() == &ObjectType::RePublish)
                .map(|obj| obj.object_name.clone()).collect();
            for uses in node.usable_objects.iter_mut().filter(|obj| obj.object_type() == &ObjectType::ImplicitUse) {
                let splits: Vec<&str> = uses.object_name.split("::").collect();
                if splits.len() <= 1 {
                    continue;
                }
                if let Some(prefix) = use_paths.iter().find(|prefix| prefix.split("::").last().unwrap() == splits[0]) {
                    uses.object_name = format!("{}::{}", prefix, splits[1..].join("::"));
                }
            }
        }
    }

    fn correct_republish_paths(&mut self) {
        let mut republish_map = HashMap::new();
        let fully_qualified_names: Vec<String> = self.tree.iter().map(|node| node.get_fully_qualified_path(&self.tree)).collect();

        for (index, node) in self.tree.iter().enumerate() {
            let prefix = fully_qualified_names[index].clone();
            for path_obj in node.usable_objects.iter().filter(|obj| obj.object_type() == &ObjectType::RePublish) {
                let split_vec = path_obj.object_name.split("::").collect::<Vec<&str>>();
                republish_map.insert(format!("{}::{}", prefix, split_vec.last().unwrap()), path_obj.object_name.clone());
            }
        }

        for node in self.tree.iter_mut() {
            for uses in node.usable_objects.iter_mut()
                .filter(|obj| (obj.object_type() == &ObjectType::Use || obj.object_type() == &ObjectType::ImplicitUse || obj.object_type() == &ObjectType::RePublish)) {
                uses.object_name = republish_map.get(&uses.object_name).cloned().unwrap_or(uses.object_name.clone());
            }
        }
    }

    fn construct_possible_use_map(&mut self) {
        let fully_qualified_names: Vec<String> = self.tree.iter().map(|node| node.get_fully_qualified_path(&self.tree)).collect();
        for (index, node) in self.tree.iter().enumerate() {
            let prefix = fully_qualified_names[index].clone();
            for path_obj in node.usable_objects.iter().filter(|obj| obj.object_type() != &ObjectType::RePublish
                && obj.object_type() != &ObjectType::Use && obj.object_type() != &ObjectType::ImplicitUse) {
                let full_path = format!("{}::{}", prefix, path_obj.object_name);
                self.possible_uses.insert(full_path.clone(), ObjectUse::new(index, full_path, path_obj.clone()));
            }
        }
    }
}

fn parse_file(file_content: String) -> (Vec<String>, Vec<(String, String)>, Vec<UsableObject>) {
    let mut usable_objects: Vec<UsableObject> = Vec::new();

    let result = SourceFile::parse(&file_content);
    for item in result.tree().items() {
        match item.syntax().kind() {
            SyntaxKind::USE => {
                let (is_pub, paths) = parse_use_paths(item.syntax());
                for path in paths {
                    usable_objects.push(UsableObject::new(if is_pub { ObjectType::RePublish } else { ObjectType::Use }, path));
                }
            }
            SyntaxKind::STRUCT => {}
            SyntaxKind::STRUCT_KW => {
                println!("STRUCT_KW ?!?!: {}", item.to_string());
            }
            SyntaxKind::ENUM => {}
            SyntaxKind::ENUM_KW => {
                println!("ENUM_KW ?!?!: {}", item.to_string());
            }
            SyntaxKind::FN => {}
            SyntaxKind::FN_KW => {
                println!("FN_KW ?!?!: {}", item.to_string());
            }
            _ => {
                // Do Nothing
                continue;
            }
        }
    }

    println!("{:?}", usable_objects);

    (vec![], vec![], vec![])
}

fn parse_use_paths(syntax_node: &SyntaxNode) -> (bool, Vec<String>) {
    let mut visibility = false;
    let mut paths = Vec::new();
    for child in syntax_node.children() {
        match child.kind() {
            SyntaxKind::VISIBILITY => {
                visibility = true;
            },
            SyntaxKind::USE_TREE => {
                paths = parse_use_tree(&child)
            }
            _ => unreachable!()
        }
    }
    (visibility, paths)
}

fn parse_use_tree(syntax_node: &SyntaxNode) -> Vec<String> {
    let mut path_segments = Vec::new();
    let mut current_prefix = String::new();
    for sub_child in syntax_node.children() {
        match sub_child.kind() {
            SyntaxKind::PATH => {
                current_prefix = sub_child.to_string();
            },
            SyntaxKind::USE_TREE_LIST => {
                for use_tree in sub_child.children() {
                    for segment in parse_use_tree(&use_tree) {
                        path_segments.push(format!("{}::{}", current_prefix, segment));
                    }
                }
            }
            _ => unreachable!()
        }
    }
    if path_segments.is_empty() {
        return vec![current_prefix];
    }
    path_segments
}

fn parse_file_DEPRECATED(contents: String) -> (Vec<String>, Vec<(String, String)>, Vec<UsableObject>) {
    let mut chars = contents.chars();
    let mut scope_counter = 0;
    let mut previous_characters = String::new();

    let mut begin_identifier_name = false;
    let mut parse_use_stmt = false;
    let mut current_identifier_object_type: Option<ObjectType> = None;
    let mut current_identifier = String::new();

    let mut impl_scope: Option<i32> = None;
    let mut mod_scope: Option<(String, String, i32)> = None;

    let mut child_modules: Vec<String> = Vec::new();
    let mut usable_objects: Vec<UsableObject> = Vec::new();
    let mut mod_pseudo_files: Vec<(String, String)> = Vec::new();

    loop {
        let next_character = chars.next();
        if next_character.is_none() {
            break;
        }
        let mut character = next_character.unwrap();

        if let Some((_, pseudo_file, _)) = mod_scope.as_mut() {
            *pseudo_file += &character.to_string();
        }

        if begin_identifier_name && ((!character.is_alphanumeric() && character != '_' && character != ':' && (!parse_use_stmt || (character != ' ' && character != '{' && character != '}' && character != ',')))
            || character == ';' || (!parse_use_stmt && (character == ' ' || character == '{'))) {
            begin_identifier_name = false;

            if current_identifier_object_type.contains(&ObjectType::Mod) {
                let look_ahead_characters = look_ahead_until_or_eof(chars.clone(), vec![';', '{'], character);
                if look_ahead_characters.ends_with(";") {
                    child_modules.push(current_identifier);
                } else if mod_scope.is_none() {
                    if character != '{' && advance_until_or_eof(&mut chars, '{') {
                        character = '{';
                    }
                    mod_scope = Some((current_identifier, String::new(), scope_counter + 1));
                }
            } else if current_identifier_object_type.contains(&ObjectType::Impl) {
                impl_scope = Some(scope_counter + 1);
            } else if let Some(object_type) = current_identifier_object_type {
                if impl_scope.is_none() && mod_scope.is_none() {
                    if parse_use_stmt {
                        // TODO: Support nested identifier, like a::{b, c::{d, e}}
                        // Currently only a::b::{c, d} is supported
                        let splits = current_identifier.split("{").collect::<Vec<&str>>();
                        if splits.len() == 2 {
                            for sub_module in splits[1].split(",") {
                                let sub_module = sub_module.trim_end_matches("}").trim_start_matches("{").trim_start().trim_end();
                                usable_objects.push(UsableObject::new(object_type.clone(), format!("{}{}", splits[0], sub_module)));
                            }
                        } else {
                            usable_objects.push(UsableObject::new(object_type, current_identifier));
                        }
                    } else {
                        usable_objects.push(UsableObject::new(object_type, current_identifier));
                    }
                }
            }

            current_identifier_object_type = None;
            current_identifier = String::new();
            parse_use_stmt = false;
        }

        if begin_identifier_name && (character.is_alphanumeric() || character == '_' || character == ':'
            || (parse_use_stmt && (character == '{' || character == '}' || character == ',' || character == ' '))) {
            current_identifier += &character.to_string();
        }

        if character == '{' {
            scope_counter += 1;
        } else if character == '}' {
            if let Some(impl_scope_counter) = impl_scope {
                if impl_scope_counter == scope_counter {
                    impl_scope = None;
                }
            }

            if mod_scope.iter().any(|(_, _, scope)| *scope == scope_counter) {
                let (module_name, mut pseudo_file, _) = mod_scope.unwrap();
                pseudo_file.pop();
                mod_pseudo_files.push((module_name, pseudo_file));
                mod_scope = None;
            }

            scope_counter -= 1;
        } else if character == ' ' {
            if previous_characters.ends_with("mod") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Mod);
            } else if previous_characters.ends_with("struct") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Struct);
            } else if previous_characters.ends_with("enum") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Enum);
            } else if previous_characters.ends_with("trait") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Trait);
            } else if previous_characters.ends_with("fn") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Function);
            } else if previous_characters.ends_with("impl") {
                begin_identifier_name = true;
                current_identifier_object_type = Some(ObjectType::Impl);
            } else if previous_characters.ends_with("pub use") {
                begin_identifier_name = true;
                parse_use_stmt = true;
                current_identifier_object_type = Some(ObjectType::RePublish);
            } else if previous_characters.ends_with("use") {
                begin_identifier_name = true;
                parse_use_stmt = true;
                current_identifier_object_type = Some(ObjectType::Use);
            }
        }

        previous_characters += &character.to_string();
    }

    let re_use2 = Regex::new(r"([a-zA-Z0-9_:]+::[a-zA-Z0-9_]+)+\(").unwrap();
    for cap in re_use2.captures_iter(&contents) {
        let capture = cap.get(1).unwrap().as_str().to_string();
        usable_objects.push(UsableObject::new(ObjectType::ImplicitUse, capture));
    }

    (child_modules, mod_pseudo_files, usable_objects)
}

fn look_ahead_until_or_eof(mut chars: Chars, until: Vec<char>, current_character: char) -> String {
    let mut result = String::new();
    result += &current_character.to_string();
    while !until.iter().any(|until_char| result.ends_with(*until_char)) {
        let next_char = chars.next();
        if let Some(character) = next_char {
            result += &character.to_string();
        } else {
            break;
        }
    }
    result
}

fn advance_until_or_eof(chars: &mut Chars, until: char) -> bool {
    loop {
        let next_char = chars.next();
        if let Some(character) = next_char {
            if character == until {
                return true;
            }
        } else {
            break;
        }
    }
    false
}
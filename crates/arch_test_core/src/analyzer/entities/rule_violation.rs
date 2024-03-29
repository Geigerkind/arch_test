use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

use ra_ap_syntax::{TextRange, TextSize};

use crate::analyzer::domain_values::RuleViolationType;
use crate::parser::domain_values::UseRelation;
use crate::parser::entities::ModuleNode;

#[derive(Debug)]
pub struct RuleViolation<'r> {
    violation_type: RuleViolationType,
    access_rule: Box<dyn Debug + 'r>,
    involved_object_uses: Vec<UseRelation>,
}

impl<'r> RuleViolation<'r> {
    pub fn new(
        violation_type: RuleViolationType,
        access_rule: Box<dyn Debug + 'r>,
        involved_object_uses: Vec<UseRelation>,
    ) -> Self {
        RuleViolation {
            violation_type,
            access_rule,
            involved_object_uses,
        }
    }

    pub fn violation_type(&self) -> RuleViolationType {
        self.violation_type
    }

    pub fn involved_object_uses(&self) -> &Vec<UseRelation> {
        &self.involved_object_uses
    }

    pub fn access_rule(&self) -> &(dyn Debug + 'r) {
        &self.access_rule
    }

    pub fn print(&self, tree: &[ModuleNode]) {
        match self.violation_type {
            RuleViolationType::LayerDoNotExist => {
                println!(
                    "Layers specified in the rule {:?} do not match specified architecture layers.",
                    self.access_rule
                );
            }
            RuleViolationType::IncompleteLayerSpecification => {
                println!("Layer specification is incomplete!");
            }
            RuleViolationType::SingleLocation => {
                let using_object = self.involved_object_uses[0].using_object();
                let used_object = self.involved_object_uses[0].used_object();
                let (in_file_line_number, in_file_column_range, in_file_line) =
                    find_text_range_in_file(
                        tree[using_object.node_index()].file_path(),
                        using_object.usable_object().text_range(),
                    );
                let (acc_file_line_number, acc_file_column_range, acc_file_line) =
                    find_text_range_in_file(
                        tree[used_object.node_index()].file_path(),
                        used_object.usable_object().text_range(),
                    );
                println!("Violated rule     | {:?}", self.access_rule);
                println!("-------------------");
                println!(
                    "Accessor file     | {}",
                    tree[using_object.node_index()].file_path()
                );
                println!(
                    "Object            | {:?}: {}@{:?}",
                    using_object.usable_object().object_type(),
                    using_object.usable_object().object_name(),
                    using_object.usable_object().text_range()
                );
                println!(
                    "Line in file      | ({}, {:?}): {}",
                    in_file_line_number, in_file_column_range, in_file_line
                );
                println!("-------------------");
                println!(
                    "Accessed file     | {}",
                    tree[used_object.node_index()].file_path()
                );
                println!("Object path       | {}", used_object.full_module_path());
                println!(
                    "Object            | {:?}: {}@{:?}",
                    used_object.usable_object().object_type(),
                    used_object.usable_object().object_name(),
                    used_object.usable_object().text_range()
                );
                println!(
                    "Line in file      | ({}, {:?}): {}",
                    acc_file_line_number, acc_file_column_range, acc_file_line
                );
            }
            RuleViolationType::Cycle => {
                println!("Violated rule: {:?}", self.access_rule);
                for use_relation in self.involved_object_uses.iter() {
                    let using_object = use_relation.using_object();
                    println!(
                        " | File path:    {}",
                        tree[using_object.node_index()].file_path()
                    );
                    println!(
                        " | Accessed:     {:?}: {}@{:?}",
                        using_object.usable_object().object_type(),
                        using_object.usable_object().object_name(),
                        using_object.usable_object().text_range()
                    );
                    println!(" ⮟ ");
                }
            }
        }
    }
}

fn find_text_range_in_file(file_path: &str, text_range: &TextRange) -> (usize, TextRange, String) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut text_conquered: u32 = 0;
    for (line_index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if TextSize::from(text_conquered + line.len() as u32) >= text_range.end() {
                return (
                    line_index + 1,
                    TextRange::new(TextSize::from(1), TextSize::from(line.len() as u32)),
                    line,
                );
            }
            text_conquered += line.len() as u32;
        }
    }
    unreachable!()
}

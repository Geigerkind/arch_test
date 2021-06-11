use std::collections::HashSet;

use crate::analyzer::domain_values::RuleViolation;
use crate::analyzer::services::AccessRule;
use crate::parser::materials::ModuleTree;

pub struct Architecture {
    layer_names: HashSet<String>,
    access_rules: Vec<Box<dyn AccessRule + 'static>>,
}

impl Architecture {
    pub fn new(layer_names: HashSet<String>) -> Self {
        Architecture {
            layer_names,
            access_rules: Vec::default(),
        }
    }

    pub fn with_access_rule(mut self, access_rule: impl AccessRule + 'static) -> Self {
        self.access_rules.push(Box::new(access_rule));
        self
    }

    pub fn check_access_rules(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for access_rule in self.access_rules.iter() {
            access_rule.check(&module_tree)?;
        }
        Ok(())
    }

    pub fn check_complete_layer_specification(&self, module_tree: &ModuleTree) {
        unimplemented!()
    }

    /*
    pub fn add_access_rule_precedence(&mut self, accessor_layer: &str, accessed_layer: &str) {
        let accessor = self.get_layer_index(accessor_layer);
        let accessed = self.get_layer_index(accessed_layer);
        let mut subject_rules = self.access_rules.entry(accessor).or_insert_with(Vec::new);
        subject_rules.push(AccessRule::Precedence(accessed));
    }

    pub fn add_access_rule_may_access(&mut self, accessor_layer: &str, accessed_layers: HashSet<&str>) {
        let accessor = self.get_layer_index(accessor_layer);
        let accessed_layers = accessed_layers.into_iter().map(|layer| self.get_layer_index(layer)).collect::<HashSet<usize>>();
        let mut subject_rules = self.access_rules.entry(accessor).or_insert_with(Vec::new);
        subject_rules.push(AccessRule::MayAccess(accessed_layers));
    }

    pub fn add_access_rule_may_not_access(&mut self, accessor_layer: &str, not_accessed_layers: HashSet<&str>) {
        let accessor = self.get_layer_index(accessor_layer);
        let not_accessed_layers = not_accessed_layers.into_iter().map(|layer| self.get_layer_index(layer)).collect::<HashSet<usize>>();
        let mut subject_rules = self.access_rules.entry(accessor).or_insert_with(Vec::new);
        subject_rules.push(AccessRule::MayNotAccess(not_accessed_layers));
    }
     */
}
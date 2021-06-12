use std::collections::HashSet;
use std::fmt::Debug;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess, NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess};
use crate::analyzer::domain_values::RuleViolationType;
use crate::analyzer::entities::RuleViolation;
use crate::analyzer::services::cyclic_dependency::{contains_cyclic_dependency, contains_cyclic_dependency_on_any_level};
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;

pub trait AccessRule: Debug {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation>;
}

impl AccessRule for MayOnlyAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (index, node) in module_tree.tree().iter().enumerate().filter(|(_, node)| node.module_name() == self.accessor()) {
            if let Some(obj_use) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .find(|obj_use| !self.accessed().contains(module_tree.tree()[*obj_use.used_object().node_index()].module_name())
                    && !has_parent_matching_name(self.accessed(), *obj_use.used_object().node_index(), module_tree.tree())
                    && (!*self.when_same_parent() || module_tree.tree()[*obj_use.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![(index, obj_use.clone())]));
            }
        }
        Ok(())
    }
}

impl AccessRule for MayNotAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (index, node) in module_tree.tree().iter().enumerate().filter(|(_, node)| node.module_name() == self.accessor()) {
            if let Some(obj_use) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .find(|obj_use| (self.accessed().contains(module_tree.tree()[*obj_use.used_object().node_index()].module_name())
                    || has_parent_matching_name(self.accessed(), *obj_use.used_object().node_index(), module_tree.tree()))
                    && (!*self.when_same_parent() || module_tree.tree()[*obj_use.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![(index, obj_use.clone())]));
            }
        }
        Ok(())
    }
}

impl AccessRule for NoParentAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (index, node) in module_tree.tree().iter().enumerate().filter(|(_, node)| node.parent_index().is_some()) {
            if let Some(obj_use) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .find(|obj_use| node.parent_index().contains(obj_use.used_object().node_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![(index, obj_use.clone())]));
            }
        }
        Ok(())
    }
}

impl AccessRule for NoModuleCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if let Some(involved) = contains_cyclic_dependency(module_tree) {
            return Err(RuleViolation::new(RuleViolationType::Cycle, Box::new(self.clone()), involved));
        }
        Ok(())
    }
}

impl AccessRule for NoLayerCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if let Some(involved) = contains_cyclic_dependency_on_any_level(module_tree) {
            return Err(RuleViolation::new(RuleViolationType::Cycle, Box::new(self.clone()), involved));
        }
        Ok(())
    }
}

fn has_parent_matching_name(accessor_name: &HashSet<String>, mut node_index: usize, tree: &Vec<ModuleNode>) -> bool {
    while let Some(parent_index) = tree[node_index].parent_index() {
        if accessor_name.contains(tree[*parent_index].module_name()) {
            return true;
        }
        node_index = *parent_index;
    }
    false
}
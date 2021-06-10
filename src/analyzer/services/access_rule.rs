use std::collections::HashSet;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess, NoParentAccess, NoModuleCyclicDependencies, NoLayerCyclicDependencies};
use crate::analyzer::domain_values::RuleViolation;
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;
use crate::velcro::hash_set;
use crate::analyzer::services::{contains_cyclic_dependency, contains_cyclic_dependency_on_any_level};

pub trait AccessRule {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation>;
}

impl AccessRule for MayOnlyAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (_, node) in module_tree.tree().iter().enumerate().filter(|(index, node)| node.module_name() == self.accessor()) {
            if node.object_uses(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .any(|obj_use| !self.accessed().contains(module_tree.tree()[*obj_use.node_index()].module_name())
                    && !has_parent_matching_name(self.accessed(), *obj_use.node_index(), module_tree.tree())) {
                return Err(RuleViolation);
            }
        }
        Ok(())
    }
}

impl AccessRule for MayNotAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (_, node) in module_tree.tree().iter().enumerate().filter(|(index, node)| node.module_name() == self.accessor()) {
            if node.object_uses(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .any(|obj_use| self.accessed().contains(module_tree.tree()[*obj_use.node_index()].module_name())
                    || has_parent_matching_name(self.accessed(), *obj_use.node_index(), module_tree.tree())) {
                return Err(RuleViolation);
            }
        }
        Ok(())
    }
}

impl AccessRule for NoParentAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter() {
            if node.parent_index().is_some() && node.object_uses(module_tree.tree(), module_tree.possible_uses(), true).iter()
                .any(|obj_use| node.parent_index().contains(obj_use.node_index())) {
                return Err(RuleViolation);
            }
        }
        Ok(())
    }
}

impl AccessRule for NoModuleCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if contains_cyclic_dependency(module_tree) {
            return Err(RuleViolation);
        }
        Ok(())
    }
}

impl AccessRule for NoLayerCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if contains_cyclic_dependency_on_any_level(module_tree) {
            return Err(RuleViolation);
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
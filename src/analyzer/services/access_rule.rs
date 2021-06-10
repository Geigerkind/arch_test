use std::collections::HashSet;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayOnlyAccess};
use crate::analyzer::domain_values::RuleViolation;
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;
use crate::velcro::hash_set;

pub trait AccessRule {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation>;
}

impl AccessRule for MayOnlyAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for (_, node) in module_tree.tree().iter().enumerate().filter(|(index, node)| node.module_name() == self.accessor()
            || has_parent_matching_name(&hash_set![self.accessor().clone()], *index, module_tree.tree())) {
            if node.object_uses(module_tree.tree(), module_tree.possible_uses(), false).iter()
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
        for (_, node) in module_tree.tree().iter().enumerate().filter(|(index, node)| node.module_name() == self.accessor()
            || has_parent_matching_name(&hash_set![self.accessor().clone()], *index, module_tree.tree())) {
            if node.object_uses(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .any(|obj_use| self.accessed().contains(module_tree.tree()[*obj_use.node_index()].module_name())
                    || has_parent_matching_name(self.accessed(), *obj_use.node_index(), module_tree.tree())) {
                return Err(RuleViolation);
            }
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
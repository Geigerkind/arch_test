use std::collections::HashSet;
use std::fmt::Debug;

use velcro::hash_set;

use crate::analyzer::domain_values::access_rules::{MayNotAccess, MayNotBeAccessedBy, MayOnlyAccess, MayOnlyBeAccessedBy, NoLayerCyclicDependencies, NoModuleCyclicDependencies, NoParentAccess};
use crate::analyzer::domain_values::RuleViolationType;
use crate::analyzer::entities::RuleViolation;
use crate::analyzer::services::cyclic_dependency::{contains_cyclic_dependency, contains_cyclic_dependency_on_any_level};
use crate::parser::entities::ModuleNode;
use crate::parser::materials::ModuleTree;
use std::collections::hash_map::RandomState;

pub trait AccessRule: Debug {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation>;
    fn validate(&self, layer_names: &HashSet<String>) -> bool;
}

impl AccessRule for MayOnlyAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter()
            .filter(|node| node.module_name() == self.accessor() || has_parent_matching_name(&hash_set![self.accessor().clone()], node.index(), module_tree.tree())) {
            if let Some(use_relation) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .find(|use_relation| !self.accessed().contains(module_tree.tree()[use_relation.used_object().node_index()].module_name())
                    && !has_parent_matching_name(self.accessed(), use_relation.used_object().node_index(), module_tree.tree())
                    && (!self.when_same_parent() || module_tree.tree()[use_relation.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![use_relation.clone()]));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessor()) && self.accessed().iter().all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayNotAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter()
            .filter(|node| node.module_name() == self.accessor() || has_parent_matching_name(&hash_set![self.accessor().clone()], node.index(), module_tree.tree())) {
            if let Some(use_relation) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .find(|use_relation| (self.accessed().contains(module_tree.tree()[use_relation.used_object().node_index()].module_name())
                    || has_parent_matching_name(self.accessed(), use_relation.used_object().node_index(), module_tree.tree()))
                    && (!self.when_same_parent() || module_tree.tree()[use_relation.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![use_relation.clone()]));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessor()) && self.accessed().iter().all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayOnlyBeAccessedBy {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter()
            .filter(|node| !self.accessors().contains(node.module_name()) && !has_parent_matching_name(self.accessors(), node.index(), module_tree.tree())) {
            if let Some(use_relation) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .find(|use_relation| (self.accessed() == module_tree.tree()[use_relation.used_object().node_index()].module_name()
                    || has_parent_matching_name(&hash_set![self.accessed().clone()], use_relation.used_object().node_index(), module_tree.tree()))
                    && (!self.when_same_parent() || module_tree.tree()[use_relation.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![use_relation.clone()]));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessed()) && self.accessors().iter().all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for MayNotBeAccessedBy {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter()
            .filter(|node| self.accessors().contains(node.module_name()) || has_parent_matching_name(self.accessors(), node.index(), module_tree.tree())) {
            if let Some(use_relation) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .find(|use_relation| (self.accessed() == module_tree.tree()[use_relation.used_object().node_index()].module_name()
                    || has_parent_matching_name(&hash_set![self.accessed().clone()], use_relation.used_object().node_index(), module_tree.tree()))
                    && (!self.when_same_parent() || module_tree.tree()[use_relation.used_object().node_index()].parent_index() == node.parent_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![use_relation.clone()]));
            }
        }
        Ok(())
    }

    fn validate(&self, layer_names: &HashSet<String, RandomState>) -> bool {
        layer_names.contains(self.accessed()) && self.accessors().iter().all(|layer| layer_names.contains(layer))
    }
}

impl AccessRule for NoParentAccess {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        for node in module_tree.tree().iter().filter(|node| node.parent_index().is_some()) {
            if let Some(use_relation) = node.use_relations(module_tree.tree(), module_tree.possible_uses(), false).iter()
                .find(|use_relation| node.parent_index().contains(&use_relation.used_object().node_index())) {
                return Err(RuleViolation::new(RuleViolationType::SingleLocation, Box::new(self.clone()), vec![use_relation.clone()]));
            }
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

impl AccessRule for NoModuleCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if let Some(involved) = contains_cyclic_dependency(module_tree) {
            return Err(RuleViolation::new(RuleViolationType::Cycle, Box::new(self.clone()), involved));
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

impl AccessRule for NoLayerCyclicDependencies {
    fn check(&self, module_tree: &ModuleTree) -> Result<(), RuleViolation> {
        if let Some(involved) = contains_cyclic_dependency_on_any_level(module_tree) {
            return Err(RuleViolation::new(RuleViolationType::Cycle, Box::new(self.clone()), involved));
        }
        Ok(())
    }

    fn validate(&self, _layer_names: &HashSet<String, RandomState>) -> bool {
        true
    }
}

fn has_parent_matching_name(accessor_name: &HashSet<String>, mut node_index: usize, tree: &Vec<ModuleNode>) -> bool {
    while let Some(parent_index) = tree[node_index].parent_index() {
        if accessor_name.contains(tree[parent_index].module_name()) {
            return true;
        }
        node_index = parent_index;
    }
    false
}
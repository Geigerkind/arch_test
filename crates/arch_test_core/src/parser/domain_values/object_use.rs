use crate::parser::domain_values::usable_object::UsableObject;

#[derive(Debug, Clone, Getters, new, Hash, Eq, PartialEq)]
pub struct ObjectUse {
    node_index: usize,
    full_module_path: String,
    usable_object: UsableObject,
}
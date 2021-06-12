use crate::parser::domain_values::{ObjectUse, UsableObject};

#[derive(Debug, Clone, Getters, new, Hash, Eq, PartialEq)]
pub struct UseRelation {
    using_object: UsableObject,
    used_object: ObjectUse,
}
use crate::parser::domain_values::{ObjectUse, UsableObject};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UseRelation {
    using_object: UsableObject,
    used_object: ObjectUse,
}

impl UseRelation {
    pub fn new(using_object: UsableObject, used_object: ObjectUse) -> Self {
        UseRelation { using_object, used_object }
    }

    pub fn using_object(&self) -> &UsableObject {
        &self.using_object
    }

    pub fn used_object(&self) -> &ObjectUse {
        &self.used_object
    }
}
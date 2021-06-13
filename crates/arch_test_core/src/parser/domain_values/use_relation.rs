use crate::parser::domain_values::ObjectUse;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct UseRelation {
    using_object: ObjectUse,
    used_object: ObjectUse,
}

impl UseRelation {
    pub fn new(using_object: ObjectUse, used_object: ObjectUse) -> Self {
        UseRelation {
            using_object,
            used_object,
        }
    }

    pub fn using_object(&self) -> &ObjectUse {
        &self.using_object
    }

    pub fn used_object(&self) -> &ObjectUse {
        &self.used_object
    }
}

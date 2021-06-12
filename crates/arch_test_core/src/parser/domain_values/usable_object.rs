use std::hash::{Hash, Hasher};

use syntax::TextRange;

use crate::parser::domain_values::ObjectType;

#[derive(Debug, Clone, Getters, new)]
pub struct UsableObject {
    object_type: ObjectType,
    pub object_name: String,
    text_range: TextRange,
}

impl Hash for UsableObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.object_type.hash(state);
        self.object_name.hash(state);
    }
}

impl PartialEq for UsableObject {
    fn eq(&self, other: &Self) -> bool {
        self.object_type == other.object_type
            && self.object_name == other.object_name
    }
}

impl Eq for UsableObject {}
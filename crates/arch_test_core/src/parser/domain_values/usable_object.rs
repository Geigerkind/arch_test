use std::hash::{Hash, Hasher};

use syntax::TextRange;

use crate::parser::domain_values::ObjectType;

#[derive(Debug, Clone)]
pub struct UsableObject {
    object_type: ObjectType,
    pub object_name: String,
    text_range: TextRange,
}

impl UsableObject {
    pub fn new(object_type: ObjectType, object_name: String, text_range: TextRange) -> Self {
        UsableObject {
            object_type,
            object_name,
            text_range,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        self.object_type
    }

    pub fn object_name(&self) -> &String {
        &self.object_name
    }

    pub fn text_range(&self) -> &TextRange {
        &self.text_range
    }
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
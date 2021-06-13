use std::hash::{Hash, Hasher};

use ra_ap_syntax::TextRange;

use crate::parser::domain_values::ObjectType;

#[derive(Debug, Clone)]
pub struct UsableObject {
    is_public: bool,
    object_type: ObjectType,
    pub object_name: String,
    text_range: TextRange,
}

impl UsableObject {
    pub fn new(
        is_public: bool,
        object_type: ObjectType,
        object_name: String,
        text_range: TextRange,
    ) -> Self {
        UsableObject {
            is_public,
            object_type,
            object_name,
            text_range,
        }
    }

    pub fn is_public(&self) -> bool {
        self.is_public
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
        self.object_name.hash(state);
    }
}

impl PartialEq for UsableObject {
    fn eq(&self, other: &Self) -> bool {
        self.object_name == other.object_name
    }
}

impl Eq for UsableObject {}

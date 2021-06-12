use syntax::TextRange;

use crate::parser::domain_values::ObjectType;

#[derive(Debug, Clone, Getters, new, Hash, PartialEq, Eq)]
pub struct UsableObject {
    object_type: ObjectType,
    pub object_name: String,
    text_range: TextRange,
}
use crate::parser::domain_values::usable_object::UsableObject;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ObjectUse {
    node_index: usize,
    full_module_path: String,
    usable_object: UsableObject,
}

impl ObjectUse {
    pub fn new(node_index: usize, full_module_path: String, usable_object: UsableObject) -> Self {
        ObjectUse {
            node_index,
            full_module_path,
            usable_object,
        }
    }

    pub fn node_index(&self) -> usize {
        self.node_index
    }

    pub fn full_module_path(&self) -> &String {
        &self.full_module_path
    }

    pub fn usable_object(&self) -> &UsableObject {
        &self.usable_object
    }
}

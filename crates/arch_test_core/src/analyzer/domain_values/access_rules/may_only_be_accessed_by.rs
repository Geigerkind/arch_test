use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MayOnlyBeAccessedBy {
    accessors: HashSet<String>,
    accessed: String,
    when_same_parent: bool,
}

impl MayOnlyBeAccessedBy {
    pub fn new(accessed: String, accessor_layers: HashSet<String>, when_same_parent: bool) -> Self {
        MayOnlyBeAccessedBy {
            accessors: accessor_layers,
            accessed,
            when_same_parent,
        }
    }

    pub fn accessors(&self) -> &HashSet<String> {
        &self.accessors
    }

    pub fn accessed(&self) -> &String {
        &self.accessed
    }

    pub fn when_same_parent(&self) -> bool {
        self.when_same_parent
    }
}

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MayNotBeAccessedBy {
    accessors: HashSet<String>,
    accessed: String,
    when_same_parent: bool,
}

impl MayNotBeAccessedBy {
    pub fn new(accessed: String, accessor_layers: HashSet<String>, when_same_parent: bool) -> Self {
        MayNotBeAccessedBy { accessors: accessor_layers, accessed, when_same_parent }
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
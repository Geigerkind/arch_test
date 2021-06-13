use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct MayNotAccess {
    accessor: String,
    accessed: HashSet<String>,
    when_same_parent: bool,
}

impl MayNotAccess {
    pub fn new(accessor: String, accessed_layers: HashSet<String>, when_same_parent: bool) -> Self {
        MayNotAccess { accessor, accessed: accessed_layers, when_same_parent }
    }

    pub fn accessor(&self) -> &String {
        &self.accessor
    }

    pub fn accessed(&self) -> &HashSet<String> {
        &self.accessed
    }

    pub fn when_same_parent(&self) -> bool {
        self.when_same_parent
    }
}
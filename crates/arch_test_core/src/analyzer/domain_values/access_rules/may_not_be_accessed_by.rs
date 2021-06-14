use std::collections::HashSet;

/// # `Accessed` may not be accessed by `accessors` relation
/// This access rule relation states that the `accessors` layers may not access the specified `accessed` layer.
/// As layer name it attempts to match either the module name or the parent module name, which is the directory the files were placed in.
/// If `when_same_parent` is `true`, the access rule is only applied within the same scope of modules that share the same parent.
#[derive(Debug, Clone)]
pub struct MayNotBeAccessedBy {
    accessors: HashSet<String>,
    accessed: String,
    when_same_parent: bool,
}

impl MayNotBeAccessedBy {
    pub fn new(accessed: String, accessor_layers: HashSet<String>, when_same_parent: bool) -> Self {
        MayNotBeAccessedBy {
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

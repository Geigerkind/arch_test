use std::collections::HashSet;

use crate::analyzer::utils::assert_layer_exist;

#[derive(Debug, Getters, Clone)]
pub struct MayOnlyBeAccessedBy {
    accessors: HashSet<String>,
    accessed: String,
    when_same_parent: bool,
}

impl MayOnlyBeAccessedBy {
    pub fn new(layer_names: &HashSet<String>, accessed: String, accessor_layers: HashSet<String>, when_same_parent: bool) -> Self {
        accessor_layers.iter().for_each(|layer| {
            assert_layer_exist(layer_names, &layer);
        });
        assert_layer_exist(layer_names, &accessed);
        MayOnlyBeAccessedBy { accessors: accessor_layers, accessed, when_same_parent }
    }
}
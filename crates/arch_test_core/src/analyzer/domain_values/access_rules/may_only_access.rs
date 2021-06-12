use std::collections::HashSet;
use crate::analyzer::utils::assert_layer_exist;

#[derive(Debug, Getters)]
pub struct MayOnlyAccess {
    accessor: String,
    accessed: HashSet<String>,
    when_same_parent: bool,
}

impl MayOnlyAccess {
    pub fn new(layer_names: &HashSet<String>, accessor: String, accessed_layers: HashSet<String>, when_same_parent: bool) -> Self {
        assert_layer_exist(layer_names, &accessor);
        accessed_layers.iter().for_each(|layer| {
            assert_layer_exist(layer_names, &layer);
        });
        MayOnlyAccess { accessor, accessed: accessed_layers, when_same_parent }
    }
}
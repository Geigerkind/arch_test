use std::collections::HashSet;
use crate::analyzer::utils::assert_layer_exist;

#[derive(Debug, Getters)]
pub struct MayNotAccess {
    accessor: String,
    accessed: HashSet<String>
}

impl MayNotAccess {
    pub fn new(layer_names: &HashSet<String>, accessor: String, accessed_layers: HashSet<String>) -> Self {
        assert_layer_exist(layer_names, &accessor);
        accessed_layers.iter().for_each(|layer| {
            assert_layer_exist(layer_names, &layer);
        });
        MayNotAccess { accessor, accessed: accessed_layers }
    }
}
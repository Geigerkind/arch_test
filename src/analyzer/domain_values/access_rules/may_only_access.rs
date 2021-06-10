use std::collections::HashSet;
use crate::analyzer::utils::assert_layer_exist;

#[derive(Debug, Getters)]
pub struct MayOnlyAccess {
    accessor: String,
    accessed: HashSet<String>
}

impl MayOnlyAccess {
    pub fn new(layer_names: &HashSet<String>, accessor: &str, accessed_layers: HashSet<&str>) -> Self {
        assert_layer_exist(layer_names, accessor);
        let accessed = accessed_layers.into_iter().map(|layer| {
            assert_layer_exist(layer_names, layer);
            layer.to_owned()
        }).collect::<HashSet<String>>();
        MayOnlyAccess { accessor: accessor.to_owned(), accessed }
    }
}
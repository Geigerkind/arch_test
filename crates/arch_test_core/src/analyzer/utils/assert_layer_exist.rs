use std::collections::HashSet;

pub fn assert_layer_exist(layer_names: &HashSet<String>, layer: &String) {
    assert!(layer_names.contains(layer), "Layer '{}' was not defined!", layer);
}
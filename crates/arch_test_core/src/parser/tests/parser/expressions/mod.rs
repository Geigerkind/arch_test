use crate::parser::services::parse_main_or_mod_file_into_tree;
use std::path::Path;

#[test]
fn record() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/expressions/record.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    // We actively dont parse the record expression
}

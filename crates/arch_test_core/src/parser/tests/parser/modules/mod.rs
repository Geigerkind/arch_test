use crate::parser::services::parse_main_or_mod_file_into_tree;
use std::path::Path;
use crate::parser::domain_values::ObjectType;

#[test]
fn inner_modules() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/modules/inner_modules.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree.len(), 4);
    assert_eq!(node_tree[0].level(), &0);
    assert_eq!(node_tree[0].module_name(), "WAMBO");
    assert_eq!(node_tree[0].children(), &[1, 3]);
    assert_eq!(node_tree[0].usable_objects.len(), 2);
    assert_eq!(node_tree[0].usable_objects[0].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[0].usable_objects[0].object_name, "Test0".to_owned());
    assert_eq!(node_tree[0].usable_objects[1].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[0].usable_objects[1].object_name, "Test0_2".to_owned());

    assert_eq!(node_tree[1].level(), &1);
    assert_eq!(node_tree[1].module_name(), "mod1");
    assert_eq!(node_tree[1].children(), &[2]);
    assert_eq!(node_tree[1].usable_objects.len(), 2);
    assert_eq!(node_tree[1].usable_objects[0].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[1].usable_objects[0].object_name, "Test1".to_owned());
    assert_eq!(node_tree[1].usable_objects[1].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[1].usable_objects[1].object_name, "Test1_2".to_owned());

    assert_eq!(node_tree[2].level(), &2);
    assert_eq!(node_tree[2].module_name(), "mod2");
    assert_eq!(node_tree[2].children(), &[]);
    assert_eq!(node_tree[2].usable_objects.len(), 1);
    assert_eq!(node_tree[2].usable_objects[0].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[2].usable_objects[0].object_name, "Test2".to_owned());

    assert_eq!(node_tree[3].level(), &1);
    assert_eq!(node_tree[3].module_name(), "mod3");
    assert_eq!(node_tree[3].children(), &[]);
    assert_eq!(node_tree[3].usable_objects.len(), 1);
    assert_eq!(node_tree[3].usable_objects[0].object_type(), ObjectType::Struct);
    assert_eq!(node_tree[3].usable_objects[0].object_name, "Test3".to_owned());
}

#[test]
fn nested_folders_and_files() {
    let mut node_tree = Vec::new();
    let path = Path::new("src/parser/tests/parser/modules/nested_folders_and_files/root.rs");
    parse_main_or_mod_file_into_tree(&mut node_tree, path, 0, None, "WAMBO".to_owned());

    assert_eq!(node_tree.len(), 10);
    assert_eq!(node_tree[0].level(), &0);
    assert_eq!(node_tree[0].module_name(), "WAMBO");
    assert_eq!(node_tree[0].children(), &[1, 2, 4, 9]);
    assert_eq!(node_tree[0].usable_objects.len(), 0);

    assert_eq!(node_tree[1].level(), &1);
    assert_eq!(node_tree[1].module_name(), "inner_dir");
    assert_eq!(node_tree[1].children(), &[8]);
    assert_eq!(node_tree[1].usable_objects.len(), 0);

    assert_eq!(node_tree[2].level(), &1);
    assert_eq!(node_tree[2].module_name(), "dir_a");
    assert_eq!(node_tree[2].children(), &[3]);
    assert_eq!(node_tree[2].usable_objects.len(), 0);

    assert_eq!(node_tree[3].level(), &2);
    assert_eq!(node_tree[3].module_name(), "file_2");
    assert_eq!(node_tree[3].children(), &[]);
    assert_eq!(node_tree[3].usable_objects.len(), 0);

    assert_eq!(node_tree[4].level(), &1);
    assert_eq!(node_tree[4].module_name(), "dir_b");
    assert_eq!(node_tree[4].children(), &[5, 7]);
    assert_eq!(node_tree[4].usable_objects.len(), 0);

    assert_eq!(node_tree[5].level(), &2);
    assert_eq!(node_tree[5].module_name(), "dir_c");
    assert_eq!(node_tree[5].children(), &[6]);
    assert_eq!(node_tree[5].usable_objects.len(), 0);

    assert_eq!(node_tree[6].level(), &3);
    assert_eq!(node_tree[6].module_name(), "file_4");
    assert_eq!(node_tree[6].children(), &[]);
    assert_eq!(node_tree[6].usable_objects.len(), 0);

    assert_eq!(node_tree[7].level(), &2);
    assert_eq!(node_tree[7].module_name(), "file_3");
    assert_eq!(node_tree[7].children(), &[]);
    assert_eq!(node_tree[7].usable_objects.len(), 0);

    assert_eq!(node_tree[8].level(), &2);
    assert_eq!(node_tree[8].module_name(), "file_5");
    assert_eq!(node_tree[8].children(), &[]);
    assert_eq!(node_tree[8].usable_objects.len(), 0);
}
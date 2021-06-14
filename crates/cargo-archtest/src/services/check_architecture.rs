use std::path::Path;

use arch_test_core::ModuleTree;

use crate::services::parse_specification;

pub fn check_architecture(directory_path: &str, check_for_complete_layer_specification: bool) {
    let main_path_str = format!("{}/src/main.rs", directory_path);
    let main_path = Path::new(&main_path_str);
    let root_path = if main_path.exists() && main_path.is_file() {
        format!("{}/src/main.rs", directory_path)
    } else {
        format!("{}/src/lib.rs", directory_path)
    };
    let specification_path = format!("{}/architecture.json", directory_path);
    let specification = parse_specification(Path::new(&specification_path));

    if let Ok(architecture) = specification {
        let module_tree = ModuleTree::new(&root_path);
        if let Err(err) = architecture.validate_access_rules() {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if let Err(err) = architecture.check_access_rules(&module_tree) {
            err.print(&module_tree.tree());
            std::process::exit(1);
        } else if check_for_complete_layer_specification {
            if let Err(err) = architecture.check_complete_layer_specification(&module_tree) {
                err.print(&module_tree.tree());
                std::process::exit(1);
            }
        }
    } else {
        println!(
            "Specification file cant be opened for '{}'.",
            directory_path
        );
    }
}

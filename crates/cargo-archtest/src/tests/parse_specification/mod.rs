use std::path::Path;

use crate::services::parse_specification;

#[test]
fn parse() {
    let _specification =
        parse_specification(Path::new("src/tests/parse_specification/architecture.json")).unwrap();
    // Not exactly sure how to assert it, but the important stuff is that most lines are run through and none panic
}

use crate::services::check_architecture;

#[test]
fn run_check_architecture() {
    check_architecture("src/tests/check_architecture/test_architecture", true);
}

#[derive(Debug)]
pub struct NoParentAccess;

impl NoParentAccess {
    pub fn new() -> Self {
        NoParentAccess
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ParseFailure {
    NotAPath,
    PathIsNotARustDirectory
}
#[derive(Debug, Clone)]
pub enum Failure {
    SpecificationCouldNotBeParsed,
    SpecificationFileCantBeOpened,
}
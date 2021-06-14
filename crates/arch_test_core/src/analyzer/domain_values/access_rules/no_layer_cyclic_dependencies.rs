/// # Forbids cyclic dependencies within the same layer
/// As the same layer everything within the same level and below is considered (with the exception of the root level).
#[derive(Debug, Clone)]
pub struct NoLayerCyclicDependencies;

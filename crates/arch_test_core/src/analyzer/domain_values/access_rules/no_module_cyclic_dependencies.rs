/// # Forbids module level cyclic dependencies
/// This access rule forbids cyclic dependencies between individual modules.
///
/// Example: A uses something from B. B uses something from C and C uses something from A.
#[derive(Debug, Clone)]
pub struct NoModuleCyclicDependencies;

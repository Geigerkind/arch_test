#[derive(Debug, Copy, Clone)]
pub enum RuleViolationType {
    SingleLocation,
    Cycle,
    IncompleteLayerSpecification,
}
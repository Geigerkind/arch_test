#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessRule {
    NoParentAccess,
    NoModuleCyclicDependencies,
    NoLayerCyclicDependencies,
    MayOnlyAccess {
        accessor: String,
        accessed: Vec<String>
    },
    MayNotAccess {
        accessor: String,
        accessed: Vec<String>
    },
}
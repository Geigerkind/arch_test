#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessRule {
    NoParentAccess,
    NoModuleCyclicDependencies,
    NoLayerCyclicDependencies,
    MayOnlyAccess {
        accessor: String,
        accessed: Vec<String>,
        when_same_parent: bool,
    },
    MayNotAccess {
        accessor: String,
        accessed: Vec<String>,
        when_same_parent: bool,
    },
    MayOnlyBeAccessedBy {
        accessors: Vec<String>,
        accessed: String,
        when_same_parent: bool,
    },
    MayNotBeAccessedBy {
        accessors: Vec<String>,
        accessed: String,
        when_same_parent: bool,
    },
}
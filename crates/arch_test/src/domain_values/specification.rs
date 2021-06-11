use crate::domain_values::AccessRule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub layer_names: Vec<String>,
    pub access_rules: Vec<AccessRule>
}
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

/// Necessary metadata for referencing a field in an object
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FieldRef {
    /// The JSON path of the field
    ///
    /// e.g. `spec.template.spec.containers[3].resources.limits.cpu`
    pub path: String,

    /// The current value of the field
    ///
    /// Can be any value - string, number, boolean, array or object
    pub current_value: Option<Value>,

    /// The proposed value of the field to fix an issue
    ///
    /// Can be any value - string, number, boolean, array or object
    pub proposed_value: Option<Value>,
}

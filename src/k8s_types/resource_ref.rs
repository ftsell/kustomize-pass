use serde::{Serialize, Deserialize};

/// Necessary metadata for referencing a Kubernetes object
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRef {
    pub api_version: String,
    pub kind: String,
    pub namespace: Option<String>,
    pub name: String,
}

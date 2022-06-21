use serde::{Deserialize, Serialize};

/// Necessary metadata for referencing a Kubernetes object
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRef {
    /// `apiVersion` of the referenced object
    pub api_version: String,

    /// `kind` of the referenced object
    pub kind: String,

    /// Namespace in which the referenced object lives
    pub namespace: Option<String>,

    /// Name of the referenced object
    pub name: String,
}

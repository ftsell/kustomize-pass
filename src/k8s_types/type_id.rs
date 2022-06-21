use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Information that identifies an encoded object type
///
/// This should be flattened in serialized objects via `#[serde(flatten)]`
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct K8sTypeId<A = String, K = String> {
    /// Kubernetes object apiVersion of this object
    pub api_version: A,

    /// Kind of the kubernetes objects
    pub kind: K,
}

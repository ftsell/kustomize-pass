use crate::k8s_types::{K8sObjectMeta, K8sTypeId};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

/// Type definition for kubernetes secret resources
///
/// See https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/#Secret
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V1Secret {
    /// Type identification information as apiVersion and kind
    #[serde(flatten)]
    pub k8s_type_id: K8sTypeId,

    /// Standard object's metadata
    metadata: K8sObjectMeta,

    /// Data contains the secret data.
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    /// The serialized form of the secret data is a base64 encoded string, representing the arbitrary (possibly non-string) data value here.
    data: Mapping,

    /// Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified).
    /// If not set to true, the field can be modified at any time.
    /// Defaulted to nil.
    immutable: Option<bool>,

    /// Used to facilitate programmatic handling of secret data.
    ///
    /// See https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
    #[serde(rename = "type")]
    secret_type: Option<String>,
}

impl V1Secret {
    /// Create a new Secret object from the given data and configured constants
    pub fn new(
        metadata: K8sObjectMeta,
        data: Mapping,
        immutable: Option<bool>,
        secret_type: Option<String>,
    ) -> Self {
        Self {
            k8s_type_id: K8sTypeId {
                api_version: "v1".to_string(),
                kind: "Secret".to_string(),
            },
            metadata,
            data,
            immutable,
            secret_type,
        }
    }
}

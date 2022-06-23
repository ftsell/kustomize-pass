use crate::k8s_types::{K8sObjectMeta, K8sTypeId, PassSource};
use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The concrete type that is used to configure this application as a KRM function
///
/// The content of this object is similar to the [Kubernetes Secret definition](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/)
/// except that the meaning of the `data` field is different because all values are retrieved from pass.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct V1Beta1PassSecret {
    /// Type identification information as apiVersion and kind
    #[serde(flatten)]
    pub k8s_type_id: K8sTypeId<ApiVersion, Kind>,

    /// Standard object's metadata
    pub metadata: K8sObjectMeta,

    /// Reference to the store from which data of this secret should be retrieved
    #[serde(default)]
    pub source: PassSource,

    /// Data contains the secret data references.
    ///
    /// Keys will be copied to the resulting kubernetes secret object while values will be retrieved from pass.
    /// This works by using the value of **this** object as the name of the entry in pass.
    ///
    /// Each key must consist of alphanumeric characters, '-', '_' or '.'.
    pub data: BTreeMap<String, String>,

    /// Immutable, if set to true, ensures that data stored in the Secret cannot be updated (only object metadata can be modified).
    /// If not set to true, the field can be modified at any time.
    /// Defaulted to nil.
    pub immutable: Option<bool>,

    /// Used to facilitate programmatic handling of secret data.
    ///
    /// See https://kubernetes.io/docs/concepts/configuration/secret/#secret-types
    #[serde(rename = "type")]
    pub secret_type: Option<String>,
}

/// Possible values for `PassSecret`s apiVersion field
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
pub enum ApiVersion {
    #[serde(rename = "ftsell.de/v1beta1")]
    FtsellDeV1beta1,
}

/// Possible values for `PassSecrets`s kind field
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
pub enum Kind {
    #[serde(rename = "PassSecret")]
    PassSecret,
}

use crate::k8s_types::{K8sObjectMeta, K8sTypeId};
use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

/// The concrete type that is used to configure this application as a KRM function
///
/// The content of this object is similar to the [Kubernetes Secret definition](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/)
/// except that the meaning of the `data` field is different because all values are retrieved from pass.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct V1Beta1PassSecret {
    #[serde(flatten)]
    pub k8s_type_id: K8sTypeId,

    pub metadata: K8sObjectMeta,

    pub data: Mapping,

    pub immutable: Option<bool>,

    #[serde(rename = "type")]
    pub secret_type: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiVersion {
    #[serde(rename = "ftsell.de/v1beta1")]
    FtsellDeV1beta1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Kind {
    #[serde(rename = "PassSecret")]
    PassSecret,
}

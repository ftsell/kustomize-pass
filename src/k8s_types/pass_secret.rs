use crate::k8s_types::{K8sObjectMeta, K8sTypeId};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

/// The concrete type that is used to configure this application as a KRM function
///
/// The content of this object is similar to the [Kubernetes Secret definition](https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/)
/// except that the meaning of the `data` field is different because all values are retrieved from pass.
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
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

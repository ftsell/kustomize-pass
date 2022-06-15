use crate::k8s_types::{K8sObjectMeta, K8sTypeId};
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

/// Type definition for kubernetes secret resources
///
/// See https://kubernetes.io/docs/reference/kubernetes-api/config-and-storage-resources/secret-v1/
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct V1Secret {
    #[serde(flatten)]
    pub k8s_type_id: K8sTypeId,
    
    metadata: K8sObjectMeta,
    
    data: Mapping,
    
    immutable: Option<bool>,
    
    #[serde(rename = "type")]
    secret_type: Option<String>,
}

impl V1Secret {
    pub fn new(metadata: K8sObjectMeta, data: Mapping, immutable: Option<bool>, secret_type: Option<String>) -> Self {
        Self {
            k8s_type_id: K8sTypeId {
                api_version: "v1".to_string(),
                kind: "Secret".to_string()
            },
            metadata,
            data,
            immutable,
            secret_type
        }
    }
}

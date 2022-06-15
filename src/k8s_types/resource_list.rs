use crate::k8s_types::krm_result::KrmResult;
use crate::k8s_types::K8sTypeId;
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

/// The input wire format for KRM functions
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct V1ResourceList {
    #[serde(flatten)]
    pub k8s_type_id: K8sTypeId,

    ///  A list of Kubernetes objects:
    ///  https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#types-kinds).
    ///
    ///  A function will read this field in the input ResourceList and populate
    ///  this field in the output ResourceList.
    pub items: Vec<Value>,

    /// An optional Kubernetes object for passing arguments to a
    /// function invocation.
    ///
    /// **Note:** While the type definition allows this to be empty, this KRM plugin always requires a
    /// configuration. Thus, this field is later ensured to be set.
    pub function_config: Option<Mapping>,

    /// An optional list that can be used by KRM functions to emit results for observability and debugging purposes
    pub results: Option<Vec<KrmResult>>,
}

impl V1ResourceList {
    pub fn new(items: Vec<Value>) -> Self {
        Self {
            k8s_type_id: K8sTypeId {
                api_version: "config.kubernetes.io/v1".to_string(),
                kind: "ResourceList".to_string(),
            },
            results: None,
            function_config: None,
            items,
        }
    }
}

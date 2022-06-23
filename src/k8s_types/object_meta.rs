use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Standard object's metadata
///
/// See https://kubernetes.io/docs/reference/kubernetes-api/common-definitions/object-meta/
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct K8sObjectMeta {
    /// Name must be unique within a namespace.
    /// Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically.
    /// Name is primarily intended for creation idempotence and configuration definition.
    /// Cannot be updated.
    ///
    /// More info: http://kubernetes.io/docs/user-guide/identifiers#names
    pub name: String,

    /// Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
    ///
    /// Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Map of string keys and values that can be used to organize and categorize (scope and select) objects.
    /// May match selectors of replication controllers and services.
    ///
    /// More info: http://kubernetes.io/docs/user-guide/labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,

    /// Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata.
    /// They are not queryable and should be preserved when modifying objects.
    ///
    /// More info: http://kubernetes.io/docs/user-guide/annotations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
}

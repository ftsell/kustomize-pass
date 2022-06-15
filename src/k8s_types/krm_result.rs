use serde::{Serialize, Deserialize};
use serde_yaml::Mapping;
use crate::k8s_types::{FileRef, ResourceRef};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct KrmResult {
    /// A human readable message
    pub message: String,

    /// The severity of the result
    ///
    /// One of:
    /// - `error` (interpreted as error if not given)
    /// - `warning`
    /// - `info`
    pub severity: Option<String>,

    /// The metadata for referencing a Kubernetes object associated with a result
    pub resource_ref: Option<ResourceRef>,

    /// The reference to a field in the object
    /// If defined, `resource_ref` must also be provided
    pub field: Option<String>,

    /// The reference to a file containing the resource
    pub file: Option<FileRef>,

    /// An unstructured key value map stored with a result that may be set by external tools to store and
    /// retrieve arbitrary metadata
    pub tags: Option<Mapping>,
}
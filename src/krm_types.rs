//! Types defined by the [KRM Function Specification](https://github.com/kubernetes-sigs/kustomize/blob/master/cmd/config/docs/api-conventions/functions-spec.md)
//! used for program input and output
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

/// The input wire format for KRM functions
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InputResourceList {
    /// Always `config.kubernetes.io/v1` or `config.kubernetes.io/v1beta1`
    api_version: String,

    /// Always `ResourceList`
    kind: String,

    ///  A list of Kubernetes objects:
    ///  https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#types-kinds).
    ///
    ///  A function will read this field in the input ResourceList and populate
    ///  this field in the output ResourceList.
    items: Vec<String>,

    /// An optional Kubernetes object for passing arguments to a
    /// function invocation.
    function_config: Option<String>,
}

/// The Output wire format for KRM functions
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputResourceList {
    /// Always `config.kubernetes.io/v1` or `config.kubernetes.io/v1beta1`
    api_version: String,

    /// Always `ResourceList`
    kind: String,

    ///  A list of Kubernetes objects:
    ///  https://github.com/kubernetes/community/blob/master/contributors/devel/sig-architecture/api-conventions.md#types-kinds).
    ///
    ///  A function will read this field in the input ResourceList and populate
    ///  this field in the output ResourceList.
    items: Vec<Value>,

    /// An optional list that can be used by KRM functions to emit results for observability and debugging purposes
    results: Option<Vec<KrmResult>>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KrmResult {
    /// A human readable message
    message: String,

    /// The severity of the result
    ///
    /// One of:
    /// - `error` (interpreted as error if not given)
    /// - `warning`
    /// - `info`
    severity: Option<String>,

    /// The metadata for referencing a Kubernetes object associated with a result
    resource_ref: Option<KubernetesResourceRef>,

    /// The reference to a field in the object
    /// If defined, `resource_ref` must also be provided
    field: Option<String>,

    /// The reference to a file containing the resource
    file: Option<FileRef>,

    /// An unstructured key value map stored with a result that may be set by external tools to store and
    /// retrieve arbitrary metadata
    tags: Option<Mapping>,
}

/// Necessary metadata for referencing a Kubernetes object
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesResourceRef {
    api_version: String,
    kind: String,
    namespace: Option<String>,
    name: String,
}

/// Necessary metadata for referencing a field in an object
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldRef {
    /// The JSON path of the field
    ///
    /// e.g. `spec.template.spec.containers[3].resources.limits.cpu`
    path: String,

    /// The current value of the field
    ///
    /// Can be any value - string, number, boolean, array or object
    current_value: Option<Value>,

    /// The proposed value of the field to fix an issue
    ///
    /// Can be any value - string, number, boolean, array or object
    proposed_value: Option<Value>,
}

/// Necessary metadata for referencing a YAML file in the filesystem
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    /// The OS-agnostic, slash-delimited, relative path
    path: String,

    /// Index of the object in a multi-object YAML file
    index: Option<usize>,
}

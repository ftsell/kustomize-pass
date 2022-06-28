use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Possible behavior which kustomize will use when handling generated resources.
/// Defaults to `create`.
///
/// See the [upstream go definition](https://github.com/kubernetes-sigs/kustomize/blob/master/api/types/generatorargs.go)
#[derive(Copy, Clone, Debug, Hash, Serialize, Deserialize, Eq, PartialEq, JsonSchema)]
pub enum GeneratorBehavior {
    /// Create the resource and fail if another one already exists
    #[serde(rename = "create")]
    Create,

    /// Completely replace an existing resource with the generated one
    #[serde(rename = "replace")]
    Replace,

    /// Try to merge the generated and existing resource
    #[serde(rename = "merge")]
    Merge,
}

impl Default for GeneratorBehavior {
    fn default() -> Self {
        Self::Create
    }
}

impl ToString for GeneratorBehavior {
    fn to_string(&self) -> String {
        match self {
            GeneratorBehavior::Create => "create".to_string(),
            GeneratorBehavior::Replace => "replace".to_string(),
            GeneratorBehavior::Merge => "merge".to_string(),
        }
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct K8sObjectMeta {
    pub name: String,
    pub namespace: Option<String>,
    pub labels: Option<Mapping>,
    pub annotations: Option<Mapping>,
}

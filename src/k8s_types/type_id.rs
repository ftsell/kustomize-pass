use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct K8sTypeId {
    pub api_version: String,
    pub kind: String,
}

use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A reference to the source from which [`PassSecret`](crate::k8s_type::PassSecret) data is retrieved
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
pub enum PassSource {
    /// Use the existing store located at `~/.password-store` or pointed to by environment variable `PASSWORD_STORE_DIR`
    Local,

    /// Use a git repository which contains a password store at its root
    Git(GitPassSource),
}

impl Default for PassSource {
    fn default() -> Self {
        Self::Local
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
pub struct GitPassSource {
    /// Git clone url
    url: String,
}

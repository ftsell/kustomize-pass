use serde::{Serialize, Deserialize};

/// Necessary metadata for referencing a YAML file in the filesystem
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FileRef {
    /// The OS-agnostic, slash-delimited, relative path
    pub path: String,

    /// Index of the object in a multi-object YAML file
    pub index: Option<usize>,
}
use crate::git_util;
use anyhow::Context;
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use directories::ProjectDirs;
use git2::Repository;
use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A reference to the source from which [`PassSecret`](crate::k8s_type::PassSecret) data is retrieved
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
#[serde(untagged)]
pub enum PassSource {
    /// Use the existing store located at `~/.password-store` or pointed to by environment variable `PASSWORD_STORE_DIR`
    Local,

    /// Use a git repository which contains a password store at its root
    Git(GitPassSource),
}

impl PassSource {
    /// Setup the local directory so that it is as intended
    pub(crate) fn setup(&self) -> anyhow::Result<PathBuf> {
        log::debug!("Setting up password store source '{:?}'", self);

        match self {
            PassSource::Local => Ok(libpass::password_store_dir()?),
            PassSource::Git(git_source) => git_source.setup(),
        }
    }
}

impl Default for PassSource {
    fn default() -> Self {
        Self::Local
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, JsonSchema)]
pub struct GitPassSource {
    /// Git clone url
    pub url: String,
}

impl GitPassSource {
    // construct a path to which this repository can be cloned that is unique to it
    fn unique_path(&self) -> anyhow::Result<PathBuf> {
        const SUFFIX_LEN: usize = 12;

        // hash all needed source components to construct a unique digest
        let mut hasher = Blake2bVar::new(SUFFIX_LEN).context(
            "Could not construct digest algorithm for constructing a unique password source path",
        )?;
        hasher.update(self.url.as_bytes());
        let mut digest = [0u8; SUFFIX_LEN];
        hasher
            .finalize_variable(&mut digest)
            .context("Could not finalize digest for constructing a unique password source path")?;

        // append hex-encoded digest to '~/.password-store-'
        let unique_path = "password-store-".to_owned() + &hex::encode(digest);
        let unique_path = ProjectDirs::from("de", "ftsell", "kustomize-pass")
            .context("Could not retrieve project directories")?
            .data_local_dir()
            .join(unique_path);

        log::trace!(
            "Computed unique path for repo {} is {}",
            self.url,
            unique_path.display()
        );
        Ok(unique_path)
    }

    fn setup(&self) -> anyhow::Result<PathBuf> {
        let path = self.unique_path()?;
        match Repository::open(&path) {
            // repo exists, so do fetch
            Ok(repo) => git_util::do_git_pull(&repo).and(Ok(path)),
            // repo does not exist, so clone it
            Err(_) => git_util::do_git_clone(&self.url, &path).and(Ok(path)),
        }
    }
}

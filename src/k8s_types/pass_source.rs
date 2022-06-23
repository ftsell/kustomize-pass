use anyhow::{anyhow, Context};
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;
use directories::UserDirs;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{BranchType, Config, Cred, FetchOptions, RemoteCallbacks, Repository};
use okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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
        let unique_path = ".password-store-".to_owned() + &hex::encode(digest);
        let unique_path = UserDirs::new()
            .context("Could not retrieve user directories")?
            .home_dir()
            .join(unique_path);

        Ok(unique_path)
    }

    fn setup(&self) -> anyhow::Result<PathBuf> {
        let path = self.unique_path()?;
        match Repository::open(&path) {
            // repo exists, so do fetch
            Ok(repo) => {
                do_git_fetch(&repo)?;
                do_git_pull(&repo)?;
                Ok(path)
            }

            // repo does not exist, so clone it
            Err(_) => do_git_clone(&self.url, &path).and(Ok(path)),
        }
    }
}

/// Create git2 fetch options the way it is needed
fn create_fetch_options<'cb>() -> FetchOptions<'cb> {
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|url, username_from_url, _allowed_types| {
        log::debug!(
            "Trying to retrieve credentials for {} from credential helper",
            url
        );
        Cred::credential_helper(&Config::open_default()?, url, username_from_url)
            .or_else(|e| {
                log::debug!(
                    "Could not retrieve credentials from credential helper: {}",
                    e
                );
                log::debug!("Trying to retrieve credentials for {} from ssh agent", url);
                Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
            })
            .or_else(|e| {
                log::debug!("Could not retrieve credentials from ssh agent {}", e);
                log::debug!("Using default (probably unauthenticated) credentials");
                Cred::default()
            })
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    fetch_options
}

/// Do what `git fetch` would do for the main branch
fn do_git_fetch(repo: &Repository) -> anyhow::Result<()> {
    log::debug!("Fetching new content for existing repository");

    repo.find_remote("origin")?
        .fetch(&["main"], Some(&mut create_fetch_options()), None)?;
    Ok(())
}

/// Do the merging part of what `git pull` would do for the main branch
///
/// **Note:** This requires [`do_git_fetch()`] to have been called before.
fn do_git_pull(repo: &Repository) -> anyhow::Result<()> {
    log::debug!("Merging remote branch into local copy");

    // get references to important git objects and analyze merge possibilities
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    let mut main_branch = repo
        .find_branch("main", BranchType::Local)
        .unwrap()
        .into_reference();

    // act according to the analysis
    if analysis.0.is_up_to_date() {
        Ok(())
    } else if analysis.0.is_fast_forward() {
        main_branch.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(main_branch.name().unwrap())?;
        repo.checkout_head(Some(CheckoutBuilder::default().force()))?;

        Ok(())
    } else {
        Err(anyhow!(
            "Could not merge remote branch because only fast-forward merges are supported"
        ))
    }
}

/// Do what `git clone <url> <path>` would do
fn do_git_clone(url: &str, path: &Path) -> anyhow::Result<()> {
    log::debug!("Cloning repository {} to {}", url, path.display());

    match RepoBuilder::new()
        .fetch_options(create_fetch_options())
        .clone(url, path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e).context("Could not clone the repository"),
    }
}

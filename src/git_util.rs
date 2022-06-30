use anyhow::{anyhow, Context};
use directories::UserDirs;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{BranchType, Config, Cred, CredentialType, FetchOptions, RemoteCallbacks, Repository};
use std::env;
use std::path::Path;
use subprocess::Exec;

/// prompt the askpass program given by *exe* for *field* of the given *url*.
fn prompt_git_askpass(exe: &str, field: &str, url: &str) -> Result<String, git2::Error> {
    // run exe and wait for completion
    log::trace!("Executing askpass program: {exe} \"{field} for '{url}'\"");
    let response = Exec::cmd(exe)
        .arg(format!("{field} for '{url}':"))
        .capture()
        .map_err(|e| {
            git2::Error::new(
                git2::ErrorCode::Auth,
                git2::ErrorClass::Callback,
                format!("Could not retrieve credentials from askpass program: {e}"),
            )
        })?;
    if !response.success() {
        return Err(git2::Error::new(
            git2::ErrorCode::Auth,
            git2::ErrorClass::Callback,
            format!(
                "Could not retrieve credentials from askpass program: exit code {:?}",
                response.exit_status
            ),
        ));
    }
    if !response.stderr.is_empty() {
        log::trace!("askpass program stderr: {}", response.stderr_str());
    }

    // post-process response by stripping possible \n at the end
    let response_str = response.stdout_str();
    Ok(response_str
        .strip_suffix('\n')
        .unwrap_or(&response_str)
        .to_string())
}

fn create_username_password_credentials(
    url: &str,
    username_from_url: Option<&str>,
) -> Result<Cred, git2::Error> {
    // credentials from credential-helper
    log::debug!("Trying to use git credentials from credential helper");
    let mut creds = Cred::credential_helper(&Config::open_default()?, url, username_from_url);

    // credentials from GIT_ASKPASS environment variable
    if let Ok(git_askpass) = env::var("GIT_ASKPASS") {
        creds = creds.or_else(|e| {
            log::debug!(
                "Could not retrieve credentials from credential helper: {}",
                e
            );
            log::debug!("Trying to retrieve credentials using program given by GIT_ASKPASS environment variable");

            let username = prompt_git_askpass(&git_askpass, "Username", url)?;
            let password = prompt_git_askpass(&git_askpass, "Password", url)?;
            log::trace!("{username}:{password}");
            Cred::userpass_plaintext(&username, &password)
        })
    }

    creds
}

fn create_ssh_credentials(username_from_url: Option<&str>) -> Result<Cred, git2::Error> {
    // ssh key from agent or from ~/.ssh/id_rsa
    log::debug!("Trying to retrieve git credentials from ssh agent");

    Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")).or_else(|e| {
        log::debug!("Could not retrieve credentials from ssh agent: {}", e);
        log::debug!("Trying to retrieve use the ssh key at ~/.ssh/id_rsa instead");
        let dirs = UserDirs::new().unwrap();
        Cred::ssh_key(
            username_from_url.unwrap_or("git"),
            None,
            &dirs.home_dir().join(".ssh").join("id_rsa"),
            None,
        )
    })
}

/// Create git2 fetch options the way it is needed
fn create_fetch_options<'cb>() -> FetchOptions<'cb> {
    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.credentials(|url, username_from_url, allowed_types| {
        // use credentials from appropriate source
        let mut creds = if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
            create_username_password_credentials(url, username_from_url)
        } else if allowed_types.contains(CredentialType::SSH_KEY) {
            create_ssh_credentials(username_from_url)
        } else {
            log::warn!(
                "Requested key type {:?} is not supported and cannot be supplied. Using default credentials",
                allowed_types
            );
            Cred::default()
        };

        // fall back to default credentials if previous sources were unsuccessful
        creds = creds.or_else(|e| {
            log::debug!("Could not retrieve credentials from other sources: {}", e);
            log::debug!("Using default (probably unauthenticated) credentials");
            Cred::default()
        });

        creds
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
pub(crate) fn do_git_pull(repo: &Repository) -> anyhow::Result<()> {
    do_git_fetch(repo)?;
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
pub(crate) fn do_git_clone(url: &str, path: &Path) -> anyhow::Result<()> {
    log::debug!("Cloning repository {} to {}", url, path.display());

    match RepoBuilder::new()
        .fetch_options(create_fetch_options())
        .clone(url, path)
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e).context("Could not clone the repository"),
    }
}

mod krm_types;

use crate::krm_types::InputResourceList;
use anyhow::Context;
use clap::Parser;
use std::io::stdin;
use std::process::exit;

///
/// Command-Line arguments that are passed to the program
///
#[derive(Parser, Debug)]
#[clap(
    version,
    about = "A kustomize plugin that is able to generate secrets by extracting them from or replace placeholders in other manifests from pass",
    usage = "This program follows the KRM Functions Specification which means that input is passed to it from kustomize via STDIN, output is returned via STDOUT and unstructured messages are returned via STDERR.\n\n    See https://github.com/kubernetes-sigs/kustomize/blob/master/cmd/config/docs/api-conventions/functions-spec.md"
)]
struct CliArgs {}

fn main() {
    let _cli_args = CliArgs::parse();
    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .log_to_stderr()
        .start()
        .expect("Could not initialize logging");

    if let Err(e) = safe_main() {
        log::error!("{:?}", e);
        exit(1)
    }
}

fn safe_main() -> anyhow::Result<()> {
    log::trace!("Reading and parsing ResourceList from stdin");
    let input: InputResourceList =
        serde_yaml::from_reader(stdin()).context("Could not parse ResourceList from stdin")?;
    input.ensure_api_version_and_kind()?;
    log::trace!("Successfully read and parsed ResourceList from stdin");

    log::debug!("{:?}", input);

    Ok(())
}

mod k8s_types;
mod secret_conversion;

use crate::k8s_types::{V1Beta1PassSecret, V1Secret};
use anyhow::Context;
use clap::Parser;
use k8s_types::V1ResourceList;
use serde_yaml::Value;
use std::io::{stdin, stdout};
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
    // parse input
    let input: V1ResourceList =
        serde_yaml::from_reader(stdin()).context("Could not parse ResourceList from stdin")?;
    input.ensure_api_version_kind()?;

    let function_config: V1Beta1PassSecret =
        serde_yaml::from_value(Value::Mapping(input.clone().function_config.unwrap()))
            .context("Could not parse function configuration from input ResourceList")?;
    function_config.ensure_api_version_kind()?;

    // construct preliminary output with items copied from input
    log::debug!("Input items: {:#?}", &input.items);
    let mut output = V1ResourceList::new(input.items);

    // parse function config and handle it by extracting secrets from pass
    let result: V1Secret = function_config.try_into()?;
    let result = serde_yaml::to_value(result)?;
    output.items.push(result);

    // return generated output
    log::debug!("Output items: {:#?}", &output.items);
    log::debug!("{}", serde_yaml::to_string(&output).unwrap());
    serde_yaml::to_writer(stdout(), &output).context("Could not write results to stdout")?;
    Ok(())
}

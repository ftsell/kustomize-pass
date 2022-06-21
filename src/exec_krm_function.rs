use crate::{V1Beta1PassSecret, V1ResourceList, V1Secret};
use anyhow::Context;
use serde_yaml::Value;
use std::io::{stdin, stdout};

pub(crate) fn exec_krm_function() -> anyhow::Result<()> {
    log::debug!("Executing krm function");

    // parse input from stdin
    log::debug!("Parsing input from stdin");
    let input: V1ResourceList =
        serde_yaml::from_reader(stdin()).context("Could not parse ResourceList from stdin")?;
    let function_config: V1Beta1PassSecret =
        serde_yaml::from_value(Value::Mapping(input.clone().function_config.unwrap()))
            .context("Could not parse function configuration from input ResourceList")?;

    // construct preliminary output with items copied from input
    let mut output = V1ResourceList::new(input.items);

    // parse function config and handle it by extracting secrets from pass
    let result: V1Secret = function_config.try_into()?;
    let result = serde_yaml::to_value(result)?;
    output.items.push(result);

    // return generated output
    serde_yaml::to_writer(stdout(), &output).context("Could not write results to stdout")?;

    Ok(())
}

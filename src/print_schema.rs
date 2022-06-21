use crate::V1Beta1PassSecret;
use schemars::schema_for;

pub fn print_json_schema() -> anyhow::Result<()> {
    log::debug!("Generating schema for PassSecret");

    let schema = schema_for!(V1Beta1PassSecret);
    println!("{}", serde_json::to_string_pretty(&schema)?);

    Ok(())
}

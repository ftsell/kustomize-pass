use crate::V1Beta1PassSecret;
use okapi::{map, openapi3};
use schemars::gen::{SchemaGenerator, SchemaSettings};
use schemars::schema_for;

pub fn print_json_schema() -> anyhow::Result<()> {
    log::debug!("Generating schema for PassSecret");
    let schema = schema_for!(V1Beta1PassSecret);

    println!("{}", serde_json::to_string_pretty(&schema)?);
    Ok(())
}

pub fn print_openapi_schema() -> anyhow::Result<()> {
    log::debug!("Generating Schema for PassSecret");
    let mut schema_generator = SchemaGenerator::from(SchemaSettings::openapi3());
    schema_generator.subschema_for::<V1Beta1PassSecret>();
    let mut schema_definitions = schema_generator.take_definitions();

    // apply any modifications from registered visitors
    for visitor in schema_generator.visitors_mut() {
        for schema in schema_definitions.values_mut() {
            visitor.visit_schema(schema)
        }
    }

    // construct the final openapi schema with all necessary metadata
    let openapi_schema = openapi3::OpenApi {
        info: openapi3::Info {
            title: env!("CARGO_PKG_NAME").to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            license: option_env!("CARGO_PKG_LICENSE").map(|license| openapi3::License {
                name: license.to_owned(),
                url: None,
                extensions: map! {},
            }),
            contact: Some(openapi3::Contact {
                name: Some(env!("CARGO_PKG_AUTHORS").to_owned()),
                ..Default::default()
            }),
            description: Some(env!("CARGO_PKG_DESCRIPTION").to_owned()),
            ..Default::default()
        },
        components: Some(openapi3::Components {
            schemas: schema_definitions
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            ..Default::default()
        }),
        ..openapi3::OpenApi::new()
    };

    println!("{}", serde_yaml::to_string(&openapi_schema)?);
    Ok(())
}

//! Types and implementations for clap
use clap::{Parser, Subcommand, ValueEnum};

/// Command-Line arguments that are passed to the program
#[derive(Parser, Debug, Copy, Clone)]
#[clap(
    version,
    about = "A kustomize plugin that is able to generate secrets by extracting them from or replace placeholders in other manifests from pass",
    usage = "This program follows the KRM Functions Specification which means that input is passed to it from kustomize via STDIN, output is returned via STDOUT and unstructured messages are returned via STDERR.\n\n    See https://github.com/kubernetes-sigs/kustomize/blob/master/cmd/config/docs/api-conventions/functions-spec.md"
)]
pub struct CliArgs {
    /// The mode of operation which should be run
    #[clap(subcommand)]
    pub command: Option<CliCommand>,
}

/// Possible modes of operations
#[derive(Subcommand, Debug, Copy, Clone)]
pub enum CliCommand {
    /// Print the JSON schema for configuration types of this krm functions
    PrintSchema {
        /// The format in which the schema should be printed
        #[clap(value_enum, short = 'f', long = "format", default_value = "openapi")]
        format: SchemaFormat,
    },
    /// Execute the main kustomize plugin functionality (default)
    ExecPlugin,
}

/// Possible formats in which schemas can be printed
#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum SchemaFormat {
    /// [json-schema](https://json-schema.org/)
    JsonSchema,
    /// [Openapi 3](https://www.openapis.org/)
    Openapi,
}

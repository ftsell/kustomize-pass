mod exec_krm_function;
mod k8s_types;
mod print_schema;
mod secret_conversion;

pub use crate::k8s_types::{V1Beta1PassSecret, V1Secret};
use clap::{Parser, Subcommand, ValueEnum};
use k8s_types::V1ResourceList;
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
struct CliArgs {
    #[clap(subcommand)]
    command: Option<CliCommand>,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
    /// Print the JSON schema for configuration types of this krm functions
    PrintSchema {
        #[clap(value_enum, short = 'f', long = "format", default_value = "openapi")]
        format: SchemaFormat,
    },
    /// Execute the main kustomize plugin functionality (default)  
    ExecPlugin,
}

#[derive(ValueEnum, Copy, Clone, Debug)]
enum SchemaFormat {
    JsonSchema,
    Openapi,
}

fn main() {
    let cli_args = CliArgs::parse();
    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .log_to_stderr()
        .start()
        .expect("Could not initialize logging");

    let result = match cli_args.command {
        None => exec_krm_function::exec_krm_function(),
        Some(cmd) => match cmd {
            CliCommand::ExecPlugin => exec_krm_function::exec_krm_function(),
            CliCommand::PrintSchema { format } => match format {
                SchemaFormat::JsonSchema => print_schema::print_json_schema(),
                SchemaFormat::Openapi => todo!(),
            },
        },
    };

    if let Err(e) = result {
        log::error!("{:?}", e);
        exit(1)
    }
}

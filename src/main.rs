//! kustomize generator and transformer plugin for pass managed secrets

#![deny(unsafe_code)]
#![warn(
    clippy::all,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_lifetimes,
    unused_qualifications
)]

pub mod cli;
mod exec_krm_function;
pub mod k8s_types;
mod print_schema;
mod secret_conversion;

use crate::k8s_types::{V1Beta1PassSecret, V1Secret};
use clap::Parser;
use cli::*;
use k8s_types::V1ResourceList;
use std::process::exit;

fn main() {
    let cli_args = CliArgs::parse();
    flexi_logger::Logger::try_with_env_or_str("info")
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
                SchemaFormat::Openapi => print_schema::print_openapi_schema(),
            },
        },
    };

    if let Err(e) = result {
        log::error!("{:?}", e);
        exit(1)
    }
}

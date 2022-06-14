mod krm_types;

use clap::Parser;

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
    flexi_logger::Logger::with(flexi_logger::LogSpecification::info())
        .log_to_stderr()
        .start()
        .expect("Could not initialize logging");
}

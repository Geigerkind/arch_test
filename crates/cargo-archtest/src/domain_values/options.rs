use crate::domain_values::Command;

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(
        short,
        long,
        default_value = "src/main.rs",
        about = "Path to the crate root file",
        help = "Path to the crate root file"
    )]
    pub input: String,

    #[structopt(
        short,
        long,
        default_value = "architecture.json",
        about = "Specification file of the architecture",
        help = "Specification file of the architecture"
    )]
    pub specification: String,

    #[structopt(
        short,
        long,
        about = "Compares layers found with provided",
        help = "Compares layers found with provided"
    )]
    pub check_for_complete_layer_specification: bool,

    #[structopt(subcommand)]
    cmd: Command
}

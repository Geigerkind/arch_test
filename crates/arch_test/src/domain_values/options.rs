use std::path::PathBuf;

#[derive(StructOpt)]
pub struct Options {
    #[structopt(short, long, parse(from_os_str), default_value = "src/main.rs",
    about = "Path to the crate root file", help = "Path to the crate root file")]
    input: PathBuf,

    #[structopt(short, long, parse(from_os_str), default_value = "architecture.json",
    about = "Specification file of the architecture", help = "Specification file of the architecture")]
    specification: PathBuf,
}
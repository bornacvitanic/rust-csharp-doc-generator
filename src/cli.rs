use std::path::PathBuf;
use structopt_derive::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub package_dir: PathBuf,
    #[structopt(parse(from_os_str))]
    pub template_file: PathBuf,
    #[structopt(parse(from_os_str))]
    pub output_dir: PathBuf,
}
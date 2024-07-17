use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    package_dir: PathBuf,
    #[structopt(parse(from_os_str))]
    template_file: PathBuf,
    #[structopt(parse(from_os_str))]
    output_dir: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    println!("Package directory {:?}", args.package_dir);
    println!("Template file {:?}", args.template_file);
    println!("Output directory: {:?}", args.output_dir)
}

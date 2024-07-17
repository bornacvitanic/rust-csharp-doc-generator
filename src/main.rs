use structopt::StructOpt;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    package_dir: PathBuf,
    #[structopt(parse(from_os_str))]
    template_file: PathBuf,
    #[structopt(parse(from_os_str))]
    output_dir: PathBuf,
}

fn find_cs_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut cs_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok){
        if entry.path().extension().map_or(false, |ext| ext == "cs") {
            cs_files.push(entry.path().to_path_buf())
        }
    }

    cs_files
}

fn main() {
    let args = Cli::from_args();
    println!("Package directory {:?}", args.package_dir);
    println!("Template file {:?}", args.template_file);
    println!("Output directory: {:?}", args.output_dir);

    let cs_files = find_cs_files(&args.package_dir);
    for file in cs_files {
        println!("Found C# file: {:?}", file);
    }
}

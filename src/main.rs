use structopt::StructOpt;

use crate::cli::Cli;
use crate::documentation::{generate_documentation, load_template};
use crate::parser::{find_cs_files, parse_cs_files};

mod cli;
mod documentation;
mod parser;

fn main() {
    let args = Cli::from_args();
    println!("Package directory: {:?}", args.package_dir);
    println!("Template file: {:?}", args.template_file);
    println!("Output directory: {:?}", args.output_dir);
    println!("Output file: {:?}", args.output_file);

    let cs_files = find_cs_files(&args.package_dir);
    let constructs = parse_cs_files(cs_files);

    // Load the template
    let template = match load_template(&args.template_file) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load template: {}", e);
            return;
        }
    };

    // Generate the documentation
    if let Err(e) =
        generate_documentation(constructs, &template, &args.output_dir, &args.output_file)
    {
        eprintln!("Failed to generate documentation: {}", e);
    }
}

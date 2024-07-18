mod cli;
mod parser;

use std::string::String;
use structopt::StructOpt;
use std::io::Read;
use serde::Serialize;
use crate::cli::Cli;
use crate::parser::{ConstructInfo, filter_constructs_by_variant, find_cs_files, parse_cs_files};

fn main() {
    let args = Cli::from_args();
    println!("Package directory: {:?}", args.package_dir);
    println!("Template file: {:?}", args.template_file);
    println!("Output directory: {:?}", args.output_dir);

    let cs_files = find_cs_files(&args.package_dir);
    let constructs = parse_cs_files(cs_files);

    let construct_variants = [
        ConstructInfo::Class { name: String::new() },
        ConstructInfo::Struct { name: String::new() },
        ConstructInfo::Enum { name: String::new() },
        ConstructInfo::Interface { name: String::new() },
    ];

    for variant in &construct_variants {
        match variant {
            ConstructInfo::Class { .. } => println!("Classes:"),
            ConstructInfo::Struct { .. } => println!("Structs:"),
            ConstructInfo::Enum { .. } => println!("Enums:"),
            ConstructInfo::Interface { .. } => println!("Interfaces:"),
        }

        let filtered_constructs = filter_constructs_by_variant(&constructs, variant);
        for construct in filtered_constructs {
            println!("{:?}", construct);
        }
        println!();
    }
}

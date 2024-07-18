use std::string::String;
use structopt::StructOpt;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;
use serde::Serialize;
use regex::Regex;
use std::collections::HashSet;

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

#[derive(Debug, Serialize)]
enum ConstructInfo {
    Class { name: String },
    Struct { name: String },
    Enum { name: String },
    Interface { name: String },
}


fn extract_definition(line: &str, keyword: &str) -> Option<String> {
    let pattern = format!(r"\b{}\s+(\w+)", keyword);
    let re = Regex::new(&pattern).unwrap();

    if let Some(captures) = re.captures(line) {
        return Some(captures[1].to_string());
    }
    None
}

fn parse_cs_files(files: Vec<PathBuf>) -> Vec<ConstructInfo> {
    let mut constructs = Vec::new();
    let mut seen_partial_classes = HashSet::new();
    let mut inside_multiline_comment = false;

    for file_path in files {
        let mut file_content = String::new();
        if let Ok(mut file) = File::open(&file_path) {
            if file.read_to_string(&mut file_content).is_err() {
                eprintln!("Error reading file: {:?}", file_path);
                continue;
            }
        } else {
            eprintln!("Error opening file: {:?}", file_path);
            continue;
        }

        for line in file_content.lines() {
            let line = line.trim();

            if comment_detected(line, &mut inside_multiline_comment) {
                continue;
            }

            if let Some(name) = extract_definition(line, "class") {
                if seen_partial_classes.insert(name.clone()) {
                    constructs.push(ConstructInfo::Class { name });
                }
            } else if let Some(name) = extract_definition(line, "struct") {
                constructs.push(ConstructInfo::Struct { name });
            } else if let Some(name) = extract_definition(line, "enum") {
                constructs.push(ConstructInfo::Enum { name });
            } else if let Some(name) = extract_definition(line, "interface") {
                constructs.push(ConstructInfo::Interface { name });
            }
        }
    }

    constructs
}

fn comment_detected(line: &str, inside_multiline_comment: &mut bool) -> bool {
    if *inside_multiline_comment {
        if line.contains("*/") {
            *inside_multiline_comment = false;
        }
        return true;
    }
    if line.contains("/*") {
        *inside_multiline_comment = true;
        return true;
    }

    // Skip single-line and XML documentation comments
    if line.starts_with("//") || line.starts_with("///") {
        return true;
    }

    // Skip lines with comments before keywords
    if line.contains("//") && line.find("//").unwrap() < line.find("class").unwrap_or(usize::MAX) {
        return true;
    }
    false
}


fn filter_constructs_by_variant<'a>(constructs: &'a [ConstructInfo], variant: &ConstructInfo) -> Vec<&'a ConstructInfo> {
    constructs.iter().filter(|&construct| {
        match (construct, variant) {
            (ConstructInfo::Class { .. }, ConstructInfo::Class { .. }) => true,
            (ConstructInfo::Struct { .. }, ConstructInfo::Struct { .. }) => true,
            (ConstructInfo::Enum { .. }, ConstructInfo::Enum { .. }) => true,
            (ConstructInfo::Interface { .. }, ConstructInfo::Interface { .. }) => true,
            _ => false,
        }
    }).collect()
}

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

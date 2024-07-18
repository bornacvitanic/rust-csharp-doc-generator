use std::string::String;
use structopt::StructOpt;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::fs::File;
use std::io::Read;
use serde::Serialize;

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
struct ClassInfo {
    class_name: String,
    methods: Vec<String>,
}

fn parse_cs_files(files: Vec<PathBuf>) -> Vec<ClassInfo> {
    let mut class_infos = Vec::new();

    for file_path in files {
        let mut file_content = String::new();
        File::open(&file_path).unwrap().read_to_string(&mut file_content).unwrap();

        // Simplified parsing logic (placeholder for actual parser)
        if let Some(class_name) = file_content.lines().find(|line| line.contains("class ")) {
            let class_name = class_name.trim().to_string();
            let methods = file_content.lines()
                .filter(|line| line.contains("void ") || line.contains("int ") || line.contains("string "))
                .map(|line| line.trim().to_string())
                .collect();

            class_infos.push(ClassInfo {
                class_name,
                methods,
            });
        }
    }

    class_infos
}

fn main() {
    let args = Cli::from_args();
    println!("Package directory {:?}", args.package_dir);
    println!("Template file {:?}", args.template_file);
    println!("Output directory: {:?}", args.output_dir);

    let cs_files = find_cs_files(&args.package_dir);
    let class_infos = parse_cs_files(cs_files);
    for class_info in class_infos {
        println!("Found Class: {:?}", class_info.class_name);
    }
}

use tera::{Tera, Context};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::parser::ConstructInfo;

pub fn load_template(template_file: &PathBuf) -> Tera {
    let mut template_content = String::new();
    File::open(template_file).unwrap().read_to_string(&mut template_content).unwrap();
    let mut tera = Tera::default();
    tera.add_raw_template("doc_template", &template_content).unwrap();
    tera
}

pub fn generate_documentation(
    constructs: Vec<ConstructInfo>,
    tera: Tera,
    output_dir: &PathBuf,
    output_file: &PathBuf
) {
    let mut context = Context::new();

    let mut classes = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();
    let mut interfaces = Vec::new();

    for construct in constructs {
        match construct {
            ConstructInfo::Class { name } => classes.push(name),
            ConstructInfo::Struct { name } => structs.push(name),
            ConstructInfo::Enum { name } => enums.push(name),
            ConstructInfo::Interface { name } => interfaces.push(name),
        }
    }

    context.insert("classes", &classes);
    context.insert("structs", &structs);
    context.insert("enums", &enums);
    context.insert("interfaces", &interfaces);

    let rendered = tera.render("doc_template", &context).unwrap();

    let output_path = output_dir.join(output_file);
    std::fs::write(output_path, rendered).unwrap();
}
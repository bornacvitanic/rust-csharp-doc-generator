use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::parser::ConstructInfo;

pub fn load_template(template_file: &PathBuf) -> String {
    let mut template_content = String::new();
    File::open(template_file).unwrap().read_to_string(&mut template_content).unwrap();
    template_content
}

fn expand_template(template: &str, placeholder: &str, items: &[String]) -> String {
    let mut expanded_template = String::new();
    for line in template.lines() {
        if line.contains(placeholder) {
            for item in items {
                let expanded_line = line.replace(placeholder, item);
                expanded_template.push_str(&expanded_line);
                expanded_template.push('\n');
            }
        } else {
            expanded_template.push_str(line);
            expanded_template.push('\n');
        }
    }
    expanded_template
}

pub fn generate_documentation(
    constructs: Vec<ConstructInfo>,
    template: &str,
    output_dir: &PathBuf,
    output_file: &PathBuf
) {
    let mut interfaces = Vec::new();
    let mut classes = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();

    for construct in constructs {
        match construct {
            ConstructInfo::Class { name } => classes.push(name),
            ConstructInfo::Struct { name } => structs.push(name),
            ConstructInfo::Enum { name } => enums.push(name),
            ConstructInfo::Interface { name } => interfaces.push(name),
        }
    }

    let expanded_template = expand_template(template, "{{ interface }}", &interfaces);
    let expanded_template = expand_template(&expanded_template, "{{ class }}", &classes);
    let expanded_template = expand_template(&expanded_template, "{{ struct }}", &structs);
    let expanded_template = expand_template(&expanded_template, "{{ enum }}", &enums);

    let output_path = output_dir.join(output_file);
    let mut output_file = File::create(output_path).unwrap();
    output_file.write_all(expanded_template.as_bytes()).unwrap();
}
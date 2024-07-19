use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::parser::{ConstructInfo, ConstructType};

pub fn load_template(template_file: &PathBuf) -> Result<String, io::Error> {
    let mut template_content = String::new();
    let mut file = File::open(template_file)?;
    file.read_to_string(&mut template_content)?;
    Ok(template_content)
}

fn expand_template(template: &str, placeholder: &str, items: &[ConstructInfo], summary_placeholder: &str) -> String {
    let mut expanded_template = String::new();
    for line in template.lines() {
        if line.contains(placeholder) {
            for item in items {
                let summary = item.docstring.clone().unwrap_or_else(|| summary_placeholder.to_string());
                let expanded_line = line.replace(placeholder, &item.name).replace(summary_placeholder, &summary);
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
) -> Result<(), io::Error> {
    let mut interfaces = Vec::new();
    let mut classes = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();

    for construct in constructs {
        match construct.construct_type {
            ConstructType::Class => classes.push(construct),
            ConstructType::Struct => structs.push(construct),
            ConstructType::Enum => enums.push(construct),
            ConstructType::Interface => interfaces.push(construct),
        }
    }

    let expanded_template = expand_template(template, "{{ interface }}", &interfaces, "[one_sentence_summary]");
    let expanded_template = expand_template(&expanded_template, "{{ class }}", &classes, "[one_sentence_summary]");
    let expanded_template = expand_template(&expanded_template, "{{ struct }}", &structs, "[one_sentence_summary]");
    let expanded_template = expand_template(&expanded_template, "{{ enum }}", &enums, "[one_sentence_summary]");

    let output_path = output_dir.join(output_file);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(expanded_template.as_bytes())?;
    Ok(())
}
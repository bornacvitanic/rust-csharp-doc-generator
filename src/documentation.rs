use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use strum::IntoEnumIterator;
use crate::parser::{ConstructInfo, ConstructType};

pub fn load_template(template_file: &PathBuf) -> Result<String, io::Error> {
    let mut template_content = String::new();
    let mut file = File::open(template_file)?;
    file.read_to_string(&mut template_content)?;
    Ok(template_content)
}

fn expand_template(template: &mut String, placeholder: &str, items: &[ConstructInfo], summary_placeholder: &str) -> String{
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
    let mut construct_map: HashMap<ConstructType, Vec<ConstructInfo>> = HashMap::new();

    for construct in constructs {
        construct_map.entry(construct.construct_type.clone()).or_insert_with(Vec::new).push(construct);
    }

    let mut expanded_template= template.to_string();
    for construct in ConstructType::iter() {
        let construct_identifier = format!("{{{{ {} }}}}", construct.as_lowercase());
        expanded_template = expand_template(&mut expanded_template, &*construct_identifier, &construct_map[&construct], "[one_sentence_summary]");
    }

    let output_path = output_dir.join(output_file);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(expanded_template.as_bytes())?;
    Ok(())
}
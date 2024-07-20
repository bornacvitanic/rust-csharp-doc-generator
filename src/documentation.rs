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

fn expand_template(template: &mut String, placeholder: &str, items: &[ConstructInfo], summary_placeholder: &str) -> String {
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
    output_file: &PathBuf,
) -> Result<(), io::Error> {
    let construct_map = categorize_constructs(constructs);

    let mut expanded_template = template.to_string();
    for construct_type in ConstructType::iter() {
        if let Some(constructs) = construct_map.get(&construct_type) {
            let construct_identifier = format!("{{{{ {} }}}}", construct_type.as_lowercase());
            expanded_template = expand_template(&mut expanded_template, &*construct_identifier, constructs, "[one_sentence_summary]");
        }
    }

    let output_path = output_dir.join(output_file);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(expanded_template.as_bytes())?;
    Ok(())
}

fn categorize_constructs(constructs: Vec<ConstructInfo>) -> HashMap<ConstructType, Vec<ConstructInfo>> {
    let mut construct_map: HashMap<ConstructType, Vec<ConstructInfo>> = HashMap::new();

    for construct in constructs {
        construct_map.entry(construct.construct_type.clone()).or_insert_with(Vec::new).push(construct);
    }
    construct_map
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Write;

    use crate::parser::AccessModifier;

    use super::*;

    #[test]
    fn test_load_template() {
        let test_dir = PathBuf::from("_load_template_test_data");
        fs::create_dir_all(&test_dir).unwrap();
        let file_path = test_dir.join("template.md");
        let template_content = "This is a test template.";

        {
            let mut file = File::create(&file_path).unwrap();
            file.write_all(template_content.as_bytes()).unwrap();
        }

        let loaded_template = load_template(&file_path).unwrap();
        assert_eq!(loaded_template, template_content);
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_expand_template() {
        let template = "## Key Interfaces\n- **`{{ interface }}`**: [one_sentence_summary].\n";
        let mut template_str = template.to_string();
        let constructs = vec![
            ConstructInfo {
                name: "PublicInterface".to_string(),
                access_modifier: AccessModifier::Public,
                docstring: Some("Public interface summary".to_string()),
                construct_type: ConstructType::Interface,
            },
        ];

        let expanded_template = expand_template(&mut template_str, "{{ interface }}", &constructs, "[one_sentence_summary]");
        let expected_output = "## Key Interfaces\n- **`PublicInterface`**: Public interface summary.\n";

        assert_eq!(expanded_template, expected_output);
    }
}
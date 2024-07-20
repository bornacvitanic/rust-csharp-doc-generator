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

fn expand_template(template: &str, construct_map: &HashMap<ConstructType, Vec<ConstructInfo>>) -> String {
    let mut expanded_template = String::new();

    for line in template.lines() {
        let mut pass_through_line = true;
        for construct_type in ConstructType::iter() {
            let construct_placeholder = construct_type.as_placeholder();
            if line.contains(&construct_placeholder) {
                if let Some(constructs) = construct_map.get(&construct_type) {
                    pass_through_line = false;
                    for item in constructs {
                        let mut expanded_line = line.replace(&construct_placeholder, &item.name);
                        expanded_line = expanded_line.replace("[summary]", &item.docstring.clone().unwrap_or_else(|| "[summary]".to_string()));
                        expanded_line = expanded_line.replace("[one_sentence_summary]", &substring_until_dot(&item.docstring.clone().unwrap_or_else(|| "[one_sentence_summary]".to_string())).to_string());
                        expanded_template.push_str(&expanded_line);
                        expanded_template.push('\n');
                    }
                }
                break;
            }
        }
        if pass_through_line {
            expanded_template.push_str(line);
            expanded_template.push('\n');
        }
    }
    expanded_template
}

fn substring_until_dot(s: &str) -> &str {
    s.split('.').next().unwrap_or(s)
}

fn categorize_constructs(constructs: Vec<ConstructInfo>) -> HashMap<ConstructType, Vec<ConstructInfo>> {
    let mut construct_map: HashMap<ConstructType, Vec<ConstructInfo>> = HashMap::new();
    for construct in constructs {
        construct_map.entry(construct.construct_type.clone()).or_insert_with(Vec::new).push(construct);
    }
    construct_map
}

pub fn generate_documentation(
    constructs: Vec<ConstructInfo>,
    template: &str,
    output_dir: &PathBuf,
    output_file: &PathBuf,
) -> Result<(), io::Error> {
    let construct_map = categorize_constructs(constructs);
    let expanded_template = expand_template(template, &construct_map);

    let output_path = output_dir.join(output_file);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(expanded_template.as_bytes())?;
    Ok(())
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
        let template = "
        # Documentation

        ## Classes
        - **{{ class }}**: [one_sentence_summary]

        ## Structs
        - **{{ struct }}**: [one_sentence_summary]

        ## Interfaces
        - **{{ interface }}**: [one_sentence_summary]

        ## Enums
        - **{{ enum }}**: [one_sentence_summary]
        ";

        let constructs = vec![
            ConstructInfo { name: "MyClass".to_string(), docstring: Some("This is a class.".to_string()), access_modifier: AccessModifier::Public, construct_type: ConstructType::Class },
            ConstructInfo { name: "MyStruct".to_string(), docstring: Some("This is a struct.".to_string()), access_modifier: AccessModifier::Public, construct_type: ConstructType::Struct },
            ConstructInfo { name: "MyInterface".to_string(), docstring: Some("This is an interface.".to_string()), access_modifier: AccessModifier::Public, construct_type: ConstructType::Interface },
            ConstructInfo { name: "MyEnum".to_string(), docstring: Some("This is an enum.".to_string()), access_modifier: AccessModifier::Public, construct_type: ConstructType::Enum },
        ];

        let construct_map = categorize_constructs(constructs);
        let result = expand_template(template, &construct_map);
        let expected = "
        # Documentation

        ## Classes
        - **MyClass**: This is a class

        ## Structs
        - **MyStruct**: This is a struct

        ## Interfaces
        - **MyInterface**: This is an interface

        ## Enums
        - **MyEnum**: This is an enum
        ";

        assert_eq!(result.trim(), expected.trim());
    }
}
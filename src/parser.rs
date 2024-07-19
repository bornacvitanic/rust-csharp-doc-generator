use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, Serialize)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
    Internal,
}

pub struct ConstructInfo {
    pub docstring: Option<String>,
    pub access_modifier: AccessModifier,
    pub construct_type: ConstructType,
    pub name: String,
}

#[derive(Debug, Serialize, PartialEq, EnumIter)]
pub enum ConstructType {
    Class,
    Struct,
    Enum,
    Interface,
}

impl ConstructType {
    pub fn as_lowercase(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

pub fn find_cs_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut cs_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok){
        if entry.path().extension().map_or(false, |ext| ext == "cs") {
            cs_files.push(entry.path().to_path_buf())
        }
    }

    cs_files
}

pub fn extract_definition(line: &str, keyword: &str) -> Option<String> {
    let pattern = format!(r"\b{}\s+(\w+)", keyword);
    let re = Regex::new(&pattern).unwrap();

    if let Some(captures) = re.captures(line) {
        return Some(captures[1].to_string());
    }
    None
}

pub fn parse_cs_files(files: Vec<PathBuf>) -> Vec<ConstructInfo> {
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

        let mut current_docstring: Option<String> = None;

        for line in file_content.lines() {
            let line = line.trim();

            match extract_docstring(line) {
                Some(doc_line) => {
                    current_docstring = match current_docstring {
                        Some(mut existing) => {
                            existing.push(' ');
                            existing.push_str(&doc_line);
                            Some(existing)
                        },
                        None => Some(doc_line)
                    };
                }
                None => {}
            };

            if comment_detected(line, &mut inside_multiline_comment) {
                continue;
            }

            let access_modifier = extract_access_modifier(line);

            for construct in ConstructType::iter() {
                if let Some(name) = extract_definition(line, &*construct.as_lowercase()) {
                    if construct != ConstructType::Class || seen_partial_classes.insert(name.clone()) {
                        constructs.push(ConstructInfo {
                            docstring: current_docstring.clone(),
                            access_modifier,
                            construct_type: construct,
                            name,
                        });
                        current_docstring = None; // Reset the docstring after use
                        break;
                    }
                }
            }
        }
    }

    constructs
}

fn extract_access_modifier(line: &str) -> AccessModifier {
    let access_modifier_regex = Regex::new(r"(?m)^\s*(public|private|protected|internal)").unwrap();

    if let Some(captures) = access_modifier_regex.captures(line) {
        match captures.get(1).unwrap().as_str() {
            "public" => AccessModifier::Public,
            "private" => AccessModifier::Private,
            "protected" => AccessModifier::Protected,
            "internal" => AccessModifier::Internal,
            _ => AccessModifier::Private,
        }
    } else {
        AccessModifier::Private
    }
}

fn extract_docstring(line: &str) -> Option<String> {
    let docstring_regex = Regex::new(r"(?m)^\s*///\s*(.*)$").unwrap();
    let xml_tag_regex = Regex::new(r"</?[^>]+>").unwrap();
    if let Some(captures) = docstring_regex.captures(line) {
        let doc_line = captures.get(1).unwrap().as_str().to_string();
        let doc_line = xml_tag_regex.replace_all(&doc_line, "").to_string();
        Some(doc_line)
    } else {
        None
    }
}

pub fn comment_detected(line: &str, inside_multiline_comment: &mut bool) -> bool {
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


pub fn filter_constructs_by_variant(constructs: &[ConstructInfo], variant: ConstructType) -> Vec<&ConstructInfo> {
    constructs.iter().filter(|&construct| construct.construct_type == variant).collect()
}
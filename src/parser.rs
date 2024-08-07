use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

use regex::Regex;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::{Display, EnumString};
use walkdir::WalkDir;

#[derive(Debug, Serialize, PartialEq, EnumString, Display, EnumIter)]
#[strum(serialize_all = "snake_case")]
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

#[derive(Debug, Eq, Hash, Clone, Serialize, PartialEq, EnumIter)]
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

    pub fn as_placeholder(&self, suffix: &str) -> String {
        format!("[{}{}]", self.as_lowercase(), suffix)
    }
}

impl AccessModifier {
    pub fn variants_as_regex() -> String {
        let variants: Vec<String> = AccessModifier::iter().map(|v| v.to_string()).collect();
        let pattern = variants.join("|");
        format!(r"(?m)^\s*({})", pattern)
    }
}

pub fn find_cs_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut cs_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
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

    let mut extractor = DocstringExtractor::new();
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

            if let Some(doc_line) = extractor.extract_docstring(line) {
                current_docstring = match current_docstring {
                    Some(mut existing) => {
                        existing.push(' ');
                        existing.push_str(&doc_line);
                        Some(existing)
                    }
                    None => Some(doc_line),
                };
            };

            if skip_due_to_comment(line, &mut inside_multiline_comment) {
                continue;
            }

            let access_modifier = extract_access_modifier(line);

            for construct in ConstructType::iter() {
                if let Some(name) = extract_definition(line, &construct.as_lowercase()) {
                    if construct != ConstructType::Class
                        || seen_partial_classes.insert(name.clone())
                    {
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

pub fn extract_access_modifier(line: &str) -> AccessModifier {
    let access_modifier_regex = Regex::new(&AccessModifier::variants_as_regex()).unwrap();

    if let Some(captures) = access_modifier_regex.captures(line) {
        let modifier_str = captures.get(1).unwrap().as_str();
        AccessModifier::from_str(modifier_str).unwrap_or(AccessModifier::Private)
    } else {
        AccessModifier::Private
    }
}

struct DocstringExtractor {
    in_summary: bool,
    summary_content: String,
}

impl DocstringExtractor {
    fn new() -> Self {
        Self {
            in_summary: false,
            summary_content: String::new(),
        }
    }

    fn extract_docstring(&mut self, line: &str) -> Option<String> {
        let docstring_regex = Regex::new(r"^\s*///\s*(.*)$").unwrap();
        let xml_tag_regex = Regex::new(r"</?[^>]+>").unwrap();

        if let Some(captures) = docstring_regex.captures(line) {
            let doc_line = captures.get(1).unwrap().as_str();

            let opening_tag = "<summary>";
            let closing_tag = "</summary>";

            let has_opening_tag = doc_line.contains(opening_tag);
            let has_closing_tag = doc_line.contains(closing_tag);

            if has_opening_tag && has_closing_tag {
                if let Some(start) = doc_line.find(opening_tag) {
                    if let Some(end) = doc_line.find(closing_tag) {
                        self.summary_content.push_str(&doc_line[start + 9..end]);
                        // Clean the accumulated summary content and return it
                        let cleaned_text = xml_tag_regex
                            .replace_all(&self.summary_content, "")
                            .to_string();
                        self.summary_content.clear(); // Clear the content for the next block
                        return Some(cleaned_text.trim().to_string());
                    }
                }
            } else if has_opening_tag {
                self.in_summary = true;
                if let Some(start) = doc_line.find(opening_tag) {
                    self.summary_content.push_str(&doc_line[start + 9..]); // Append text after <summary>
                }
            } else if has_closing_tag {
                if let Some(end) = doc_line.find(closing_tag) {
                    self.summary_content.push_str(&doc_line[..end]); // Append text before </summary>
                }
                self.in_summary = false;

                // Clean the accumulated summary content and return it
                let cleaned_text = xml_tag_regex
                    .replace_all(&self.summary_content, "")
                    .to_string();
                self.summary_content.clear(); // Clear the content for the next block
                return Some(cleaned_text.trim().to_string());
            } else if self.in_summary {
                if !self.summary_content.is_empty() {
                    self.summary_content.push(' ');
                }
                self.summary_content.push_str(doc_line);
            }
        }

        None
    }
}

pub fn skip_due_to_comment(line: &str, inside_multiline_comment: &mut bool) -> bool {
    let mut result = false;

    if line.contains("/*") {
        *inside_multiline_comment = true;
        result = true;
    }

    if *inside_multiline_comment {
        if line.contains("*/") {
            *inside_multiline_comment = false;
        }
        result = true;
    }

    // Skip single-line and XML documentation comments
    if line.starts_with("//") || line.starts_with("///") {
        result = true;
    }

    // Skip lines with comments before keywords for all construct types
    if line.contains("//") {
        for construct in ConstructType::iter() {
            if let Some(keyword_pos) = line.find(&construct.as_lowercase()) {
                if line.find("//").unwrap() < keyword_pos {
                    return true;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_construct_type_as_lowercase() {
        assert_eq!(ConstructType::Class.as_lowercase(), "class");
        assert_eq!(ConstructType::Struct.as_lowercase(), "struct");
        assert_eq!(ConstructType::Enum.as_lowercase(), "enum");
        assert_eq!(ConstructType::Interface.as_lowercase(), "interface");
    }

    #[test]
    fn test_access_modifier_variants_as_regex() {
        let regex_pattern = AccessModifier::variants_as_regex();
        assert!(regex::Regex::new(&regex_pattern).is_ok());
    }

    #[test]
    fn test_find_cs_files() {
        let test_dir = PathBuf::from("_find_cs_test_data");
        fs::create_dir_all(&test_dir).unwrap();
        File::create(test_dir.join("example1.cs")).unwrap();
        File::create(test_dir.join("example2.cs")).unwrap();

        let cs_files = find_cs_files(&test_dir);
        assert_eq!(cs_files.len(), 2);
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_parse_cs_files() {
        let test_dir = PathBuf::from("_parse_test_data");
        fs::create_dir_all(&test_dir).unwrap();
        fs::write(test_dir.join("example.cs"), "public class MyClass { }").unwrap();

        let cs_files = find_cs_files(&test_dir);
        let constructs = parse_cs_files(cs_files);
        assert_eq!(constructs.len(), 1);
        assert_eq!(constructs[0].name, "MyClass");
        assert_eq!(constructs[0].construct_type, ConstructType::Class);
        fs::remove_dir_all(&test_dir).unwrap();
    }

    #[test]
    fn test_extract_definition() {
        let line = "public class MyClass";
        let keyword = "class";
        let result = extract_definition(line, keyword);
        assert_eq!(result, Some("MyClass".to_string()))
    }

    #[test]
    fn test_extract_access_modifier() {
        assert_eq!(
            extract_access_modifier("public class MyClass"),
            AccessModifier::Public
        );
        assert_eq!(
            extract_access_modifier("private class MyClass"),
            AccessModifier::Private
        );
        assert_eq!(
            extract_access_modifier("protected class MyClass"),
            AccessModifier::Protected
        );
        assert_eq!(
            extract_access_modifier("internal class MyClass"),
            AccessModifier::Internal
        );
    }

    #[test]
    fn test_extract_single_line_docstring() {
        let mut extractor = DocstringExtractor::new();
        let line = "/// <summary>This is a single-line summary</summary>";
        assert_eq!(
            extractor.extract_docstring(line),
            Some("This is a single-line summary".to_string())
        );
    }

    #[test]
    fn test_extract_multi_line_docstring() {
        let mut extractor = DocstringExtractor::new();
        let lines = vec![
            "/// <summary>",
            "/// This is a multi-line",
            "/// summary",
            "/// </summary>",
        ];
        let mut result = None;
        for line in lines {
            result = extractor.extract_docstring(line);
        }
        assert_eq!(result, Some("This is a multi-line summary".to_string()));
    }

    #[test]
    fn test_extract_single_line_inside_multi_line_docstring() {
        let mut extractor = DocstringExtractor::new();
        let lines = vec![
            "/// <summary>This is a single-line summary inside multi-line",
            "/// but not ended yet",
            "/// </summary>",
        ];
        let mut result = None;
        for line in lines {
            result = extractor.extract_docstring(line);
        }
        assert_eq!(
            result,
            Some("This is a single-line summary inside multi-line but not ended yet".to_string())
        );
    }

    #[test]
    fn test_skip_due_to_comment() {
        let mut inside_multiline_comment = false;

        // Test single-line comment
        let line = "// This is a single-line comment";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test XML documentation comment
        let line = "/// This is an XML doc comment";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test multi-line comment start and end on the same line
        let line = "/* This is a multi-line comment */";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test multi-line comment start
        let line = "/* This is a multi-line comment";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(inside_multiline_comment);

        // Test multi-line comment end
        let line = "still in multi-line comment */";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test line with construct type and comment after it
        let line = "public class MyClass // This is a comment";
        assert!(!skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test line with construct type and comment before it
        let line = "// This is a comment before class definition\npublic class MyClass";
        assert!(skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test line with construct type and no comment
        let line = "public class MyClass";
        assert!(!skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        // Test line with construct type and comment before keyword for other construct types
        let line = "public struct MyStruct // Comment";
        assert!(!skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        let line = "public enum MyEnum // Comment";
        assert!(!skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);

        let line = "public interface MyInterface // Comment";
        assert!(!skip_due_to_comment(line, &mut inside_multiline_comment));
        assert!(!inside_multiline_comment);
    }
}

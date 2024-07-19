# Documentation Generator

This project is a Rust-based documentation generator for C# codebases. It scans C# source files, extracts relevant constructs (classes, structs, enums, and interfaces), and generates documentation based on a provided template.

## Features

- Parses C# source files to extract constructs.
- Supports XML doc comments.
- Generates documentation in a customizable format using templates.
- Supports different access modifiers and types of constructs.
- Can be integrated into CI/CD pipelines for automated documentation generation.

## Getting Started

### Prerequisites

- Rust programming language
- Cargo (Rust package manager)

## Usage

1. Prepare a template file (e.g., `template.md`):
   ```md
   # Documentation for [System Name]

   ## Overview
   [Brief overview of the system]

   ## Key Interfaces
    - **{{ interface }}**: [one_sentence_summary].

   ## Main Classes
    - **{{ class }}**: [one_sentence_summary].

   ## Structs
    - **{{ struct }}**: [one_sentence_summary].

   ## Enums
    - **{{ enum }}**: [one_sentence_summary].

   ## Usage
   [Usage examples]
   ```

2. Run the documentation generator:
   ```sh
   cargo run --release -- --package_dir path/to/csharp/code --template_file path/to/template.md --output_dir path/to/output --output_file documentation.md
   ```

### Command Line Options

- `--package_dir`: Directory containing C# source files.
- `--template_file`: Path to the template file.
- `--output_dir`: Directory to save the generated documentation.
- `--output_file`: Name of the generated documentation file.

## Project Structure

```
.
├── src
│   ├── cli.rs
│   ├── parser.rs
│   ├── documentation.rs
│   └── main.rs
├── Cargo.toml
└── README.md
```
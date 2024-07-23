[![Test](https://github.com/bornacvitanic/rust-csharp-doc-generator/actions/workflows/rust.yml/badge.svg)](https://github.com/bornacvitanic/rust-csharp-doc-generator/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/bornacvitanic/rust-csharp-doc-generator/status.svg)](https://deps.rs/repo/github/bornacvitanic/rust-csharp-doc-generator)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Download](https://img.shields.io/badge/download-releases-blue.svg)](https://github.com/bornacvitanic/rust-csharp-doc-generator/releases)

# Documentation Generator

This project is a Rust-based documentation generator for C# codebases. It scans C# source files, extracts relevant
constructs (classes, structs, enums, and interfaces), and generates documentation based on a provided template.

## Features

- Parses C# source files to extract constructs.
- Supports XML doc comments.
- Generates documentation in a customizable format using templates.
- Supports different access modifiers and types of constructs.
- Can be integrated into CI/CD pipelines for automated documentation generation.

## Roadmap

- **Interactive CLI**: Enhance the CLI to offer interactive prompts for users who prefer not to use command-line
  arguments directly.
- **Template Customization**: Allow users to define their own placeholders and rules within the template to support more
  flexible documentation styles.
- **Incremental Parsing**: Implement a feature to only re-parse files that have changed since the last run, improving
  performance for large projects.
- **Multiple Output Formats**: Support generating documentation in various formats such as HTML, PDF, and Markdown to
  accommodate different use cases.
- **Code Examples**: Extract and include code examples from the C# files, providing context and usage examples in the
  generated documentation.
- **Syntax Highlighting**: Integrate syntax highlighting for code snippets within the generated documentation for better
  readability.
- **Versioning**: Support versioned documentation, allowing users to generate and maintain documentation for different
  versions of their codebase.
- **Configuration File**: Allow users to define settings and preferences in a configuration file (e.g., JSON, TOML) for
  more convenient customization.
- **CI/CD Integration**: Provide easy integration with CI/CD pipelines (e.g., GitHub Actions, Travis CI) to automate
  documentation generation on code changes.
- **Documentation Coverage Report**: Generate a report showing the coverage of documentation (e.g., percentage of
  classes, methods, and properties documented).
- **Error Handling and Reporting**: Improve error handling and provide detailed error reports to help users troubleshoot
  issues with their input files or templates.

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
   - **`[interface_name]`**: [summary]
   
   ## Main Classes
   - [access_modifier] **`[class_name]`**: [one_sentence_summary]
   
   ## Structs
   - **`[struct_name]`**: [one_sentence_summary]
   
   ## Enums
   - **`[enum_name]`**: [one_sentence_summary]

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

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgements

- [walkdir](https://docs.rs/walkdir/) - Library for recursive directory traversal
- [serde](https://docs.rs/serde/) - Library for serialization and deserialization
- [regex](https://docs.rs/regex/) - Library for regular expressions
- [strum](https://docs.rs/strum/) - Library for working with enums
- [strum_macros](https://docs.rs/strum_macros/) - Macros for working with enums
- [structopt](https://docs.rs/structopt/) - Library for command-line argument parsing
- [structopt-derive](https://docs.rs/structopt-derive/) - Derive macros for `structopt`

## Contact

- **Email**: [borna.cvitanic@gmail.com](mailto:borna.cvitanic@gmail.com)
- **GitHub Issues**: [GitHub Issues Page](https://github.com/bornacvitanic/rust-csharp-doc-generator/issues)
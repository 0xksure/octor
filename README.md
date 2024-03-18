# Octor 

<img src="./static/a02dca62-d98b-46a9-b72f-49236ac21480.webp" alt="Octor logo" width="200"/>


Welcome to Octor, a versatile command-line tool designed for managing and transforming your markdown files with ease. Whether you're consolidating documentation, preparing content for web publishing, or simply organizing your markdown files, Markdown Merger is here to streamline the process. Built with simplicity and efficiency in mind, this tool supports merging multiple markdown files into a single document and converting markdown to HTML.

## Features
- Merge Markdown Files: Combine multiple markdown documents into one, making it easier to manage and organize your documentation or notes. This feature is perfect for preparing comprehensive guides or consolidating notes.
- Convert Markdown to HTML: Seamlessly convert your markdown files into HTML format, ready for web publishing or further processing into PDFs and other formats.
Configuration Flexibility: Customize your merging process with a configuration file in TOML format. Exclude specific files, define merge order, and more, tailoring the output to your needs.
- Command Line Simplicity: Everything is done through simple command-line instructions, giving you the power to automate and script processes as part of larger workflows.


# Getting Started
First, ensure you have Rust installed on your system to use Octor. Then, follow these steps to get up and running:

### Installation: Clone this repository and build the tool using Cargo, Rust's package manager and build system.


```
cargo build --release
```
Running Markdown Merger: Navigate to the target directory where your markdown files are located and run the tool using the appropriate command for your task.

Merging Markdown Files
To merge multiple markdown files into a single file, use the merge subcommand. Optionally, you can specify a custom output filename and a configuration file for advanced merging options.

```
octor merge --filename "output.md" --config_file "config.toml"
```
If no filename or configuration file is specified, defaults (output.md and octor.toml) are used.

Converting Markdown to HTML
Convert a markdown file into HTML with the convert subcommand. Simply specify the markdown file you wish to convert.

```
octor convert --filename "your_file.md"
```
Verbose Output
For detailed operation logs, especially useful for debugging or understanding the merge/conversion process, add the --verbose flag to any command.

```
octor merge --verbose
```

# Contributing
We welcome contributions! If you have suggestions for improvements, feel free to fork the repository and submit a pull request. For major changes, please open an issue first to discuss what you would like to change.

License
This tool is open-sourced under the MIT License. See the LICENSE file for more details.

Enjoy using Octor for all your markdown processing needs!
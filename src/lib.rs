use comrak::Options;
use log::info;
use serde::Deserialize;
use std::cmp::max_by;
use std::fs::File;
use std::io::{self, Write};
mod string_ext;
use crate::string_ext::StringManip;
use std::{path::PathBuf, sync::Mutex};
use walkdir::WalkDir;
#[derive(Debug, Deserialize)]
pub struct Config {
    pub folder: Folder,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            folder: Folder { ignored: vec![] },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    #[serde(default)]
    pub ignored: Vec<String>,
}
pub fn merge_markdowns(filename: String, config: Config, verbose: bool) -> Result<(), io::Error> {
    let mut markdown_files = WalkDir::new("./")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension().map_or(false, |f| f == "md")
                && !config
                    .folder
                    .ignored
                    .iter()
                    .any(|filter| e.path().to_str().unwrap().contains(filter))
        })
        .map(|e| e.into_path())
        .collect::<Vec<PathBuf>>();
    markdown_files.sort();
    if verbose {
        info!("Markdown files: {:?}", markdown_files);
    }
    let output_file = Mutex::new(File::create(PathBuf::from(filename)).unwrap());
    markdown_files.iter().for_each(|file| {
        let content = std::fs::read_to_string(file).unwrap();
        let mut output = output_file.lock().unwrap();
        if let Some(name) = file.to_str() {
            let split_file_name = name.split('/').collect::<Vec<&str>>();
            let split_file_name_start = max_by(split_file_name.len() - 2, 0, |a, b| a.cmp(b));
            let split_file_name_end = max_by(split_file_name.len() - 1, 0, |a, b| a.cmp(b));
            let header_name = split_file_name[split_file_name_start..split_file_name_end]
                .iter()
                .fold("# ".to_string(), |i, f| {
                    format!("{} {}", i, f.to_string().capitalize_first())
                });

            writeln!(output, "{}", header_name).unwrap();
        }
        writeln!(output, "{}", content).unwrap();
    });
    Ok(())
}

pub fn convert_markdown_to_html(file: String) -> Result<(), io::Error> {
    let md = std::fs::read_to_string(&file).unwrap();
    let html = comrak::markdown_to_html(&md, &Options::default());
    std::fs::write(PathBuf::from(&file).with_extension("html"), html).unwrap();
    Ok(())
}

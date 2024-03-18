use clap::{Parser, Subcommand};

use log::{error, info};
use serde::Deserialize;

use std::error::Error;

use octor::{convert_markdown_to_html, merge_markdowns, Config};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[clap(name = "Markdown Merger", version = "1.0", author = "Moi")]
struct Args {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Merge {
        #[arg(short, long)]
        filename: Option<String>,
        #[arg(short, long)]
        config_file: Option<String>,
    },
    Convert {
        #[arg(short, long)]
        filename: String,
    },
}

fn setup_logger() -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Folder {
    #[serde(default)]
    pub ignored: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    setup_logger()?;
    let args: Args = Args::parse();
    match args.subcmd {
        Some(SubCommand::Merge {
            filename,
            config_file,
        }) => {
            let fname = filename.unwrap_or("output.md".to_string());
            // Remove the file if it exists

            let mut config = Config::default();
            let config_path = config_file.unwrap_or("octor.toml".to_string());
            if PathBuf::from(&config_path).exists() {
                if args.verbose {
                    info!("Using config file: {}", config_path);
                }
                //check extension
                if PathBuf::from(&config_path).extension().unwrap() != "toml" {
                    error!("Config file must be a toml file");
                    return Ok(());
                }
                let config_content = std::fs::read_to_string(&config_path).unwrap();
                config = toml::from_str(&config_content).unwrap();
            }

            if PathBuf::from(&fname).exists() {
                std::fs::remove_file(&fname).unwrap();
            }
            match merge_markdowns(fname, config, args.verbose) {
                Ok(_) => {
                    info!("Markdowns merged successfully");
                    Ok(())
                }
                Err(e) => {
                    error!("Error merging markdowns: {}", e);
                    Ok(())
                }
            }
        }
        None => {
            println!("No subcommand was used");
            Ok(())
        }
        Some(SubCommand::Convert { filename }) => match convert_markdown_to_html(filename) {
            Ok(_) => {
                println!("Markdown converted to PDF successfully");
                Ok(())
            }
            Err(e) => {
                println!("Error converting markdown to PDF: {}", e);
                Ok(())
            }
        },
    }
}

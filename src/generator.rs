use std::io::Error;
use clap::Parser;
use roead::{sarc::Sarc, yaz0};

impl Generator {
    pub fn from_options(options: GeneratorOptions) -> Self {
        Generator {
            path: options.mod_path,
            byte_order: match options.is_switch {
                true => rstb::Endian::Little,
                _ => rstb::Endian::Big,
            },
            input_path: match options.source_file {
                Some(source_file_path) => source_file_path,
                _ => todo!()
            },
            output_path: match options.output_file_path {
                Some(output_file_path) => output_file_path,
                _ => todo!()
            }
        }
    }

    pub fn build(&self) -> Result<(), Error> {
        todo!();
    }

    pub async fn build_async(&self) -> Result<(), Error> {
        todo!();
    }
}

pub struct Generator {
    path: String,
    byte_order: rstb::Endian,
    input_path: String,
    output_path: String,
}

/// CLI tool to generate the RSTB file for BotW mods
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct GeneratorOptions {
    /// The path to your mod folder
    #[arg()]
    pub mod_path: String,

    /// The path to the source RSTB file
    #[arg(short = 's', long)]
    pub source_file: Option<String>,

    /// The platform to generate the RSTB file for
    #[arg(short = 'x', long, default_value_t = false)]
    pub is_switch: bool,

    /// The path to the output RSTB file
    #[arg(short, long)]
    pub output_file_path: Option<String>,

    /// The padding to add around every RSTB value
    #[arg(short, long, default_value_t = 0)]
    pub padding: u32,
}
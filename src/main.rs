use clap::Parser;
use roead::{sarc::Sarc, yaz0};

/// CLI tool to generate the RSTB file for BotW mods
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The path to your mod folder
    #[arg()]
    mod_path: String,

    /// The path to the source RSTB file
    #[arg(short = 's', long)]
    source_file: Option<String>,

    /// The platform to generate the RSTB file for
    #[arg(short = 'x', long, default_value_t = false)]
    is_switch: bool,

    /// The path to the output RSTB file
    #[arg(short, long)]
    output_file_path: Option<String>,

    /// The padding to add around every RSTB value
    #[arg(short, long)]
    padding: u32,
}

fn main() {
    let args: Args = Args::parse();
}

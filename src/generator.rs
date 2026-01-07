use crate::util;
use clap::Parser;
use roead::sarc::Sarc;
use roead::yaz0;
use rstb::{Endian, ResourceSizeTable};
use std::io::{Error, ErrorKind, Result, Write};
use std::path::{Path, PathBuf};
use join_str::jstr;

//noinspection SpellCheckingInspection
const RSTB_PATH: &str = "System/Resource/ResourceSizeTable.product.srsizetable";
const RSTB_FOLDER_PATH: &str = "System/Resource/";

pub struct Generator {
    path: PathBuf,
    byte_order: rstb::Endian,
    rstb: ResourceSizeTable,
    output_path: Option<String>,
    padding: u32,
}

impl Generator {
    pub fn from_options(options: GeneratorOptions) -> Self {
        let byte_order = match options.is_switch {
            true => Endian::Little,
            _ => Endian::Big,
        };

        Generator {
            path: PathBuf::from(&options.mod_path),
            byte_order,
            rstb: Self::get_rstb(&options.mod_path, &options.source_file, byte_order).unwrap(),
            output_path: options.output_file_path,
            padding: options.padding,
        }
    }

    pub fn build(&mut self) -> Result<()> {
        let (content_path, aoc_path) = self.get_content_paths();

        if content_path.exists() {
            self.generate(&content_path, false)?;
        }

        if aoc_path.exists() {
            self.generate(&aoc_path, true)?;
        }

        let mut output_file = std::fs::File::create(match &self.output_path {
            Some(output_path) => {
                let path = PathBuf::from(output_path);
                match path.parent() {
                    Some(dir_name) => std::fs::create_dir_all(dir_name)?,
                    _ => (),
                }

                path
            }
            _ => {
                std::fs::create_dir_all(content_path.join(RSTB_FOLDER_PATH))?;
                content_path.join(RSTB_PATH)
            }
        })?;

        let buffer = self.rstb.to_compressed_binary(self.byte_order);
        output_file.write_all(&buffer)?;

        Ok(())
    }

    fn generate(&mut self, path: &PathBuf, is_aoc: bool) -> Result<()> {
        for dir in std::fs::read_dir(path)? {
            let path = dir?.path();
            if path.is_dir() {
                self.generate(&path, is_aoc)?;
                continue;
            }

            self.insert_file(&path, &path.strip_prefix(&self.path).unwrap(), is_aoc)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn build_async(&mut self) -> Result<()> {
        todo!();
    }

    fn insert_file(&mut self, file_path: &Path, relative_path: &Path, is_aoc: bool) -> Result<()> {
        let canon = &util::canonicalize(relative_path);
        let ext = &canon[canon.rfind('.').unwrap() + 1..];
        let data = &std::fs::read(file_path)?;
        self.insert_file_with_data(data, file_path, canon, ext, is_aoc)
    }

    fn insert_file_with_data(&mut self, data: &[u8], file_path: &Path, canon: &String, ext: &str, is_aoc: bool) -> Result<()> {
        if data.len() <= 8 {
            return Ok(());
        }

        if Self::is_mergable_sarc(file_path, ext, &data) {
            let decomp_data = yaz0::decompress_if(data);
            self.process_archive(&decomp_data, is_aoc)?;
        }

        match ext {
            "pack" | "bgdata" | "txt" | "bgsvdata" | "yml" | "msbt" | "bat" | "ini" | "png" | "bfstm" |
            "py" | "sh" | "rsizetable" | "bflim" | "bflan" | "bflyt" | "bgsh" | "fxparam" |
            "gsh" | "baacl" | "baatn" | "baroc" | "extm" | "agstats" | "stera" | "kybin" | "bffnt" |
            // bars can be calculated, but UKMM and BCML ignore it
            "bars" => return Ok(()),
            _ => (),
        }

        if canon == "Actor/ActorInfo.product.byml" {
            return Ok(());
        }

        let file_name = file_path.to_str().unwrap();
        match rstb::calc::estimate_from_bytes_and_name(&data, file_name, self.byte_order) {
            Some(size) => {
                // println!("{}, {} + 0x{:X}", canon, size, self.padding);
                // println!("{}", canon);
                Ok(self.rstb.set(canon.as_str(), size + self.padding))
            }
            _ => match rstb::calc::estimate_from_size_and_name(data.len(), file_path.to_str().unwrap(), self.byte_order) {
                Some(size) => {
                    // println!("{}, {} + 0x{:X}", canon, size, self.padding);
                    Ok(self.rstb.set(canon.as_str(), size + self.padding))
                }
                _ => {
                    println!("{}", canon);
                    self.rstb.remove(canon.as_str());
                    Ok(())
                }
            },
        }
    }

    fn process_archive(&mut self, data: &[u8], is_aoc: bool) -> Result<()> {
        let archive: Sarc = Sarc::new(data).unwrap();

        for file in archive.files() {
            let name = file.name.unwrap();
            let file_path = match (is_aoc, name.starts_with("Aoc/0010/")) {
                (true, false) => jstr!("Aoc/0010/{name}"),
                _ => name.to_owned(),
            };

            let canon = &util::canonicalize(&file_path.as_str());
            let ext = &canon[canon.rfind('.').unwrap() + 1..];
            self.insert_file_with_data(file.data, Path::new(&file_path), canon, ext, is_aoc)?
        }

        Ok(())
    }

    fn get_content_paths(&self) -> (PathBuf, PathBuf) {
        let (content_path, aoc_path) = util::platform_prefixes(self.byte_order);
        let root = PathBuf::from(&self.path);
        (root.join(content_path), root.join(aoc_path))
    }

    fn get_rstb(mod_path: &String, input_path: &Option<String>, byte_order: Endian) -> Result<ResourceSizeTable> {
        match input_path {
            Some(input_path) => {
                if !std::fs::exists(input_path)? {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "The provided RSTB file path does not exist",
                    ));
                }

                let data = std::fs::read(input_path)?;
                Ok(ResourceSizeTable::from_binary(data).unwrap())
            }
            _ => {
                let (content_path, _) = util::platform_prefixes(byte_order);
                let path = PathBuf::from(mod_path).join(content_path).join(RSTB_PATH);
                if !path.exists() {
                    return Err(Error::new(
                        ErrorKind::NotFound,
                        "No RSTB file could be found in the provided mod. \
                        Please specify the RSTB input path or add the file to the mod",
                    ));
                }

                let data = std::fs::read(path)?;
                Ok(ResourceSizeTable::from_binary(data).unwrap())
            }
        }
    }

    fn is_mergable_sarc(file_name: &Path, ext: &str, data: &[u8]) -> bool {
        if data.len() < 0x40 {
            return false
        }

        if &data[..4] != b"SARC" && &data[0x11..0x15] != b"SARC" {
            return false
        }

        if match ext { "genvb" | "sarc" | "arc" => true, _ => false }  {
            return false
        }

        let stem = file_name.file_stem().unwrap().to_str().unwrap();
        if match stem { "tera_resource.Nin_NX_NVN" | "tera_resource.Cafe_Cafe_GX2" => true, _ => false } {
            return false
        }

        true
    }
}

/// CLI tool to generate the RSTB file for BotW mods
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct GeneratorOptions {
    /// The path to your mod folder
    #[arg()]
    pub mod_path: String,

    /// The path to the source RSTB file
    #[arg(short, long)]
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

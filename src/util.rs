use std::path::Path;

pub const fn platform_prefixes(endian: rstb::Endian) -> (&'static str, &'static str) {
    match endian {
        rstb::Endian::Little => ("01007EF00011E000/romfs", "01007EF00011F001/romfs"),
        rstb::Endian::Big => ("content", "aoc/0010"),
    }
}

pub fn canonicalize(path: impl AsRef<Path>) -> String {
    fn canonicalize(path: &Path) -> String {
        let path = path.to_str().unwrap_or("INVALID_FILENAME");
        let mut canon = path.replace('\\', "/");
        for (k, v) in [
            ("Content/", ""),
            ("content/", ""),
            ("atmosphere/titles/", ""),
            ("atmosphere/contents/", ""),
            ("01007EF00011E000/romfs/", ""),
            ("01007ef00011e000/romfs/", ""),
            ("01007EF00011E001/romfs", "Aoc/0010"),
            ("01007EF00011E002/romfs", "Aoc/0010"),
            ("01007EF00011F001/romfs", "Aoc/0010"),
            ("01007EF00011F002/romfs", "Aoc/0010"),
            ("01007ef00011e001/romfs", "Aoc/0010"),
            ("01007ef00011e002/romfs", "Aoc/0010"),
            ("01007ef00011f001/romfs", "Aoc/0010"),
            ("01007ef00011f002/romfs", "Aoc/0010"),
            ("romfs/", ""),
            ("aoc/content", "Aoc"),
            ("aoc", "Aoc"),
        ]
            .into_iter()
        {
            if canon.starts_with(k) {
                canon = [v, canon.trim_start_matches(k)].concat();
            }
        }
        canon.replace(".s", ".").into()
    }
    canonicalize(path.as_ref())
}
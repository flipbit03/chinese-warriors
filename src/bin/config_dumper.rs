use std::{fs::File, io::Write};

use chinese_warriors::assets::config::structs::CwConfig;
use ron::{extensions::Extensions, ser::PrettyConfig};

fn main() -> std::io::Result<()> {
    let dump_path = "assets/default.config";

    println!("Dumping Default CwConfig to {}", &dump_path);

    let config_as_str = ron::ser::to_string_pretty(
        &CwConfig::default(),
        PrettyConfig::new()
            .decimal_floats(true)
            .extensions(Extensions::UNWRAP_NEWTYPES)
            .separate_tuple_members(true),
    )
    .unwrap();

    let mut file = File::create(dump_path)?;

    file.write_all(config_as_str.as_bytes())?;

    Ok(())
}

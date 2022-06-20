use std::{fs::File, io::Write};

use chinese_warriors::assets::config::structs::{
    CwConfig, DEBUG_CONFIG_PATH, DEFAULT_CONFIG_PATH,
};

use ron::{extensions::Extensions, ser::PrettyConfig};

fn main() -> () {
    // Dump production and debug configs to their appropriate locations
    [
        (DEFAULT_CONFIG_PATH, false, CwConfig::default()),
        (DEBUG_CONFIG_PATH, true, CwConfig::debug_config()),
    ]
    .iter()
    .for_each(|(config_path, debug, cw_config)| {
        let debug_str = match debug {
            true => "Debug ",
            false => "",
        }
        .to_string();
        println!("Dumping {}CwConfig to {}", debug_str, &config_path);

        let config_as_str = ron::ser::to_string_pretty(
            cw_config,
            PrettyConfig::new()
                .decimal_floats(true)
                .struct_names(true)
                .depth_limit(6)
                .indentor("  ".to_ascii_lowercase())
                .extensions(Extensions::all())
                .separate_tuple_members(true),
        )
        .unwrap();

        let mut file = File::create(config_path).unwrap();
        file.write_all(config_as_str.as_bytes()).unwrap();
    });
}

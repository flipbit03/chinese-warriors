use std::{fs::File, io::Write};

use chinese_warriors::{
    assets::config::structs::{CwConfig, DEBUG_CONFIG_PATH, DEFAULT_CONFIG_PATH},
    utilities::config::dump_cwconfig_pretty,
};

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

        let config_as_str = dump_cwconfig_pretty(cw_config);

        let mut file = File::create(config_path).unwrap();
        file.write_all(config_as_str.as_bytes()).unwrap();
    });
}

use std::fs::read_to_string;

use chinese_warriors::{
    assets::config::structs::CwConfig, utilities::config::dump_cwconfig_pretty,
};
use clap::Parser;
use ron::from_str;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct RonPrettyPrintCLIConfig {
    pub input_ron_path: String,
    pub output_ron_path: String,
}

fn main() {
    let config = RonPrettyPrintCLIConfig::parse();

    let cwconfig =
        from_str::<CwConfig>(&read_to_string(config.input_ron_path).unwrap()).unwrap();

    let config_as_str = dump_cwconfig_pretty(&cwconfig);

    std::fs::write(config.output_ron_path, config_as_str).unwrap();
}

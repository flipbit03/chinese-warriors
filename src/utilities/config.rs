use ron::{extensions::Extensions, ser::PrettyConfig};

use crate::assets::config::structs::CwConfig;

pub fn dump_cwconfig_pretty(c: &CwConfig) -> String {
    ron::ser::to_string_pretty(
        c,
        PrettyConfig::new()
            .decimal_floats(true)
            .struct_names(true)
            .depth_limit(6)
            .indentor("  ".to_ascii_lowercase())
            .extensions(Extensions::all())
            .separate_tuple_members(true),
    )
    .unwrap()
}

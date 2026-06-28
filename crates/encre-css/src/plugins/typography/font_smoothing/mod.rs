#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "antialised" => {
            &[
                "-webkit-font-smoothing: antialiased;",
                "-moz-osx-font-smoothing: grayscale;",
            ]
        },
        "subpixel-antialised" => {
            &[
                "-webkit-font-smoothing: auto;",
                "-moz-osx-font-smoothing: auto;",
            ]
        }
    },
});

#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListCases {
    cases: phf_map! {
        "appearance-none" => &[
            "-webkit-appearance: none;",
            "-moz-appearance: none;",
            "appearance: none;",
        ]
    },
});

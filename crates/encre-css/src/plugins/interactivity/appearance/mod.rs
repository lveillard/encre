#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListProperties {
    props: phf_map! {
        "appearance-none" => &[
            "-webkit-appearance: none;",
            "-moz-appearance: none;",
            "appearance: none;",
        ]
    },
});

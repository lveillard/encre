#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("touch-action"),
    values: &[
        "auto",
        "pan-x",
        "pan-left",
        "pan-right",
        "pan-y",
        "pan-up",
        "pan-down",
        "pinch-zoom",
        "manipulation",
        "none",
    ],
});

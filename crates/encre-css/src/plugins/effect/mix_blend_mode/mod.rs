#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("mix-blend-mode"),
    values: phf_map! {
        "mix-blend-normal" => "normal",
        "mix-blend-multiply" => "multiply",
        "mix-blend-screen" => "screen",
        "mix-blend-overlay" => "overlay",
        "mix-blend-darken" => "darken",
        "mix-blend-lighten" => "lighten",
        "mix-blend-color-dodge" => "color-dodge",
        "mix-blend-color-burn" => "color-burn",
        "mix-blend-hard-light" => "hard-light",
        "mix-blend-soft-light" => "soft-light",
        "mix-blend-difference" => "difference",
        "mix-blend-exclusion" => "exclusion",
        "mix-blend-hue" => "hue",
        "mix-blend-saturation" => "saturation",
        "mix-blend-color" => "color",
        "mix-blend-luminosity" => "luminosity",
        "mix-blend-plus-lighter" => "plus-lighter",
    },
});

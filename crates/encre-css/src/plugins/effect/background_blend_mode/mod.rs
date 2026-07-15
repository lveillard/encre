#![doc = include_str!("README.md")]
#![doc(alias("effect", "bg", "background"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("background-blend-mode"),
    values: phf_map! {
        "bg-blend-normal" => "normal",
        "bg-blend-multiply" => "multiply",
        "bg-blend-screen" => "screen",
        "bg-blend-overlay" => "overlay",
        "bg-blend-darken" => "darken",
        "bg-blend-lighten" => "lighten",
        "bg-blend-color-dodge" => "color-dodge",
        "bg-blend-color-burn" => "color-burn",
        "bg-blend-hard-light" => "hard-light",
        "bg-blend-soft-light" => "soft-light",
        "bg-blend-difference" => "difference",
        "bg-blend-exclusion" => "exclusion",
        "bg-blend-hue" => "hue",
        "bg-blend-saturation" => "saturation",
        "bg-blend-color" => "color",
        "bg-blend-luminosity" => "luminosity",
    },
});

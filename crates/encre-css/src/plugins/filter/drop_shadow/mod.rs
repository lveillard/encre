#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-drop-shadow"),
    values: phf_map! {
        "xs" => "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))",
        "sm" => "drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06))",
        "md" => "drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06))",
        "lg" => "drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1))",
        "xl" => "drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08))",
        "2xl" => "drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))",
        "none" => "drop-shadow(0 0 #0000)",
    },
}).extra_lines(&[CSS_FILTER]);

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("--en-drop-shadow"),
    hints: &[],
    matcher: Shadow,
})
.extra_lines(&[CSS_FILTER])
.template("drop-shadow({})");

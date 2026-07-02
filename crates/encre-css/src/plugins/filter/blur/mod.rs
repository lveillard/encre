#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-blur"),
    values: phf_map! {
        "xs" => "blur(4px)",
        "sm" => "blur(8px)",
        "md" => "blur(12px)",
        "lg" => "blur(16px)",
        "xl" => "blur(24px)",
        "2xl" => "blur(40px)",
        "3xl" => "blur(64px)",
        "none" => "blur(0)",
    },
})
.extra_lines(&[CSS_FILTER]);

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("--en-blur"),
    hints: &[],
    matcher: Length,
})
.extra_lines(&[CSS_FILTER])
.template("blur({})");

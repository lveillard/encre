#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("--en-backdrop-blur"),
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
};
// TODO: extra_line CSS_BACKDROP_FILTER

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("--en-backdrop-blur"),
    hints: &[],
    matcher: Length,
};
// TODO: template blur({})
// TODO: extra_line CSS_BACKDROP_FILTER

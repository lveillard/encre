#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use crate::{plugins::filter::CSS_BACKDROP_FILTER, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
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
})
.extra_lines(&CSS_BACKDROP_FILTER);

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("--en-backdrop-blur"),
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("blur({})");

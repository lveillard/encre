#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use crate::{plugins::filter::CSS_BACKDROP_FILTER, prelude::build_plugin::*};

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("--en-backdrop-blur"),
    values: phf_map! {
        "backdrop-blur-xs" => "blur(4px)",
        "backdrop-blur-sm" => "blur(8px)",
        "backdrop-blur-md" => "blur(12px)",
        "backdrop-blur-lg" => "blur(16px)",
        "backdrop-blur-xl" => "blur(24px)",
        "backdrop-blur-2xl" => "blur(40px)",
        "backdrop-blur-3xl" => "blur(64px)",
        "backdrop-blur-none" => "blur(0)",
    },
})
.extra_lines(&CSS_BACKDROP_FILTER);

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "backdrop-blur",
    prop: SingleProp("--en-backdrop-blur"),
})
.extra_lines(&CSS_BACKDROP_FILTER)
.template("blur({})");

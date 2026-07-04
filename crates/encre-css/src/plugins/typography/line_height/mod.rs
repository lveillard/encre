#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("line-height"),
    values: phf_map! {
        "leading-none" => "1",
        "leading-tight" => "1.25",
        "leading-snug" => "1.375",
        "leading-normal" => "1.5",
        "leading-relaxed" => "1.625",
        "leading-loose" => "2",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "leading",
    prop: SingleProp("line-height"),
    has_auto: false,
    has_full: false,
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "leading",
    prop: SingleProp("line-height"),
});

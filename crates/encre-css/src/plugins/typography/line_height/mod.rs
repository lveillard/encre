#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_LIST: StaticPlugin = Plugin::new(PluginKind::ListValues {
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

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "leading",
    prop: SingleProp("line-height"),
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "leading",
    prop: SingleProp("line-height"),
});

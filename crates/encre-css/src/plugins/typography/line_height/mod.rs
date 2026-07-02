#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("line-height"),
    values: phf_map! {
        "none" => "1",
        "tight" => "1.25",
        "snug" => "1.375",
        "normal" => "1.5",
        "relaxed" => "1.625",
        "loose" => "2",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Spacing {
    prop: SingleProp("line-height"),
    has_auto: false,
    has_full: false,
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("line-height"),
    hints: &[],
    matcher: OrMultiple(&[&Custom("normal"), &Number, &Length, &Percentage]),
});

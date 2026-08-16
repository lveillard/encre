#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("border-style"),
    values: map! {
        "border-solid" => "solid",
        "border-dashed" => "dashed",
        "border-dotted" => "dotted",
        "border-double" => "double",
        "border-groove" => "groove",
        "border-ridge" => "ridge",
        "border-inset" => "inset",
        "border-outset" => "outset",
        "border-hidden" => "hidden",
        "border-none" => "none",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "border",
    prop: SingleProp("border-style"),
    hints: Some(&[]),
    matchers: Some((
        &[PluginArbitraryMatcher::LineStyle],
        PluginArbitraryMatcherSeparation::Space,
    )),
    ..Arbitrary::default()
});

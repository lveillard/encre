#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("background-size"),
    values: &["auto", "cover", "contain"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("background-size"),
    hints: &[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage],
    matcher: CommaSeparated(&OrMultiple(&[
        &SpaceSeparated(&OrMultiple(&[&Length, &Percentage, &Custom("auto")])),
        &Custom("cover"),
        &Custom("contain"),
    ])),
});

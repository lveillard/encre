#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("text-underline-offset"),
    values: &["auto"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("text-underline-offset"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("text-underline-offset"),
    hints: &[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage],
    matcher: OrMultiple(&[&Length, &Percentage, &Custom("auto")]),
});

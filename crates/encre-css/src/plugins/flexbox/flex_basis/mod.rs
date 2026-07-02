#![doc = include_str!("README.md")]
#![doc(alias = "flexbox")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prop: SingleProp("flex-basis"),
    has_auto: true,
    has_full: true,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("flex-basis"),
    hints: &[],
    matcher: Length,
});

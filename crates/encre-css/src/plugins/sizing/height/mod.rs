#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Sizing {
    prop: SingleProp("height"),
    is_horizontal: false,
    has_none: false,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("height"),
    hints: &[],
    matcher: Or(&Length, &Percentage),
});

#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Sizing {
    prop: SingleProp("min-width"),
    is_horizontal: true,
    has_none: false,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("min-width"),
    hints: &[],
    matcher: Or(&Length, &Percentage),
});

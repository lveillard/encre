#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prop: PropertyName, template: &'static str) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prop,
        has_empty: false,
        has_negative: false,
        divide_by: 1.0,
    })
    .template(template)
}

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("grid-column"),
    values: &["auto"],
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("grid-column"),
    hints: &[],
    matcher: All,
});

pub(crate) const PLUGIN_SPAN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column"),
    values: phf_map! {
        "full" => "1 / -1",
    },
});
pub(crate) const PLUGIN_SPAN_2: Plugin =
    builtin_plugin(SingleProp("grid-column"), "span {} / span {}");

pub(crate) const PLUGIN_START_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("grid-column-start"),
    values: &["auto"],
});
pub(crate) const PLUGIN_START_2: Plugin = builtin_plugin(SingleProp("grid-column-start"), "{}");

pub(crate) const PLUGIN_END_1: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("grid-column-end"),
    values: &["auto"],
});
pub(crate) const PLUGIN_END_2: Plugin = builtin_plugin(SingleProp("grid-column-end"), "{}");

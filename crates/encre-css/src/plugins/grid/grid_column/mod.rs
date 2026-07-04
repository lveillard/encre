#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

const fn builtin_plugin(prefix: &'static str, prop: PropertyName, template: &'static str) -> Plugin {
    Plugin::new(PluginKind::AnyNumber {
        prefix,
        prop,
        has_empty: false,
        has_negative: false,
        divide_by: 1.0,
    })
    .template(template)
}

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column"),
    values: phf_map! {
        "col-auto" => "auto"
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "col",
    prop: SingleProp("grid-column"),
});

pub(crate) const PLUGIN_SPAN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column"),
    values: phf_map! {
        "col-span-full" => "1 / -1",
    },
});
pub(crate) const PLUGIN_SPAN_2: Plugin =
    builtin_plugin("col-span", SingleProp("grid-column"), "span {} / span {}");

pub(crate) const PLUGIN_START_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column-start"),
    values: phf_map! {
        "col-start-auto" => "auto",
    },
});
pub(crate) const PLUGIN_START_2: Plugin = builtin_plugin("col-start", SingleProp("grid-column-start"), "{}");

pub(crate) const PLUGIN_END_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column-end"),
    values: phf_map! {
        "col-end-auto" => "auto",
    },
});
pub(crate) const PLUGIN_END_2: Plugin = builtin_plugin("col-end", SingleProp("grid-column-end"), "{}");

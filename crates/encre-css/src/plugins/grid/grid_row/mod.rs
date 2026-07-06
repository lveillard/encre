#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

const fn plugin(prefix: &'static str, prop: PropertyName, template: &'static str) -> Plugin {
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
    prop: SingleProp("grid-row"),
    values: phf_map! {
        "row-auto" => "auto",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "row",
    prop: SingleProp("grid-row"),
});

pub(crate) const PLUGIN_SPAN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row"),
    values: phf_map! {
        "row-span-full" => "1 / -1",
    },
});
pub(crate) const PLUGIN_SPAN_2: Plugin =
    plugin("row-span", SingleProp("grid-row"), "span {} / span {}");

pub(crate) const PLUGIN_START_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row-start"),
    values: phf_map! {
        "row-start-auto" => "auto",
    },
});
pub(crate) const PLUGIN_START_2: Plugin = plugin("row-start", SingleProp("grid-row-start"), "{}");

pub(crate) const PLUGIN_END_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row-end"),
    values: phf_map! {
        "row-end-auto" => "auto",
    },
});
pub(crate) const PLUGIN_END_2: Plugin = plugin("row-end", SingleProp("grid-row-end"), "{}");

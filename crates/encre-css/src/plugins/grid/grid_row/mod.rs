#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName, template: &'static str) -> StaticPlugin {
    Plugin::new(PluginKind::Number {
        namespace,
        prop,
    })
    .template(template)
}

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row"),
    values: phf_map! {
        "row-auto" => "auto",
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "row",
    prop: SingleProp("grid-row"),
});

pub(crate) const PLUGIN_SPAN_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row"),
    values: phf_map! {
        "row-span-full" => "1 / -1",
    },
});
pub(crate) const PLUGIN_SPAN_2: StaticPlugin =
    plugin("row-span", SingleProp("grid-row"), "span {} / span {}");

pub(crate) const PLUGIN_START_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row-start"),
    values: phf_map! {
        "row-start-auto" => "auto",
    },
});
pub(crate) const PLUGIN_START_2: StaticPlugin = plugin("row-start", SingleProp("grid-row-start"), "{}");

pub(crate) const PLUGIN_END_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-row-end"),
    values: phf_map! {
        "row-end-auto" => "auto",
    },
});
pub(crate) const PLUGIN_END_2: StaticPlugin = plugin("row-end", SingleProp("grid-row-end"), "{}");

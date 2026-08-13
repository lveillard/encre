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
    prop: SingleProp("grid-column"),
    values: phf_map! {
        "col-auto" => "auto"
    },
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "col",
    prop: SingleProp("grid-column"),
});

pub(crate) const PLUGIN_SPAN_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column"),
    values: phf_map! {
        "col-span-full" => "1 / -1",
    },
});
pub(crate) const PLUGIN_SPAN_2: StaticPlugin =
    plugin("col-span", SingleProp("grid-column"), "span {} / span {}");

pub(crate) const PLUGIN_START_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column-start"),
    values: phf_map! {
        "col-start-auto" => "auto",
    },
});
pub(crate) const PLUGIN_START_2: StaticPlugin = plugin("col-start", SingleProp("grid-column-start"), "{}");

pub(crate) const PLUGIN_END_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-column-end"),
    values: phf_map! {
        "col-end-auto" => "auto",
    },
});
pub(crate) const PLUGIN_END_2: StaticPlugin = plugin("col-end", SingleProp("grid-column-end"), "{}");

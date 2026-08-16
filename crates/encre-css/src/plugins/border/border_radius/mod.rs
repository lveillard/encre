#![doc = include_str!("README.md")]
#![doc(alias("border", "rounded"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn plugin(namespace: &'static str, prop: StaticPropertyName) -> (StaticPlugin, StaticPlugin) {
    (
        Plugin::new(PluginKind::ListValues {
            prop,
            values: map! {
                "none" => "0",
                "xs" => "0.125rem",
                "sm" => "0.25rem",
                "md" => "0.375rem",
                "lg" => "0.5rem",
                "xl" => "0.75rem",
                "2xl" => "1rem",
                "3xl" => "1.5rem",
                "full" => "9999px",
            },
        })
        .namespace(namespace),
        Plugin::new(PluginKind::Arbitrary { namespace, prop })
            .hints(&[])
            .matchers(&[Length, Percentage], PluginArbitraryMatcherSeparation::Space),
    )
}

pub(crate) const PLUGIN: (StaticPlugin, StaticPlugin) = plugin("rounded", SingleProp("border-radius"));
pub(crate) const PLUGIN_START: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-s",
    MultipleProps(&["border-start-start-radius", "border-end-start-radius"]),
);
pub(crate) const PLUGIN_END: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-e",
    MultipleProps(&["border-start-end-radius", "border-end-end-radius"]),
);
pub(crate) const PLUGIN_START_START: (StaticPlugin, StaticPlugin) =
    plugin("rounded-ss", SingleProp("border-start-start-radius"));
pub(crate) const PLUGIN_START_END: (StaticPlugin, StaticPlugin) =
    plugin("rounded-se", SingleProp("border-start-end-radius"));
pub(crate) const PLUGIN_END_END: (StaticPlugin, StaticPlugin) =
    plugin("rounded-ee", SingleProp("border-end-end-radius"));
pub(crate) const PLUGIN_END_START: (StaticPlugin, StaticPlugin) =
    plugin("rounded-es", SingleProp("border-end-start-radius"));
pub(crate) const PLUGIN_TOP_RIGHT: (StaticPlugin, StaticPlugin) =
    plugin("rounded-tr", SingleProp("border-top-right-radius"));
pub(crate) const PLUGIN_TOP_LEFT: (StaticPlugin, StaticPlugin) =
    plugin("rounded-tl", SingleProp("border-top-left-radius"));
pub(crate) const PLUGIN_BOTTOM_RIGHT: (StaticPlugin, StaticPlugin) =
    plugin("rounded-br", SingleProp("border-bottom-right-radius"));
pub(crate) const PLUGIN_BOTTOM_LEFT: (StaticPlugin, StaticPlugin) =
    plugin("rounded-bl", SingleProp("border-bottom-left-radius"));
pub(crate) const PLUGIN_TOP: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-t",
    MultipleProps(&["border-top-left-radius", "border-top-right-radius"]),
);
pub(crate) const PLUGIN_BOTTOM: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-b",
    MultipleProps(&["border-bottom-left-radius", "border-bottom-right-radius"]),
);
pub(crate) const PLUGIN_LEFT: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-l",
    MultipleProps(&["border-top-left-radius", "border-bottom-left-radius"]),
);
pub(crate) const PLUGIN_RIGHT: (StaticPlugin, StaticPlugin) = plugin(
    "rounded-r",
    MultipleProps(&["border-top-right-radius", "border-bottom-right-radius"]),
);

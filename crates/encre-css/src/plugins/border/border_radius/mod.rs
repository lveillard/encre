#![doc = include_str!("README.md")]
#![doc(alias("border", "rounded"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const fn plugin(prefix: &'static str, prop: PropertyName) -> (Plugin, Plugin) {
    (
        Plugin::new(PluginKind::ListValues {
            prop,
            values: phf_map! {
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
        .list_prefix(prefix),
        Plugin::new(PluginKind::Arbitrary { prefix, prop })
            .hints(&[])
            .matcher(SpaceSeparated(&Or(&Length, &Percentage))),
    )
}

pub(crate) const PLUGIN: (Plugin, Plugin) = plugin("rounded", SingleProp("border-radius"));
pub(crate) const PLUGIN_START: (Plugin, Plugin) = plugin(
    "rounded-s",
    MultipleProps(&["border-start-start-radius", "border-end-start-radius"]),
);
pub(crate) const PLUGIN_END: (Plugin, Plugin) = plugin(
    "rounded-e",
    MultipleProps(&["border-start-end-radius", "border-end-end-radius"]),
);
pub(crate) const PLUGIN_START_START: (Plugin, Plugin) =
    plugin("rounded-ss", SingleProp("border-start-start-radius"));
pub(crate) const PLUGIN_START_END: (Plugin, Plugin) =
    plugin("rounded-se", SingleProp("border-start-end-radius"));
pub(crate) const PLUGIN_END_END: (Plugin, Plugin) =
    plugin("rounded-ee", SingleProp("border-end-end-radius"));
pub(crate) const PLUGIN_END_START: (Plugin, Plugin) =
    plugin("rounded-es", SingleProp("border-end-start-radius"));
pub(crate) const PLUGIN_TOP_RIGHT: (Plugin, Plugin) =
    plugin("rounded-tr", SingleProp("border-top-right-radius"));
pub(crate) const PLUGIN_TOP_LEFT: (Plugin, Plugin) =
    plugin("rounded-tl", SingleProp("border-top-left-radius"));
pub(crate) const PLUGIN_BOTTOM_RIGHT: (Plugin, Plugin) =
    plugin("rounded-br", SingleProp("border-bottom-right-radius"));
pub(crate) const PLUGIN_BOTTOM_LEFT: (Plugin, Plugin) =
    plugin("rounded-bl", SingleProp("border-bottom-left-radius"));
pub(crate) const PLUGIN_TOP: (Plugin, Plugin) = plugin(
    "rounded-t",
    MultipleProps(&["border-top-left-radius", "border-top-right-radius"]),
);
pub(crate) const PLUGIN_BOTTOM: (Plugin, Plugin) = plugin(
    "rounded-b",
    MultipleProps(&["border-bottom-left-radius", "border-bottom-right-radius"]),
);
pub(crate) const PLUGIN_LEFT: (Plugin, Plugin) = plugin(
    "rounded-l",
    MultipleProps(&["border-top-left-radius", "border-bottom-left-radius"]),
);
pub(crate) const PLUGIN_RIGHT: (Plugin, Plugin) = plugin(
    "rounded-r",
    MultipleProps(&["border-top-right-radius", "border-bottom-right-radius"]),
);

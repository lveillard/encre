#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{plugins::sizing::CSS_SIZE_VALUES_HORIZONTAL, prelude::build_plugin::*};

pub(crate) const PLUGIN_SPACING: StaticPlugin = Plugin::new(PluginKind::Spacing {
    namespace: "max-w",
    prop: SingleProp("max-width"),
});

pub(crate) const PLUGIN_LIST_1: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("max-width"),
    values: phf_map! {
        "max-w-none" => "none",
        "max-w-xs" => "20rem",
        "max-w-sm" => "24rem",
        "max-w-md" => "28rem",
        "max-w-lg" => "32rem",
        "max-w-xl" => "36rem",
        "max-w-2xl" => "42rem",
        "max-w-3xl" => "48rem",
        "max-w-4xl" => "56rem",
        "max-w-5xl" => "64rem",
        "max-w-6xl" => "72rem",
        "max-w-7xl" => "80rem",
        "max-w-full" => "100%",
        "max-w-min" => "min-content",
        "max-w-max" => "max-content",
        "max-w-fit" => "fit-content",
        "max-w-prose" => "65ch",
        "max-w-screen" => "100vw",
        "max-w-screen-sm" => "640px",
        "max-w-screen-md" => "768px",
        "max-w-screen-lg" => "1024px",
        "max-w-screen-xl" => "1280px",
        "max-w-screen-2xl" => "1536px",
    },
});

pub(crate) const PLUGIN_LIST_2: StaticPlugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("max-width"),
    values: CSS_SIZE_VALUES_HORIZONTAL,
}).list_namespace("max-w");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "max-w",
    prop: SingleProp("max-width"),
});

#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Spacing {
    prefix: "max-w",
    prop: SingleProp("max-width"),
    has_auto: false,
    has_full: false,
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::ListValues {
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

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "max-w",
    prop: SingleProp("max-width"),
});

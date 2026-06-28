#![doc = include_str!("README.md")]
#![doc(alias("border", "rounded"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

const VALUES: phf::Map<&'static str, &'static str> = phf_map! {
    "none" => "0",
    "xs" => "0.125rem",
    "sm" => "0.25rem",
    "md" => "0.375rem",
    "lg" => "0.5rem",
    "xl" => "0.75rem",
    "2xl" => "1rem",
    "3xl" => "1.5rem",
    "full" => "9999px",
};

const fn builtin_plugin(prop: PropertyName) -> Plugin {
    Plugin::ListValues {
        prop,
        values: VALUES,
    }
}

const fn arbitrary_plugin(prop: PropertyName) -> Plugin {
    Plugin::OnlyArbitrary {
        prop,
        hints: &[],
        matcher: SpaceSeparated(&Or(&Length, &Percentage)),
    }
}

pub(crate) const PLUGIN_1: Plugin = builtin_plugin(SingleProp("border-radius"));
pub(crate) const PLUGIN_2: Plugin = arbitrary_plugin(SingleProp("border-radius"));

pub(crate) const PLUGIN_START_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-start-start-radius",
    "border-end-start-radius",
]));
pub(crate) const PLUGIN_START_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-start-start-radius",
    "border-end-start-radius",
]));

pub(crate) const PLUGIN_END_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-start-end-radius",
    "border-end-end-radius",
]));
pub(crate) const PLUGIN_END_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-start-end-radius",
    "border-end-end-radius",
]));

pub(crate) const PLUGIN_START_START_1: Plugin =
    builtin_plugin(SingleProp("border-start-start-radius"));
pub(crate) const PLUGIN_START_START_2: Plugin =
    arbitrary_plugin(SingleProp("border-start-start-radius"));

pub(crate) const PLUGIN_START_END_1: Plugin = builtin_plugin(SingleProp("border-start-end-radius"));
pub(crate) const PLUGIN_START_END_2: Plugin =
    arbitrary_plugin(SingleProp("border-start-end-radius"));

pub(crate) const PLUGIN_END_END_1: Plugin = builtin_plugin(SingleProp("border-end-end-radius"));
pub(crate) const PLUGIN_END_END_2: Plugin = arbitrary_plugin(SingleProp("border-end-end-radius"));

pub(crate) const PLUGIN_END_START_1: Plugin = builtin_plugin(SingleProp("border-end-start-radius"));
pub(crate) const PLUGIN_END_START_2: Plugin =
    arbitrary_plugin(SingleProp("border-end-start-radius"));

pub(crate) const PLUGIN_TOP_RIGHT_1: Plugin = builtin_plugin(SingleProp("border-top-right-radius"));
pub(crate) const PLUGIN_TOP_RIGHT_2: Plugin =
    arbitrary_plugin(SingleProp("border-top-right-radius"));

pub(crate) const PLUGIN_TOP_LEFT_1: Plugin = builtin_plugin(SingleProp("border-top-left-radius"));
pub(crate) const PLUGIN_TOP_LEFT_2: Plugin = arbitrary_plugin(SingleProp("border-top-left-radius"));

pub(crate) const PLUGIN_BOTTOM_RIGHT_1: Plugin =
    builtin_plugin(SingleProp("border-bottom-right-radius"));
pub(crate) const PLUGIN_BOTTOM_RIGHT_2: Plugin =
    arbitrary_plugin(SingleProp("border-bottom-right-radius"));

pub(crate) const PLUGIN_BOTTOM_LEFT_1: Plugin =
    builtin_plugin(SingleProp("border-bottom-left-radius"));
pub(crate) const PLUGIN_BOTTOM_LEFT_2: Plugin =
    arbitrary_plugin(SingleProp("border-bottom-left-radius"));

pub(crate) const PLUGIN_TOP_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-top-left-radius",
    "border-top-right-radius",
]));
pub(crate) const PLUGIN_TOP_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-top-left-radius",
    "border-top-right-radius",
]));

pub(crate) const PLUGIN_BOTTOM_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-bottom-left-radius",
    "border-bottom-right-radius",
]));
pub(crate) const PLUGIN_BOTTOM_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-bottom-left-radius",
    "border-bottom-right-radius",
]));

pub(crate) const PLUGIN_LEFT_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-top-left-radius",
    "border-bottom-left-radius",
]));
pub(crate) const PLUGIN_LEFT_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-top-left-radius",
    "border-bottom-left-radius",
]));

pub(crate) const PLUGIN_RIGHT_1: Plugin = builtin_plugin(MultipleProps(&[
    "border-top-right-radius",
    "border-bottom-right-radius",
]));
pub(crate) const PLUGIN_RIGHT_2: Plugin = arbitrary_plugin(MultipleProps(&[
    "border-top-right-radius",
    "border-bottom-right-radius",
]));

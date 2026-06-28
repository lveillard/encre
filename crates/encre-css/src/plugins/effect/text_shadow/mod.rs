#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("text-shadow"),
    values: phf_map! {
        "2xs" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.15))",
        "xs" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.2))",
        "sm" => "0px 1px 0px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.075)), 0px 2px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.075))",
        "md" => "0px 1px 1px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 2px 4px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "lg" => "0px 1px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 3px 2px var(--en-text-shadow-color, rgb(0 0 0 / 0.1)), 0px 4px 8px var(--en-text-shadow-color, rgb(0 0 0 / 0.1))",
        "none" => "none",
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::ArbitraryShadow {
  prop: SingleProp("text-shadow"),
  color_replacement: "var(--en-text-shadow-color, {})",
  extra_line: "",
};

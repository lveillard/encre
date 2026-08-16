#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: MultipleProps(&["-webkit-text-decoration-line", "text-decoration-line"]),
    values: map! {
        "underline" => "underline",
        "overline" => "overline",
        "line-through" => "line-through",
        "no-underline" => "none",
    },
    ..ListValues::default()
});

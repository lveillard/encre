#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("transform"),
    values: map! {
        "transform" | "transform-cpu" => "translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z))",
        "transform-gpu" => "translateZ(0) translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z))",
        "transform-none" => "none",
    },
    ..ListValues::default()
});

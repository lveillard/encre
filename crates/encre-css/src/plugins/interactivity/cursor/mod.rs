#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("cursor"),
    values: map! {
        "cursor-auto" => "auto",
        "cursor-default" => "default",
        "cursor-pointer" => "pointer",
        "cursor-wait" => "wait",
        "cursor-text" => "text",
        "cursor-move" => "move",
        "cursor-help" => "help",
        "cursor-not-allowed" => "not-allowed",
        "cursor-none" => "none",
        "cursor-context-menu" => "context-menu",
        "cursor-progress" => "progress",
        "cursor-cell" => "cell",
        "cursor-crosshair" => "crosshair",
        "cursor-vertical-text" => "vertical-text",
        "cursor-alias" => "alias",
        "cursor-copy" => "copy",
        "cursor-no-drop" => "no-drop",
        "cursor-grab" => "grab",
        "cursor-grabbing" => "grabbing",
        "cursor-all-scroll" => "all-scroll",
        "cursor-col-resize" => "col-resize",
        "cursor-row-resize" => "row-resize",
        "cursor-n-resize" => "n-resize",
        "cursor-e-resize" => "e-resize",
        "cursor-s-resize" => "s-resize",
        "cursor-w-resize" => "w-resize",
        "cursor-ne-resize" => "ne-resize",
        "cursor-nw-resize" => "nw-resize",
        "cursor-se-resize" => "se-resize",
        "cursor-sw-resize" => "sw-resize",
        "cursor-ew-resize" => "ew-resize",
        "cursor-ns-resize" => "ns-resize",
        "cursor-nesw-resize" => "nesw-resize",
        "cursor-nwse-resize" => "nwse-resize",
        "cursor-zoom-in" => "zoom-in",
        "cursor-zoom-out" => "zoom-out",
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "cursor",
    prop: SingleProp("cursor"),
    ..Arbitrary::default()
});

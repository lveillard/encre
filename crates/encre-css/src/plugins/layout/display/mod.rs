#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("display"),
    values: map! {
        "hidden" => "none",
        "contents" => "contents",
        "list-item" => "list-item",
        "block" => "block",
        "inline-block" => "inline-block",
        "flex" => "flex",
        "inline-flex" => "inline-flex",
        "inline" => "inline",
        "table" => "table",
        "inline-table" => "inline-table",
        "table-cell" => "table-cell",
        "table-caption" => "table-caption",
        "table-column" => "table-column",
        "table-column-group" => "table-column-group",
        "table-footer-group" => "table-footer-group",
        "table-header-group" => "table-header-group",
        "table-row-group" => "table-row-group",
        "table-row" => "table-row",
        "flow-root" => "flow-root",
        "grid" => "grid",
        "inline-grid" => "inline-grid",
    },
    ..ListValues::default()
});

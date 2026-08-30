//! Sizing utilities
pub mod height;
pub mod max_height;
pub mod max_width;
pub mod min_height;
pub mod min_width;
pub mod width;

use crate::prelude::build_plugin::*;

pub(super) const CSS_SIZE_VALUES_HORIZONTAL: phf::Map<&'static str, &'static str> = map! {
    "none" => "none",
    "screen" => "100vw",
    "min" => "min-content",
    "max" => "max-content",
    "fit" => "fit-content",
    "svw" => "100svw",
    "lvw" => "100lvw",
    "dvw" => "100dvw",
    "svh" => "100svh",
    "lvh" => "100lvh",
    "dvh" => "100dvh",
};

pub(super) const CSS_SIZE_VALUES_VERTICAL: phf::Map<&'static str, &'static str> = map! {
    "none" => "none",
    "screen" => "100vh",
    "min" => "min-content",
    "max" => "max-content",
    "fit" => "fit-content",
    "svw" => "100svw",
    "lvw" => "100lvw",
    "dvw" => "100dvw",
    "svh" => "100svh",
    "lvh" => "100lvh",
    "dvh" => "100dvh",
};

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn height() {
        assert_eq!(
            generate(["h-px"], &base_config()),
            ".h-px {
  height: 1px;
}"
        );
        assert_eq!(
            generate(["h-2.5"], &base_config()),
            r".h-2\.5 {
  height: 0.625rem;
}"
        );
        assert_eq!(
            generate(["h-60"], &base_config()),
            ".h-60 {
  height: 15rem;
}"
        );
        assert_eq!(
            generate(["-h-60"], &base_config()),
            ".-h-60 {
  height: -15rem;
}"
        );
        assert_eq!(
            generate(["h-auto"], &base_config()),
            ".h-auto {
  height: auto;
}"
        );
        assert_eq!(
            generate(["h-1/3"], &base_config()),
            r".h-1\/3 {
  height: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-h-2/3"], &base_config()),
            r".-h-2\/3 {
  height: -66.666667%;
}"
        );
        assert_eq!(
            generate(["h-full"], &base_config()),
            ".h-full {
  height: 100%;
}"
        );
        assert_eq!(
            generate(["h-screen"], &base_config()),
            ".h-screen {
  height: 100vh;
}"
        );
        assert_eq!(
            generate(["h-lvh"], &base_config()),
            ".h-lvh {
  height: 100lvh;
}"
        );
        assert_eq!(
            generate(["h-fit"], &base_config()),
            ".h-fit {
  height: fit-content;
}"
        );
        assert_eq!(
            generate(["h-[32.555rem]"], &base_config()),
            r".h-\[32\.555rem\] {
  height: 32.555rem;
}"
        );
        assert_eq!(
            generate(["h-[10%]"], &base_config()),
            r".h-\[10\%\] {
  height: 10%;
}"
        );
    }

    #[test]
    fn max_height() {
        assert_eq!(
            generate(["max-h-px"], &base_config()),
            ".max-h-px {
  max-height: 1px;
}"
        );
        assert_eq!(
            generate(["max-h-2.5"], &base_config()),
            r".max-h-2\.5 {
  max-height: 0.625rem;
}"
        );
        assert_eq!(
            generate(["max-h-60"], &base_config()),
            ".max-h-60 {
  max-height: 15rem;
}"
        );
        assert_eq!(
            generate(["-max-h-60"], &base_config()),
            ".-max-h-60 {
  max-height: -15rem;
}"
        );
        assert_eq!(
            generate(["max-h-auto"], &base_config()),
            ".max-h-auto {
  max-height: auto;
}"
        );
        assert_eq!(
            generate(["max-h-none"], &base_config()),
            ".max-h-none {
  max-height: none;
}"
        );
        assert_eq!(
            generate(["max-h-1/3"], &base_config()),
            r".max-h-1\/3 {
  max-height: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-max-h-2/3"], &base_config()),
            r".-max-h-2\/3 {
  max-height: -66.666667%;
}"
        );
        assert_eq!(
            generate(["max-h-full"], &base_config()),
            ".max-h-full {
  max-height: 100%;
}"
        );
        assert_eq!(
            generate(["max-h-screen"], &base_config()),
            ".max-h-screen {
  max-height: 100vh;
}"
        );
        assert_eq!(
            generate(["max-h-lvh"], &base_config()),
            ".max-h-lvh {
  max-height: 100lvh;
}"
        );
        assert_eq!(
            generate(["max-h-fit"], &base_config()),
            ".max-h-fit {
  max-height: fit-content;
}"
        );
        assert_eq!(
            generate(["max-h-[32.555rem]"], &base_config()),
            r".max-h-\[32\.555rem\] {
  max-height: 32.555rem;
}"
        );
        assert_eq!(
            generate(["max-h-[10%]"], &base_config()),
            r".max-h-\[10\%\] {
  max-height: 10%;
}"
        );
    }

    #[test]
    fn max_width() {
        assert_eq!(
            generate(["max-w-6xl"], &base_config()),
            ".max-w-6xl {
  max-width: 72rem;
}"
        );
        assert_eq!(
            generate(["max-w-prose"], &base_config()),
            ".max-w-prose {
  max-width: 65ch;
}"
        );
        assert_eq!(
            generate(["max-w-screen-xl"], &base_config()),
            ".max-w-screen-xl {
  max-width: 1280px;
}"
        );
        assert_eq!(
            generate(["max-w-px"], &base_config()),
            ".max-w-px {
  max-width: 1px;
}"
        );
        assert_eq!(
            generate(["max-w-2.5"], &base_config()),
            r".max-w-2\.5 {
  max-width: 0.625rem;
}"
        );
        assert_eq!(
            generate(["max-w-60"], &base_config()),
            ".max-w-60 {
  max-width: 15rem;
}"
        );
        assert_eq!(
            generate(["-max-w-60"], &base_config()),
            ".-max-w-60 {
  max-width: -15rem;
}"
        );
        assert_eq!(
            generate(["max-w-none"], &base_config()),
            ".max-w-none {
  max-width: none;
}"
        );
        assert_eq!(
            generate(["max-w-1/3"], &base_config()),
            r".max-w-1\/3 {
  max-width: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-max-w-2/3"], &base_config()),
            r".-max-w-2\/3 {
  max-width: -66.666667%;
}"
        );
        assert_eq!(
            generate(["max-w-full"], &base_config()),
            ".max-w-full {
  max-width: 100%;
}"
        );
        assert_eq!(
            generate(["max-w-screen"], &base_config()),
            ".max-w-screen {
  max-width: 100vw;
}"
        );
        assert_eq!(
            generate(["max-w-fit"], &base_config()),
            ".max-w-fit {
  max-width: fit-content;
}"
        );
        assert_eq!(
            generate(["max-w-[32.555rem]"], &base_config()),
            r".max-w-\[32\.555rem\] {
  max-width: 32.555rem;
}"
        );
        assert_eq!(
            generate(["max-w-[10%]"], &base_config()),
            r".max-w-\[10\%\] {
  max-width: 10%;
}"
        );
    }

    #[test]
    fn min_height() {
        assert_eq!(
            generate(["min-h-px"], &base_config()),
            ".min-h-px {
  min-height: 1px;
}"
        );
        assert_eq!(
            generate(["min-h-2.5"], &base_config()),
            r".min-h-2\.5 {
  min-height: 0.625rem;
}"
        );
        assert_eq!(
            generate(["min-h-60"], &base_config()),
            ".min-h-60 {
  min-height: 15rem;
}"
        );
        assert_eq!(
            generate(["-min-h-60"], &base_config()),
            ".-min-h-60 {
  min-height: -15rem;
}"
        );
        assert_eq!(
            generate(["min-h-auto"], &base_config()),
            ".min-h-auto {
  min-height: auto;
}"
        );
        assert_eq!(
            generate(["min-h-1/3"], &base_config()),
            r".min-h-1\/3 {
  min-height: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-min-h-2/3"], &base_config()),
            r".-min-h-2\/3 {
  min-height: -66.666667%;
}"
        );
        assert_eq!(
            generate(["min-h-full"], &base_config()),
            ".min-h-full {
  min-height: 100%;
}"
        );
        assert_eq!(
            generate(["min-h-screen"], &base_config()),
            ".min-h-screen {
  min-height: 100vh;
}"
        );
        assert_eq!(
            generate(["min-h-lvh"], &base_config()),
            ".min-h-lvh {
  min-height: 100lvh;
}"
        );
        assert_eq!(
            generate(["min-h-fit"], &base_config()),
            ".min-h-fit {
  min-height: fit-content;
}"
        );
        assert_eq!(
            generate(["min-h-[32.555rem]"], &base_config()),
            r".min-h-\[32\.555rem\] {
  min-height: 32.555rem;
}"
        );
        assert_eq!(
            generate(["min-h-[10%]"], &base_config()),
            r".min-h-\[10\%\] {
  min-height: 10%;
}"
        );
    }

    #[test]
    fn min_width() {
        assert_eq!(
            generate(["min-w-px"], &base_config()),
            ".min-w-px {
  min-width: 1px;
}"
        );
        assert_eq!(
            generate(["min-w-2.5"], &base_config()),
            r".min-w-2\.5 {
  min-width: 0.625rem;
}"
        );
        assert_eq!(
            generate(["min-w-60"], &base_config()),
            ".min-w-60 {
  min-width: 15rem;
}"
        );
        assert_eq!(
            generate(["-min-w-60"], &base_config()),
            ".-min-w-60 {
  min-width: -15rem;
}"
        );
        assert_eq!(
            generate(["min-w-auto"], &base_config()),
            ".min-w-auto {
  min-width: auto;
}"
        );
        assert_eq!(
            generate(["min-w-1/3"], &base_config()),
            r".min-w-1\/3 {
  min-width: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-min-w-2/3"], &base_config()),
            r".-min-w-2\/3 {
  min-width: -66.666667%;
}"
        );
        assert_eq!(
            generate(["min-w-full"], &base_config()),
            ".min-w-full {
  min-width: 100%;
}"
        );
        assert_eq!(
            generate(["min-w-screen"], &base_config()),
            ".min-w-screen {
  min-width: 100vw;
}"
        );
        assert_eq!(
            generate(["min-w-fit"], &base_config()),
            ".min-w-fit {
  min-width: fit-content;
}"
        );
        assert_eq!(
            generate(["min-w-[32.555rem]"], &base_config()),
            r".min-w-\[32\.555rem\] {
  min-width: 32.555rem;
}"
        );
        assert_eq!(
            generate(["min-w-[10%]"], &base_config()),
            r".min-w-\[10\%\] {
  min-width: 10%;
}"
        );
    }

    #[test]
    fn width() {
        assert_eq!(
            generate(["w-px"], &base_config()),
            ".w-px {
  width: 1px;
}"
        );
        assert_eq!(
            generate(["w-2.5"], &base_config()),
            r".w-2\.5 {
  width: 0.625rem;
}"
        );
        assert_eq!(
            generate(["w-60"], &base_config()),
            ".w-60 {
  width: 15rem;
}"
        );
        assert_eq!(
            generate(["-w-60"], &base_config()),
            ".-w-60 {
  width: -15rem;
}"
        );
        assert_eq!(
            generate(["w-auto"], &base_config()),
            ".w-auto {
  width: auto;
}"
        );
        assert_eq!(
            generate(["w-1/3"], &base_config()),
            r".w-1\/3 {
  width: 33.333333%;
}"
        );
        assert_eq!(
            generate(["-w-2/3"], &base_config()),
            r".-w-2\/3 {
  width: -66.666667%;
}"
        );
        assert_eq!(
            generate(["w-full"], &base_config()),
            ".w-full {
  width: 100%;
}"
        );
        assert_eq!(
            generate(["w-screen"], &base_config()),
            ".w-screen {
  width: 100vw;
}"
        );
        assert_eq!(
            generate(["w-lvw"], &base_config()),
            ".w-lvw {
  width: 100lvw;
}"
        );
        assert_eq!(
            generate(["w-fit"], &base_config()),
            ".w-fit {
  width: fit-content;
}"
        );
        assert_eq!(
            generate(["w-[32.555rem]"], &base_config()),
            r".w-\[32\.555rem\] {
  width: 32.555rem;
}"
        );
        assert_eq!(
            generate(["w-[10%]"], &base_config()),
            r".w-\[10\%\] {
  width: 10%;
}"
        );
    }
}

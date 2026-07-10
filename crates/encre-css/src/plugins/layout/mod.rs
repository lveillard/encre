//! Layout utilities
pub mod aspect_ratio;
pub mod box_decoration_break;
pub mod box_sizing;
pub mod break_after;
pub mod break_before;
pub mod break_inside;
pub mod clear;
pub mod columns;
pub mod container;
pub mod at_container;
pub mod display;
pub mod floats;
pub mod isolation;
pub mod object_fit;
pub mod object_position;
pub mod overflow;
pub mod overscroll_behavior;
pub mod placement;
pub mod position;
pub mod visibility;
pub mod z_index;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn aspect_ratio() {
        assert_eq!(
            generate(["aspect-auto"], &base_config()),
            ".aspect-auto {
  aspect-ratio: auto;
}"
        );
        assert_eq!(
            generate(["aspect-video"], &base_config()),
            ".aspect-video {
  aspect-ratio: 16 / 9;
}"
        );
    }

    #[test]
    fn box_decoration_break() {
        assert_eq!(
            generate(["box-decoration-clone"], &base_config()),
            ".box-decoration-clone {
  box-decoration-break: clone;
}"
        );
    }

    #[test]
    fn box_sizing() {
        assert_eq!(
            generate(["box-content"], &base_config()),
            ".box-content {
  box-sizing: content-box;
}"
        );
    }

    #[test]
    fn break_after() {
        assert_eq!(
            generate(["break-after-all"], &base_config()),
            ".break-after-all {
  break-after: all;
}"
        );
        assert_eq!(
            generate(["break-after-avoid-page"], &base_config()),
            ".break-after-avoid-page {
  break-after: avoid-page;
}"
        );
        assert_eq!(
            generate(["break-after-column"], &base_config()),
            ".break-after-column {
  break-after: column;
}"
        );
    }

    #[test]
    fn break_before() {
        assert_eq!(
            generate(["break-before-all"], &base_config()),
            ".break-before-all {
  break-before: all;
}"
        );
        assert_eq!(
            generate(["break-before-avoid-page"], &base_config()),
            ".break-before-avoid-page {
  break-before: avoid-page;
}"
        );
        assert_eq!(
            generate(["break-before-column"], &base_config()),
            ".break-before-column {
  break-before: column;
}"
        );
    }

    #[test]
    fn break_inside() {
        assert_eq!(
            generate(["break-inside-auto"], &base_config()),
            ".break-inside-auto {
  break-inside: auto;
}"
        );
        assert_eq!(
            generate(["break-inside-avoid-page"], &base_config()),
            ".break-inside-avoid-page {
  break-inside: avoid-page;
}"
        );
    }

    #[test]
    fn clear() {
        assert_eq!(
            generate(["clear-both"], &base_config()),
            ".clear-both {
  clear: both;
}"
        );

        assert_eq!(
            generate(["clear-start"], &base_config()),
            ".clear-start {
  clear: inline-start;
}"
        );

        assert_eq!(
            generate(["clear-end"], &base_config()),
            ".clear-end {
  clear: inline-end;
}"
        );
    }

    #[test]
    fn columns() {
        assert_eq!(
            generate(["columns-md"], &base_config()),
            ".columns-md {
  columns: 28rem;
}"
        );
        assert_eq!(
            generate(["columns-4"], &base_config()),
            ".columns-4 {
  columns: 4;
}"
        );
    }

    #[test]
    fn container() {
        assert_eq!(
            generate(["container"], &base_config()),
            ".container {
  width: 100%;
}

@media (width >= 40rem) {
  .container {
    max-width: 40rem;
  }
}

@media (width >= 48rem) {
  .container {
    max-width: 48rem;
  }
}

@media (width >= 64rem) {
  .container {
    max-width: 64rem;
  }
}

@media (width >= 80rem) {
  .container {
    max-width: 80rem;
  }
}

@media (width >= 96rem) {
  .container {
    max-width: 96rem;
  }
}"
        );
    }

    #[test]
    fn display() {
        assert_eq!(
            generate(["inline-flex"], &base_config()),
            ".inline-flex {
  display: inline-flex;
}"
        );
        assert_eq!(
            generate(["hidden"], &base_config()),
            ".hidden {
  display: none;
}"
        );
    }

    #[test]
    fn floats() {
        assert_eq!(
            generate(["float-right"], &base_config()),
            ".float-right {
  float: right;
}"
        );

        assert_eq!(
            generate(["float-start"], &base_config()),
            ".float-start {
  float: inline-start;
}"
        );

        assert_eq!(
            generate(["float-end"], &base_config()),
            ".float-end {
  float: inline-end;
}"
        );
    }

    #[test]
    fn isolation() {
        assert_eq!(
            generate(["isolate"], &base_config()),
            ".isolate {
  isolation: isolate;
}"
        );
        assert_eq!(
            generate(["isolation-auto"], &base_config()),
            ".isolation-auto {
  isolation: auto;
}"
        );
    }

    #[test]
    fn object_fit() {
        assert_eq!(
            generate(["object-contain"], &base_config()),
            ".object-contain {
  object-fit: contain;
}"
        );
        assert_eq!(
            generate(["object-scale-down"], &base_config()),
            ".object-scale-down {
  object-fit: scale-down;
}"
        );
    }

    #[test]
    fn object_position() {
        assert_eq!(
            generate(["object-bottom"], &base_config()),
            ".object-bottom {
  object-position: bottom;
}"
        );
        assert_eq!(
            generate(["object-top-left"], &base_config()),
            ".object-top-left {
  object-position: top left;
}"
        );
        assert_eq!(
            generate(["object-[center_bottom]"], &base_config()),
            r".object-\[center_bottom\] {
  object-position: center bottom;
}"
        );
    }

    #[test]
    fn overflow() {
        assert_eq!(
            generate(["overflow-hidden"], &base_config()),
            ".overflow-hidden {
  overflow: hidden;
}"
        );
        assert_eq!(
            generate(["overflow-y-scroll"], &base_config()),
            ".overflow-y-scroll {
  overflow-y: scroll;
}"
        );
    }

    #[test]
    fn overscroll_behavior() {
        assert_eq!(
            generate(["overscroll-auto"], &base_config()),
            ".overscroll-auto {
  overscroll-behavior: auto;
}"
        );
        assert_eq!(
            generate(["overscroll-y-none"], &base_config()),
            ".overscroll-y-none {
  overscroll-behavior-y: none;
}"
        );
    }

    #[test]
    fn placement() {
        assert_eq!(
            generate(["top-12"], &base_config()),
            ".top-12 {
  top: 3rem;
}"
        );
        assert_eq!(
            generate(["top-[1px]"], &base_config()),
            r".top-\[1px\] {
  top: 1px;
}"
        );
        assert_eq!(
            generate(["left-auto"], &base_config()),
            ".left-auto {
  left: auto;
}"
        );
        assert_eq!(
            generate(["right-full"], &base_config()),
            ".right-full {
  right: 100%;
}"
        );
        assert_eq!(
            generate(["inset-2"], &base_config()),
            ".inset-2 {
  inset: 0.5rem;
}"
        );
        assert_eq!(
            generate(["-top-full"], &base_config()),
            ".-top-full {
  top: -100%;
}"
        );
        assert_eq!(
            generate(["-left-20"], &base_config()),
            ".-left-20 {
  left: -5rem;
}"
        );
        assert_eq!(
            generate(["right-[20%]"], &base_config()),
            r".right-\[20\%\] {
  right: 20%;
}"
        );
        assert_eq!(
            generate(["inset-[20px]"], &base_config()),
            r".inset-\[20px\] {
  inset: 20px;
}"
        );
        assert_eq!(
            generate(["inset-y-[10em]"], &base_config()),
            r".inset-y-\[10em\] {
  inset-block: 10em;
}"
        );
    }

    #[test]
    fn position() {
        assert_eq!(
            generate(["relative"], &base_config()),
            ".relative {
  position: relative;
}"
        );
    }

    #[test]
    fn visibility() {
        assert_eq!(
            generate(["visible"], &base_config()),
            ".visible {
  visibility: visible;
}"
        );
        assert_eq!(
            generate(["invisible"], &base_config()),
            ".invisible {
  visibility: hidden;
}"
        );
    }

    #[test]
    fn z_index() {
        assert_eq!(
            generate(["z-22"], &base_config()),
            ".z-22 {
  z-index: 22;
}"
        );

        assert_eq!(
            generate(["z-auto"], &base_config()),
            ".z-auto {
  z-index: auto;
}"
        );
    }

    #[test]
    fn at_container() {
        assert_eq!(
            generate(
                ["@container @container-[size_scroll-state] @md:bg-red-300"],
                &base_config()
            ),
            r".\@container {
  container-type: inline-size;
}

.\@container-\[size_scroll-state\] {
  container-type: size scroll-state;
}

@container (width >= 28rem) {
  .\@md\:bg-red-300 {
    background-color: oklch(80.8% .114 19.571);
  }
}"
        );
    }
}

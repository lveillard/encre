//! SVG utilities
pub mod fill;
pub mod stroke;
pub mod stroke_width;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn fill() {
        assert_eq!(
            generate(["fill-red-400"], &base_config()),
            ".fill-red-400 {
  fill: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["fill-[rgb(12,12,12)]"], &base_config()),
            r".fill-\[rgb\(12\,12\,12\)\] {
  fill: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["fill-[purple]"], &base_config()),
            r".fill-\[purple\] {
  fill: purple;
}"
        );
    }

    #[test]
    fn stroke() {
        assert_eq!(
            generate(["stroke-red-400"], &base_config()),
            ".stroke-red-400 {
  stroke: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["stroke-[rgb(12,12,12)]"], &base_config()),
            r".stroke-\[rgb\(12\,12\,12\)\] {
  stroke: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["stroke-[purple]"], &base_config()),
            r".stroke-\[purple\] {
  stroke: purple;
}"
        );
    }

    #[test]
    fn stroke_width() {
        assert_eq!(
            generate(["stroke-2"], &base_config()),
            ".stroke-2 {
  stroke-width: 2px;
}"
        );
        assert_eq!(
            generate(["stroke-[0.25rem]"], &base_config()),
            r".stroke-\[0\.25rem\] {
  stroke-width: 0.25rem;
}"
        );
        assert_eq!(
            generate(["stroke-[5%]"], &base_config()),
            r".stroke-\[5\%\] {
  stroke-width: 5%;
}"
        );
        assert_eq!(
            generate(["stroke-[5%,20px]"], &base_config()),
            r".stroke-\[5\%\,20px\] {
  stroke-width: 5%,20px;
}"
        );
    }
}

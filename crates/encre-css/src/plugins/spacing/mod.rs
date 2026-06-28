//! Spacing utilities
pub mod margin;
pub mod padding;
// pub mod space_between;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn margin() {
        assert_eq!(
            generate(["m-2"], &base_config()),
            ".m-2 {
  margin: 0.5rem;
}"
        );
        assert_eq!(
            generate(["mt-auto"], &base_config()),
            ".mt-auto {
  margin-top: auto;
}"
        );
        assert_eq!(
            generate(["mx-[2px]"], &base_config()),
            r".mx-\[2px\] {
  margin-inline: 2px;
}"
        );
        assert_eq!(
            generate(["my-[20%]"], &base_config()),
            r".my-\[20\%\] {
  margin-block: 20%;
}"
        );
    }

    #[test]
    fn padding() {
        assert_eq!(
            generate(["p-2"], &base_config()),
            ".p-2 {
  padding: 0.5rem;
}"
        );
        assert_eq!(
            generate(["pt-auto"], &base_config()),
            ".pt-auto {
  padding-top: auto;
}"
        );
        assert_eq!(
            generate(["px-[2px]"], &base_config()),
            r".px-\[2px\] {
  padding-inline: 2px;
}"
        );
        assert_eq!(
            generate(["py-[20%]"], &base_config()),
            r".py-\[20\%\] {
  padding-block: 20%;
}"
        );
    }

    #[test]
    fn space_between() {
        assert_eq!(
            generate(["space-x-42"], &base_config()),
            ".space-x-42 > :not(:last-child) {
  --en-space-x-reverse: 0;
  margin-inline-start: calc(10.5rem * var(--en-space-x-reverse));
  margin-inline-end: calc(10.5rem * calc(1 - var(--en-space-x-reverse)));
}"
        );
        assert_eq!(
            generate(["space-x-[42px]"], &base_config()),
            r".space-x-\[42px\] > :not(:last-child) {
  --en-space-x-reverse: 0;
  margin-inline-start: calc(42px * var(--en-space-x-reverse));
  margin-inline-end: calc(42px * calc(1 - var(--en-space-x-reverse)));
}"
        );
        assert_eq!(
            generate(["space-x-reverse"], &base_config()),
            ".space-x-reverse > :not(:last-child) {
  --en-space-x-reverse: 1;
}"
        );
        assert_eq!(
            generate(["space-y-reverse"], &base_config()),
            ".space-y-reverse > :not(:last-child) {
  --en-space-y-reverse: 1;
}"
        );
        assert_eq!(
            generate(["space-y-[12%]"], &base_config()),
            r".space-y-\[12\%\] > :not(:last-child) {
  --en-space-y-reverse: 0;
  margin-block-start: calc(12% * var(--en-space-y-reverse));
  margin-block-end: calc(12% * calc(1 - var(--en-space-y-reverse)));
}"
        );
    }
}

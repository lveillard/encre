//! Table utilities
pub mod border_collapse;
pub mod border_spacing;
pub mod caption_side;
pub mod table_layout;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn border_collapse() {
        assert_eq!(
            generate(["border-separate"], &base_config()),
            ".border-separate {
  border-collapse: separate;
}"
        );
    }

    #[test]
    fn border_spacing() {
        assert_eq!(
            generate(["border-spacing-32"], &base_config()),
            ".border-spacing-32 {
  --en-border-spacing-x: 8rem;
  --en-border-spacing-y: 8rem;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            generate(["border-spacing-x-px"], &base_config()),
            ".border-spacing-x-px {
  --en-border-spacing-x: 1px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            generate(["border-spacing-y-0"], &base_config()),
            ".border-spacing-y-0 {
  --en-border-spacing-y: 0px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            generate(["border-spacing-[22px]"], &base_config()),
            r".border-spacing-\[22px\] {
  --en-border-spacing-x: 22px;
  --en-border-spacing-y: 22px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            generate(["border-spacing-x-[22px]"], &base_config()),
            r".border-spacing-x-\[22px\] {
  --en-border-spacing-x: 22px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
        assert_eq!(
            generate(["border-spacing-y-[12px]"], &base_config()),
            r".border-spacing-y-\[12px\] {
  --en-border-spacing-y: 12px;
  border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);
}"
        );
    }

    #[test]
    fn table_layout() {
        assert_eq!(
            generate(["table-fixed"], &base_config()),
            ".table-fixed {
  table-layout: fixed;
}"
        );
    }

    #[test]
    fn caption_side() {
        assert_eq!(
            generate(["caption-top"], &base_config()),
            ".caption-top {
  caption-side: top;
}"
        );

        assert_eq!(
            generate(["caption-bottom"], &base_config()),
            ".caption-bottom {
  caption-side: bottom;
}"
        );
    }
}

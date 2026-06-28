//! Border utilities
pub mod border_color;
pub mod border_radius;
pub mod border_style;
pub mod border_width;
// pub mod divide_color;
// pub mod divide_style;
// pub mod divide_width;
pub mod outline_color;
pub mod outline_offset;
pub mod outline_style;
pub mod outline_width;
pub mod ring_color;
pub mod ring_offset_color;
pub mod ring_offset_width;
pub mod ring_width;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn border_color() {
        assert_eq!(
            generate(["border-red-400"], &base_config()),
            ".border-red-400 {
  border-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["border-[rgb(12,12,12)]"], &base_config()),
            r".border-\[rgb\(12\,12\,12\)\] {
  border-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["border-x-[#ff0]"], &base_config()),
            r".border-x-\[\#ff0\] {
  border-inline-color: #ff0;
}"
        );
        assert_eq!(
            generate(["border-s-blue-400"], &base_config()),
            ".border-s-blue-400 {
  border-inline-start-color: oklch(70.7% .165 254.624);
}"
        );
        assert_eq!(
            generate(["border-e-blue-400/10"], &base_config()),
            r".border-e-blue-400\/10 {
  border-inline-end-color: color-mix(in oklab, oklch(70.7% .165 254.624) 10%, transparent);
}"
        );
    }

    #[test]
    fn border_style() {
        assert_eq!(
            generate(["border-dashed"], &base_config()),
            ".border-dashed {
  border-style: dashed;
}"
        );

        assert_eq!(
            generate(["border-[solid_none_solid_none]"], &base_config()),
            r".border-\[solid_none_solid_none\] {
  border-style: solid none solid none;
}"
        );
    }

    #[test]
    fn border_width() {
        assert_eq!(
            generate(["border"], &base_config()),
            ".border {
  border-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-x"], &base_config()),
            ".border-x {
  border-inline-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-s"], &base_config()),
            ".border-s {
  border-inline-start-width: 1px;
}"
        );
        assert_eq!(
            generate(["border-t"], &base_config()),
            ".border-t {
  border-top-width: 1px;
}"
        );

        assert_eq!(
            generate(["border-2"], &base_config()),
            ".border-2 {
  border-width: 2px;
}"
        );
        assert_eq!(
            generate(["border-x-24"], &base_config()),
            ".border-x-24 {
  border-inline-width: 24px;
}"
        );
        assert_eq!(
            generate(["border-e-2"], &base_config()),
            ".border-e-2 {
  border-inline-end-width: 2px;
}"
        );
        assert_eq!(
            generate(["border-t-42"], &base_config()),
            ".border-t-42 {
  border-top-width: 42px;
}"
        );
        assert_eq!(
            generate(["border-b-[3rem]"], &base_config()),
            r".border-b-\[3rem\] {
  border-bottom-width: 3rem;
}"
        );
        assert_eq!(
            generate(["border-y-[thick]"], &base_config()),
            r".border-y-\[thick\] {
  border-block-width: thick;
}"
        );
    }

    #[test]
    fn border_radius() {
        assert_eq!(
            generate(["rounded-sm"], &base_config()),
            ".rounded-sm {
  border-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-[2px]"], &base_config()),
            r".rounded-\[2px\] {
  border-radius: 2px;
}"
        );
        assert_eq!(
            generate(["rounded-bl-sm"], &base_config()),
            ".rounded-bl-sm {
  border-bottom-left-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-t-sm"], &base_config()),
            ".rounded-t-sm {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-s-sm"], &base_config()),
            ".rounded-s-sm {
  border-start-start-radius: 0.25rem;
  border-end-start-radius: 0.25rem;
}"
        );
        assert_eq!(
            generate(["rounded-xl"], &base_config()),
            ".rounded-xl {
  border-radius: 0.75rem;
}"
        );

        assert_eq!(
            generate(["rounded-xs"], &base_config()),
            ".rounded-xs {
  border-radius: 0.125rem;
}"
        );
        assert_eq!(
            generate(["rounded-bl-full"], &base_config()),
            ".rounded-bl-full {
  border-bottom-left-radius: 9999px;
}"
        );
        assert_eq!(
            generate(["rounded-ee-md"], &base_config()),
            ".rounded-ee-md {
  border-end-end-radius: 0.375rem;
}"
        );
        assert_eq!(
            generate(["rounded-t-3xl"], &base_config()),
            ".rounded-t-3xl {
  border-top-left-radius: 1.5rem;
  border-top-right-radius: 1.5rem;
}"
        );
        assert_eq!(
            generate(["rounded-b-[3em]"], &base_config()),
            r".rounded-b-\[3em\] {
  border-bottom-left-radius: 3em;
  border-bottom-right-radius: 3em;
}"
        );
        assert_eq!(
            generate(["rounded-tr-[20%]"], &base_config()),
            r".rounded-tr-\[20\%\] {
  border-top-right-radius: 20%;
}"
        );
    }

    #[test]
    fn divide_style() {
        assert_eq!(
            generate(["divide-double"], &base_config()),
            ".divide-double > :not([hidden]) ~ :not([hidden]) {
  border-style: double;
}"
        );
    }

    #[test]
    fn divide_color() {
        assert_eq!(
            generate(["divide-red-400"], &base_config()),
            ".divide-red-400 > :not([hidden]) ~ :not([hidden]) {
  border-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["divide-[rgb(12,12,12)]"], &base_config()),
            r".divide-\[rgb\(12\,12\,12\)\] > :not([hidden]) ~ :not([hidden]) {
  border-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn divide_width() {
        assert_eq!(
            generate(["divide-x"], &base_config()),
            ".divide-x > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 0;
  border-inline-start-width: calc(1px * var(--en-divide-x-reverse));
  border-inline-end-width: calc(1px * calc(1 - var(--en-divide-x-reverse)));
}"
        );
        assert_eq!(
            generate(["divide-y-2"], &base_config()),
            ".divide-y-2 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-block-start-width: calc(2px * var(--en-divide-y-reverse));
  border-block-end-width: calc(2px * calc(1 - var(--en-divide-y-reverse)));
}"
        );
        assert_eq!(
            generate(["divide-x-reverse"], &base_config()),
            ".divide-x-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 1;
}"
        );
        assert_eq!(
            generate(["divide-y-[0.1rem]"], &base_config()),
            r".divide-y-\[0\.1rem\] > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 0;
  border-block-start-width: calc(0.1rem * var(--en-divide-y-reverse));
  border-block-end-width: calc(0.1rem * calc(1 - var(--en-divide-y-reverse)));
}"
        );
        assert_eq!(
            generate(["divide-y-reverse"], &base_config()),
            ".divide-y-reverse > :not([hidden]) ~ :not([hidden]) {
  --en-divide-y-reverse: 1;
}"
        );
    }

    #[test]
    fn outline_style() {
        assert_eq!(
            generate(["outline-hidden"], &base_config()),
            ".outline-hidden {
  outline: 2px solid transparent;
  outline-offset: 2px;
}"
        );
        assert_eq!(
            generate(["outline-dashed"], &base_config()),
            ".outline-dashed {
  outline-style: dashed;
}"
        );
    }

    #[test]
    fn outline_width() {
        assert_eq!(
            generate(["outline-33"], &base_config()),
            ".outline-33 {
  outline-width: 33px;
}"
        );
        assert_eq!(
            generate(["outline-[2rem]"], &base_config()),
            r".outline-\[2rem\] {
  outline-width: 2rem;
}"
        );
        assert_eq!(
            generate(["outline-[thin]"], &base_config()),
            r".outline-\[thin\] {
  outline-width: thin;
}"
        );
    }

    #[test]
    fn outline_color() {
        assert_eq!(
            generate(["outline-red-400"], &base_config()),
            ".outline-red-400 {
  outline-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["outline-[rgb(12,12,12)]"], &base_config()),
            r".outline-\[rgb\(12\,12\,12\)\] {
  outline-color: rgb(12,12,12);
}"
        );
    }

    #[test]
    fn outline_offset() {
        assert_eq!(
            generate(["outline-offset-12"], &base_config()),
            ".outline-offset-12 {
  outline-offset: 12px;
}"
        );
        assert_eq!(
            generate(["outline-offset-[1rem]"], &base_config()),
            r".outline-offset-\[1rem\] {
  outline-offset: 1rem;
}"
        );
    }

    #[test]
    fn ring_width() {
        assert_eq!(
            generate(["ring"], &base_config()),
            ".ring {
  --en-ring-shadow: 0 0 0 calc(1px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            generate(["ring-11"], &base_config()),
            ".ring-11 {
  --en-ring-shadow: 0 0 0 calc(11px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            generate(["inset-ring"], &base_config()),
            ".inset-ring {
  --en-inset-ring-shadow: inset 0 0 0 calc(1px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);
}"
        );
        assert_eq!(
            generate(["inset-ring-12"], &base_config()),
            ".inset-ring-12 {
  --en-inset-ring-shadow: inset 0 0 0 calc(12px + var(--en-ring-offset-width)) var(--en-ring-color);
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow, 0 0 #0000);
}"
        );
    }

    #[test]
    fn ring_color() {
        assert_eq!(
            generate(["ring-red-400"], &base_config()),
            ".ring-red-400 {
  --en-ring-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["ring-[rgb(12,12,12)]"], &base_config()),
            r".ring-\[rgb\(12\,12\,12\)\] {
  --en-ring-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["inset-ring-red-400"], &base_config()),
            ".inset-ring-red-400 {
  --en-inset-ring-color: oklch(70.4% .191 22.216);
}"
        );
    }

    #[test]
    fn ring_offset_width() {
        assert_eq!(
            generate(["ring-offset-13"], &base_config()),
            ".ring-offset-13 {
  --en-ring-offset-width: 13px;
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
}"
        );
        assert_eq!(
            generate(["ring-offset-[13em]"], &base_config()),
            r".ring-offset-\[13em\] {
  --en-ring-offset-width: 13em;
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
}"
        );
    }

    #[test]
    fn ring_offset_color() {
        assert_eq!(
            generate(["ring-offset-red-400"], &base_config()),
            ".ring-offset-red-400 {
  --en-ring-offset-color: oklch(70.4% .191 22.216);
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
}"
        );
        assert_eq!(
            generate(["ring-offset-[rgb(12,12,12)]"], &base_config()),
            r".ring-offset-\[rgb\(12\,12\,12\)\] {
  --en-ring-offset-color: rgb(12,12,12);
  --en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);
}"
        );
    }
}

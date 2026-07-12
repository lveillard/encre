//! Background utilities
pub mod background_attachment;
pub mod background_clip;
pub mod background_color;
pub mod background_image;
pub mod background_origin;
pub mod background_position;
pub mod background_repeat;
pub mod background_size;
pub mod gradient_color_stops;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn background_attachment() {
        assert_eq!(
            generate(["bg-fixed"], &base_config()),
            ".bg-fixed {
  background-attachment: fixed;
}"
        );
    }

    #[test]
    fn background_clip() {
        assert_eq!(
            generate(["bg-clip-border"], &base_config()),
            ".bg-clip-border {
  background-clip: border-box;
}"
        );
        assert_eq!(
            generate(["bg-clip-padding"], &base_config()),
            ".bg-clip-padding {
  background-clip: padding-box;
}"
        );
        assert_eq!(
            generate(["bg-clip-text"], &base_config()),
            ".bg-clip-text {
  background-clip: text;
}"
        );
    }

    #[test]
    fn background_origin() {
        assert_eq!(
            generate(["bg-origin-padding"], &base_config()),
            ".bg-origin-padding {
  background-origin: padding-box;
}"
        );
    }

    #[test]
    fn background_repeat() {
        assert_eq!(
            generate(["bg-repeat-y"], &base_config()),
            ".bg-repeat-y {
  background-repeat: repeat-y;
}"
        );
    }

    #[test]
    fn background_size() {
        assert_eq!(
            generate(["bg-cover"], &base_config()),
            ".bg-cover {
  background-size: cover;
}"
        );
        assert_eq!(
            generate(["bg-[25%]"], &base_config()),
            r".bg-\[25\%\] {
  background-size: 25%;
}"
        );
    }

    #[test]
    fn background_position() {
        assert_eq!(
            generate(["bg-right-bottom"], &base_config()),
            ".bg-right-bottom {
  background-position: right bottom;
}"
        );
        assert_eq!(
            generate(["bg-[position:25%_100px]"], &base_config()),
            r".bg-\[position\:25\%_100px\] {
  background-position: 25% 100px;
}"
        );
    }

    #[test]
    fn background_image() {
        assert_eq!(
            generate(["bg-gradient-to-b"], &base_config()),
            ".bg-gradient-to-b {
  background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-linear-to-b"], &base_config()),
            ".bg-linear-to-b {
  background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-linear-123"], &base_config()),
            ".bg-linear-123 {
  background-image: linear-gradient(123deg in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-linear-123/hsl"], &base_config()),
            r".bg-linear-123\/hsl {
  background-image: linear-gradient(123deg in hsl, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-linear-to-b/decreasing"], &base_config()),
            r".bg-linear-to-b\/decreasing {
  background-image: linear-gradient(to bottom in oklch decreasing hue, var(--en-gradient-stops));
}"
        );
        assert_eq!(generate(["bg-linear-to-b/d"], &base_config()), "",);
        assert_eq!(generate(["bg-linear-to-b/"], &base_config()), "",);
        assert_eq!(
            generate(["bg-linear-[to_bottom_#000,#fff]"], &base_config()),
            r".bg-linear-\[to_bottom_\#000\,\#fff\] {
  background-image: linear-gradient(to bottom #000,#fff);
}"
        );
        assert_eq!(
            generate(["bg-[url(&#39;/hello.png&#39;)]"], &base_config()),
            r".bg-\[url\(\'\/hello\.png\'\)\] {
  background-image: url('/hello.png');
}"
        );
        assert_eq!(
            generate(
                ["bg-[url(&#39;/hello_with_underscores.png&#39;)]"],
                &base_config()
            ),
            r".bg-\[url\(\'\/hello_with_underscores\.png\'\)\] {
  background-image: url('/hello_with_underscores.png');
}"
        );
        assert_eq!(
            generate(["bg-radial"], &base_config()),
            ".bg-radial {
  background-image: radial-gradient(in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-radial-[at_50%_75%]"], &base_config()),
            r".bg-radial-\[at_50\%_75\%\] {
  background-image: radial-gradient(at 50% 75%);
}"
        );
        assert_eq!(
            generate(["bg-conic"], &base_config()),
            ".bg-conic {
  background-image: conic-gradient(in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-conic/decreasing"], &base_config()),
            r".bg-conic\/decreasing {
  background-image: conic-gradient(in oklch decreasing hue, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-conic-180"], &base_config()),
            ".bg-conic-180 {
  background-image: conic-gradient(from 180deg in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["-bg-conic-180"], &base_config()),
            ".-bg-conic-180 {
  background-image: conic-gradient(from -180deg in oklab, var(--en-gradient-stops));
}"
        );
        assert_eq!(
            generate(["bg-none"], &base_config()),
            ".bg-none {
  background-image: none;
}"
        );
        assert!(generate(["bg-none/decreasing"], &base_config()).is_empty(),);
    }

    #[test]
    fn background_color() {
        assert_eq!(
            generate(["bg-red-400"], &base_config()),
            ".bg-red-400 {
  background-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["bg-[rgb(12,12,12)]"], &base_config()),
            r".bg-\[rgb\(12\,12\,12\)\] {
  background-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["bg-[purple]"], &base_config()),
            r".bg-\[purple\] {
  background-color: purple;
}"
        );
        assert_eq!(
            generate(["bg-[color:var(--primary)]"], &base_config()),
            r".bg-\[color\:var\(--primary\)\] {
  background-color: var(--primary);
}"
        );
    }

    #[test]
    fn gradient_color_stops() {
        assert_eq!(
            generate(["from-red-200"], &base_config()),
            r".from-red-200 {
  --en-gradient-from: oklch(88.5% .062 18.334);
  --en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);
}"
        );

        assert_eq!(
            generate(["from-[#f00]"], &base_config()),
            r".from-\[\#f00\] {
  --en-gradient-from: #f00;
  --en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);
}"
        );

        assert_eq!(
            generate(["via-blue-200"], &base_config()),
            r".via-blue-200 {
  --en-gradient-stops: var(--en-gradient-from), oklch(88.2% .059 254.128), var(--en-gradient-to, transparent);
}"
        );

        assert_eq!(
            generate(["via-[rgb(12,12,12)]"], &base_config()),
            r".via-\[rgb\(12\,12\,12\)\] {
  --en-gradient-stops: var(--en-gradient-from), rgb(12,12,12), var(--en-gradient-to, transparent);
}"
        );

        assert_eq!(
            generate(["to-green-200"], &base_config()),
            r".to-green-200 {
  --en-gradient-to: oklch(92.5% .084 155.995);
}"
        );

        assert_eq!(
            generate(["to-[hwb(61_0%_0%)]"], &base_config()),
            r".to-\[hwb\(61_0\%_0\%\)\] {
  --en-gradient-to: hwb(61 0% 0%);
}"
        );
    }
}

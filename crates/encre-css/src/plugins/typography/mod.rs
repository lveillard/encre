//! Typography utilities
pub mod content;
pub mod font_family;
pub mod font_size;
pub mod font_smoothing;
pub mod font_style;
pub mod font_variant_numeric;
pub mod font_weight;
pub mod letter_spacing;
pub mod line_clamp;
pub mod line_height;
pub mod list_style_position;
pub mod list_style_type;
pub mod text_align;
pub mod text_color;
pub mod text_decoration;
pub mod text_decoration_color;
pub mod text_decoration_style;
pub mod text_decoration_thickness;
pub mod text_indent;
pub mod text_overflow;
pub mod text_transform;
pub mod text_underline_offset;
pub mod vertical_align;
pub mod whitespace;
pub mod text_wrap;
pub mod word_break;

#[cfg(test)]
mod tests {
    use crate::{generate, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn content() {
        assert_eq!(
            generate(["before:content-none"], &base_config()),
            r".before\:content-none::before {
  --en-content: none;
  content: var(--en-content);
}"
        );
        assert_eq!(
            generate(["before:content-[&#39;1234_some_words&#39;]"], &base_config()),
            r".before\:content-\[\'1234_some_words\'\]::before {
  --en-content: '1234 some words';
  content: var(--en-content);
}"
        );
        assert_eq!(
            generate(["before:content-[&#39;:-><-:&#39;]"], &base_config()),
            r".before\:content-\[\'\:-\>\<-\:\'\]::before {
  --en-content: ':-><-:';
  content: var(--en-content);
}"
        );
        assert_eq!(
            generate(["before:content-[&#39;&#91;inside&#93;&#39;]"], &base_config()),
            r".before\:content-\[\'\[inside\]\'\]::before {
  --en-content: '[inside]';
  content: var(--en-content);
}"
        );
    }

    #[test]
    fn font_family() {
        assert_eq!(
            generate(["font-mono"], &base_config()),
            r#".font-mono {
  font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;
}"#
        );
        assert_eq!(
            generate(["font-[&#39;Open_Sans&#39;,Roboto,sans-serif]"], &base_config()),
            r".font-\[\'Open_Sans\'\,Roboto\,sans-serif\] {
  font-family: 'Open Sans',Roboto,sans-serif;
}"
        );
        assert_eq!(
            generate(["font-[\u{fb17}\u{fb17}]"], &base_config()),
            ".font-\\[\u{fb17}\u{fb17}\\] {
  font-family: \u{fb17}\u{fb17};
}"
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn font_size() {
        assert_eq!(
            generate(["text-xs"], &base_config()),
            ".text-xs {
  font-size: 0.75rem;
  line-height: 1rem;
}"
        );
        assert_eq!(
            generate(["text-sm"], &base_config()),
            ".text-sm {
  font-size: 0.875rem;
  line-height: 1.25rem;
}"
        );
        assert_eq!(
            generate(["text-base"], &base_config()),
            ".text-base {
  font-size: 1rem;
  line-height: 1.5rem;
}"
        );
        assert_eq!(
            generate(["text-lg"], &base_config()),
            ".text-lg {
  font-size: 1.125rem;
  line-height: 1.75rem;
}"
        );
        assert_eq!(
            generate(["text-xl"], &base_config()),
            ".text-xl {
  font-size: 1.25rem;
  line-height: 1.75rem;
}"
        );
        assert_eq!(
            generate(["text-2xl"], &base_config()),
            ".text-2xl {
  font-size: 1.5rem;
  line-height: 2rem;
}"
        );
        assert_eq!(
            generate(["text-3xl"], &base_config()),
            ".text-3xl {
  font-size: 1.875rem;
  line-height: 2.25rem;
}"
        );
        assert_eq!(
            generate(["text-4xl"], &base_config()),
            ".text-4xl {
  font-size: 2.25rem;
  line-height: 2.5rem;
}"
        );
        assert_eq!(
            generate(["text-5xl"], &base_config()),
            ".text-5xl {
  font-size: 3rem;
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["text-6xl"], &base_config()),
            ".text-6xl {
  font-size: 3.75rem;
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["text-7xl"], &base_config()),
            ".text-7xl {
  font-size: 4.5rem;
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["text-8xl"], &base_config()),
            ".text-8xl {
  font-size: 6rem;
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["text-9xl"], &base_config()),
            ".text-9xl {
  font-size: 8rem;
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["text-[18px]"], &base_config()),
            r".text-\[18px\] {
  font-size: 18px;
}"
        );
        assert_eq!(
            generate(["text-[10%]"], &base_config()),
            r".text-\[10\%\] {
  font-size: 10%;
}"
        );
        assert_eq!(
            generate(["text-[x-large]"], &base_config()),
            r".text-\[x-large\] {
  font-size: x-large;
}"
        );
        assert_eq!(
            generate(["text-[smaller]"], &base_config()),
            r".text-\[smaller\] {
  font-size: smaller;
}"
        );
    }

    #[test]
    fn font_smoothing() {
        assert_eq!(
            generate(["antialised"], &base_config()),
            ".antialised {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}"
        );
        assert_eq!(
            generate(["subpixel-antialised"], &base_config()),
            ".subpixel-antialised {
  -webkit-font-smoothing: auto;
  -moz-osx-font-smoothing: auto;
}"
        );
    }

    #[test]
    fn font_style() {
        assert_eq!(
            generate(["italic"], &base_config()),
            ".italic {
  font-style: italic;
}"
        );
        assert_eq!(
            generate(["not-italic"], &base_config()),
            ".not-italic {
  font-style: normal;
}"
        );
    }

    #[test]
    fn font_variant_numeric() {
        assert_eq!(
            generate(["normal-nums"], &base_config()),
            ".normal-nums {
  font-variant-numeric: normal;
}"
        );
        assert_eq!(
            generate(["ordinal"], &base_config()),
            ".ordinal {
  --en-ordinal: ordinal;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["slashed-zero"], &base_config()),
            ".slashed-zero {
  --en-slashed-zero: slashed-zero;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["lining-nums"], &base_config()),
            ".lining-nums {
  --en-numeric-figure: lining-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["oldstyle-nums"], &base_config()),
            ".oldstyle-nums {
  --en-numeric-figure: oldstyle-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["proportional-nums"], &base_config()),
            ".proportional-nums {
  --en-numeric-spacing: proportional-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["tabular-nums"], &base_config()),
            ".tabular-nums {
  --en-numeric-spacing: tabular-nums;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["diagonal-fractions"], &base_config()),
            ".diagonal-fractions {
  --en-numeric-fraction: diagonal-fractions;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
        assert_eq!(
            generate(["stacked-fractions"], &base_config()),
            ".stacked-fractions {
  --en-numeric-fraction: stacked-fractions;
  font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);
}"
        );
    }

    #[test]
    fn font_weight() {
        assert_eq!(
            generate(["font-thin"], &base_config()),
            ".font-thin {
  font-weight: 100;
}"
        );
        assert_eq!(
            generate(["font-extralight"], &base_config()),
            ".font-extralight {
  font-weight: 200;
}"
        );
        assert_eq!(
            generate(["font-light"], &base_config()),
            ".font-light {
  font-weight: 300;
}"
        );
        assert_eq!(
            generate(["font-normal"], &base_config()),
            ".font-normal {
  font-weight: 400;
}"
        );
        assert_eq!(
            generate(["font-medium"], &base_config()),
            ".font-medium {
  font-weight: 500;
}"
        );
        assert_eq!(
            generate(["font-semibold"], &base_config()),
            ".font-semibold {
  font-weight: 600;
}"
        );
        assert_eq!(
            generate(["font-bold"], &base_config()),
            ".font-bold {
  font-weight: 700;
}"
        );
        assert_eq!(
            generate(["font-extrabold"], &base_config()),
            ".font-extrabold {
  font-weight: 800;
}"
        );
        assert_eq!(
            generate(["font-black"], &base_config()),
            ".font-black {
  font-weight: 900;
}"
        );
        assert_eq!(
            generate(["font-[50]"], &base_config()),
            r".font-\[50\] {
  font-weight: 50;
}"
        );
    }

    #[test]
    fn letter_spacing() {
        assert_eq!(
            generate(["tracking-tighter"], &base_config()),
            ".tracking-tighter {
  letter-spacing: -0.05em;
}"
        );
        assert_eq!(
            generate(["tracking-tight"], &base_config()),
            ".tracking-tight {
  letter-spacing: -0.025em;
}"
        );
        assert_eq!(
            generate(["tracking-normal"], &base_config()),
            ".tracking-normal {
  letter-spacing: 0;
}"
        );
        assert_eq!(
            generate(["tracking-wide"], &base_config()),
            ".tracking-wide {
  letter-spacing: 0.025em;
}"
        );
        assert_eq!(
            generate(["tracking-wider"], &base_config()),
            ".tracking-wider {
  letter-spacing: 0.05em;
}"
        );
        assert_eq!(
            generate(["tracking-widest"], &base_config()),
            ".tracking-widest {
  letter-spacing: 0.1em;
}"
        );
        assert_eq!(
            generate(["tracking-[10px]"], &base_config()),
            r".tracking-\[10px\] {
  letter-spacing: 10px;
}"
        );
    }

    #[test]
    fn line_clamp() {
        assert_eq!(
            generate(["line-clamp-none"], &base_config()),
            ".line-clamp-none {
  -webkit-line-clamp: unset;
}"
        );
        assert_eq!(
            generate(["line-clamp-12"], &base_config()),
            ".line-clamp-12 {
  -webkit-line-clamp: 12;
  overflow: hidden;
  display: -webkit-box;
  -webkit-box-orient: vertical;
}"
        );
    }

    #[test]
    fn line_height() {
        assert_eq!(
            generate(["leading-none"], &base_config()),
            ".leading-none {
  line-height: 1;
}"
        );
        assert_eq!(
            generate(["leading-tight"], &base_config()),
            ".leading-tight {
  line-height: 1.25;
}"
        );
        assert_eq!(
            generate(["leading-snug"], &base_config()),
            ".leading-snug {
  line-height: 1.375;
}"
        );
        assert_eq!(
            generate(["leading-normal"], &base_config()),
            ".leading-normal {
  line-height: 1.5;
}"
        );
        assert_eq!(
            generate(["leading-relaxed"], &base_config()),
            ".leading-relaxed {
  line-height: 1.625;
}"
        );
        assert_eq!(
            generate(["leading-loose"], &base_config()),
            ".leading-loose {
  line-height: 2;
}"
        );
        assert_eq!(
            generate(["leading-4"], &base_config()),
            ".leading-4 {
  line-height: 1rem;
}"
        );
        assert_eq!(
            generate(["-leading-4"], &base_config()),
            ".-leading-4 {
  line-height: -1rem;
}"
        );
        assert_eq!(
            generate(["leading-1/2"], &base_config()),
            r".leading-1\/2 {
  line-height: 50%;
}"
        );
        assert_eq!(
            generate(["leading-[8]"], &base_config()),
            r".leading-\[8\] {
  line-height: 8;
}"
        );
        assert_eq!(
            generate(["leading-[16px]"], &base_config()),
            r".leading-\[16px\] {
  line-height: 16px;
}"
        );
        assert_eq!(
            generate(["leading-[22%]"], &base_config()),
            r".leading-\[22\%\] {
  line-height: 22%;
}"
        );
    }

    #[test]
    fn list_style_position() {
        assert_eq!(
            generate(["list-inside"], &base_config()),
            ".list-inside {
  list-style-position: inside;
}"
        );
        assert_eq!(
            generate(["list-outside"], &base_config()),
            ".list-outside {
  list-style-position: outside;
}"
        );
    }

    #[test]
    fn list_style_type() {
        assert_eq!(
            generate(["list-disc"], &base_config()),
            ".list-disc {
  list-style-type: disc;
}"
        );
        assert_eq!(
            generate(["list-decimal"], &base_config()),
            ".list-decimal {
  list-style-type: decimal;
}"
        );
        assert_eq!(
            generate(["list-none"], &base_config()),
            ".list-none {
  list-style-type: none;
}"
        );
        assert_eq!(
            generate(["list-[greek]"], &base_config()),
            r".list-\[greek\] {
  list-style-type: greek;
}"
        );
    }

    #[test]
    fn text_align() {
        assert_eq!(
            generate(["text-center"], &base_config()),
            ".text-center {
  text-align: center;
}"
        );
        assert_eq!(
            generate(["text-justify"], &base_config()),
            ".text-justify {
  text-align: justify;
}"
        );
    }

    #[test]
    fn text_color() {
        assert_eq!(
            generate(["text-red-400"], &base_config()),
            ".text-red-400 {
  color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["text-[rgb(12,12,12)]"], &base_config()),
            r".text-\[rgb\(12\,12\,12\)\] {
  color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["text-[purple]"], &base_config()),
            r".text-\[purple\] {
  color: purple;
}"
        );
    }

    #[test]
    fn text_decoration() {
        assert_eq!(
            generate(["underline"], &base_config()),
            ".underline {
  -webkit-text-decoration-line: underline;
  text-decoration-line: underline;
}"
        );
        assert_eq!(
            generate(["no-underline"], &base_config()),
            ".no-underline {
  -webkit-text-decoration-line: none;
  text-decoration-line: none;
}"
        );
    }

    #[test]
    fn text_decoration_color() {
        assert_eq!(
            generate(["decoration-red-400"], &base_config()),
            ".decoration-red-400 {
  -webkit-text-decoration-color: oklch(70.4% .191 22.216);
  text-decoration-color: oklch(70.4% .191 22.216);
}"
        );
        assert_eq!(
            generate(["decoration-[rgb(12,12,12)]"], &base_config()),
            r".decoration-\[rgb\(12\,12\,12\)\] {
  -webkit-text-decoration-color: rgb(12,12,12);
  text-decoration-color: rgb(12,12,12);
}"
        );
        assert_eq!(
            generate(["decoration-[purple]"], &base_config()),
            r".decoration-\[purple\] {
  -webkit-text-decoration-color: purple;
  text-decoration-color: purple;
}"
        );
    }

    #[test]
    fn text_decoration_style() {
        assert_eq!(
            generate(["decoration-wavy"], &base_config()),
            ".decoration-wavy {
  text-decoration-style: wavy;
}"
        );
    }

    #[test]
    fn text_decoration_thickness() {
        assert_eq!(
            generate(["decoration-auto"], &base_config()),
            ".decoration-auto {
  text-decoration-thickness: auto;
}"
        );
        assert_eq!(
            generate(["decoration-from-font"], &base_config()),
            ".decoration-from-font {
  text-decoration-thickness: from-font;
}"
        );
        assert_eq!(
            generate(["decoration-12"], &base_config()),
            ".decoration-12 {
  text-decoration-thickness: 12px;
}"
        );
        assert_eq!(
            generate(["decoration-[4.2rem]"], &base_config()),
            r".decoration-\[4\.2rem\] {
  text-decoration-thickness: 4.2rem;
}"
        );
        assert_eq!(
            generate(["decoration-[2%]"], &base_config()),
            r".decoration-\[2\%\] {
  text-decoration-thickness: 2%;
}"
        );
    }

    #[test]
    fn text_indent() {
        assert_eq!(
            generate(["indent-2"], &base_config()),
            ".indent-2 {
  text-indent: 0.5rem;
}"
        );
        assert_eq!(
            generate(["-indent-2"], &base_config()),
            ".-indent-2 {
  text-indent: -0.5rem;
}"
        );
        assert_eq!(
            generate(["indent-[20px]"], &base_config()),
            r".indent-\[20px\] {
  text-indent: 20px;
}"
        );
    }

    #[test]
    fn text_overflow() {
        assert_eq!(
            generate(["truncate"], &base_config()),
            ".truncate {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}"
        );
        assert_eq!(
            generate(["text-ellipsis"], &base_config()),
            ".text-ellipsis {
  text-overflow: ellipsis;
}"
        );
        assert_eq!(
            generate(["text-clip"], &base_config()),
            ".text-clip {
  text-overflow: clip;
}"
        );
    }

    #[test]
    fn text_transform() {
        assert_eq!(
            generate(["normal-case"], &base_config()),
            ".normal-case {
  text-transform: none;
}"
        );
        assert_eq!(
            generate(["uppercase"], &base_config()),
            ".uppercase {
  text-transform: uppercase;
}"
        );
    }

    #[test]
    fn text_underline_offset() {
        assert_eq!(
            generate(["underline-offset-auto"], &base_config()),
            ".underline-offset-auto {
  text-underline-offset: auto;
}"
        );
        assert_eq!(
            generate(["underline-offset-12"], &base_config()),
            ".underline-offset-12 {
  text-underline-offset: 12px;
}"
        );
        assert_eq!(
            generate(["underline-offset-[2rem]"], &base_config()),
            r".underline-offset-\[2rem\] {
  text-underline-offset: 2rem;
}"
        );
        assert_eq!(
            generate(["underline-offset-[10%]"], &base_config()),
            r".underline-offset-\[10\%\] {
  text-underline-offset: 10%;
}"
        );
    }

    #[test]
    fn vertical_align() {
        assert_eq!(
            generate(["align-sub"], &base_config()),
            ".align-sub {
  vertical-align: sub;
}"
        );
        assert_eq!(
            generate(["align-text-top"], &base_config()),
            ".align-text-top {
  vertical-align: text-top;
}"
        );
    }

    #[test]
    fn whitespace() {
        assert_eq!(
            generate(["whitespace-normal"], &base_config()),
            ".whitespace-normal {
  white-space: normal;
}"
        );
        assert_eq!(
            generate(["whitespace-pre-wrap"], &base_config()),
            ".whitespace-pre-wrap {
  white-space: pre-wrap;
}"
        );
    }

    #[test]
    fn text_wrap() {
        assert_eq!(
            generate(["text-wrap"], &base_config()),
            ".text-wrap {
  text-wrap: wrap;
}"
        );
        assert_eq!(
            generate(["text-pretty"], &base_config()),
            ".text-pretty {
  text-wrap: pretty;
}"
        );
    }

    #[test]
    fn word_break() {
        assert_eq!(
            generate(["break-normal"], &base_config()),
            ".break-normal {
  overflow-wrap: normal;
  word-break: normal;
}"
        );
        assert_eq!(
            generate(["break-keep"], &base_config()),
            ".break-keep {
  word-break: keep-all;
}"
        );
    }
}

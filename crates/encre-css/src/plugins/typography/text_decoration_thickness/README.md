Utilities for controlling the thickness of text decorations.

<table style="display: table;">
  <thead>
    <tr>
      <th style="text-align: center;">Class</th>
      <th style="text-align: center;">Properties</th>
    </tr>
  </thead>
  <tbody>
    <tr><td>decoration-auto</td><td>text-decoration-thickness: auto;</td></tr>
    <tr><td>decoration-from-font</td><td>text-decoration-thickness: from-font;</td></tr>
    <tr><td>decoration-<i>&lt;integer&gt;</i></td><td>text-decoration-thickness: <i>&lt;integer&gt;</i>px;</td></tr>
  </tbody>
</table>

### Tailwind compatibility

Thickness values don't follow Tailwind's philosophy of limiting possible values and all
numbers are supported. They are however perfectly compatible with Tailwind's values.

### Arbitrary values

Any [`<length>`](crate::utils::value_matchers::is_matching_length) or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage) property or a keyword among `auto` and `from-font` (needs either `length:` or `number:` prefixed as hint before the arbitrary value, ie. `decoration-[length:from-font]`) is allowed as arbitrary value.
For example, `decoration-[12%]`.

[Tailwind reference](https://tailwindcss.com/docs/text-decoration-thickness)

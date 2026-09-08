# Temporary Encre CSS fork

Upstream: https://gitlab.com/encre-org/encre-css

`blitzstore-patches` starts at the published 0.21.0 source commit
`99ffd0f4334cdbffd7aee269296aa01f0b54c325` and adds two independent fixes:

- Close the final quoted font name emitted by `font-sans`.
- Reuse the immutable built-in plugin index between generation calls. Custom
  plugin configurations keep separate indexes.

The two proposal branches target upstream `main` at
`46ca4995dde61e9076bb55385c7f33dcfa743080` and can be reviewed independently.
GitHub Actions runs the library tests on both proposals and the combined branch.

BlitzStore pins an immutable Git revision. Return to the upstream release when it
includes the corrections. The upstream MIT license remains unchanged.

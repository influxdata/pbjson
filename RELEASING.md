# Releasing

This document describes the release process for [`pbjson`], [`pbjson-types`] and
[`pbjson-build`] to crates.io.

Note that `pbjson-test` is not released as it is only used for testing.

[`pbjson`]: https://crates.io/crates/pbjson
[`pbjson-types`]: https://crates.io/crates/pbjson-types
[`pbjson-build`]: https://crates.io/crates/pbjson-build

## Step 1: Update Version

First make a PR to update the version (example [#127](https://github.com/influxdata/pbjson/issues/127))

## Step 2: Release
Run the following commands to release `pbjson`, `pbjson-types` and `pbjson-build`

```shell
cargo publish -p pbjson
cargo publish -p pbjson-build
cargo publish -p pbjson-types
```

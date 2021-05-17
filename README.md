# CucumberWM: My configuration of [UmberWM][1]

## Using this configuration

You will need to:

1. Update `Cargo.toml` to point to the appropriate version of [`umberwm`][1]. Either:
    + Update `Cargo.toml` to point to [`umberwm`][1] on [`crates.io`][2]
      ```toml
      umberwm = "0.0.20"
      ```
      or,

    + Clone [`umberwm`][1] yourself and update `path` to its location on your system.
      ```toml
      umberwm = { path = "/path/to/umberwm" }
      ```

2. Follow the instructions in the `umberwm` [README][3], except that you should use this
   repository's `install.py` script, which will copy/symlink the `cucumberwm` counterparts to `umberwm`, `umberwm-start` and `umberwm.desktop`. There is no major difference between them,
   other than that they reference the `cucumberwm` binary instead of `umberwm`.

## Proceed with caution

This configuration will generally reference my own branch of `umberwm`, which may at times be
incompatible with the `master` branch of `umberwm`.


[1]: https://github.com/yazgoo/umberwm
[2]: https://crates.io/crates/umberwm
[3]: https://github.com/yazgoo/umberwm/blob/master/README.md#using-it

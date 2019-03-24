
# libspnav

[![crates.io](https://img.shields.io/crates/v/libspnav.svg)](https://crates.io/crates/libspnav)

Minimal Rust wrapper around [`libspnav`](https://github.com/FreeSpacenav/libspnav.git) providing a type-safe interface.

[Documentation](https://docs.rs/libspnav)

## Optional features

- `serde-serialize` - Make key event structs serializable.

## Limitations
The wrapper currently only supports the non-X11 based method for communicating with `spacenavd` (UNIX domain socket). X11
support will be considered in the future.

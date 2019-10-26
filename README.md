# Pollua
[![Actions Status](https://github.com/MisterPeModder/Pollua/workflows/CI/badge.svg)](https://github.com/MisterPeModder/Pollua/actions)
=========================

(WIP) rust bindings to Lua 5.3

## Building

### Using embedded Lua
On Windows and Linux:
```
cargo build
```
### Using system Lua

on Windows:
requires vcpkg

```
vcpkg install --triplet x64-windows lua
cargo build
```

### Cargo features:
- **std**: Uses stdlib.
- **embedded-lua**: Links againsts the embedded Lua library.
- **system-lua**: Attempts to link against the system Lua library.
  conflicts with `embedded-lua`

Features `std` and `embedded-lua` are enabled by default

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

# Pollua

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

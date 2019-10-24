# Pollua

(WIP readme)

## Building

see [bindgen requirements](https://github.com/rust-lang/rust-bindgen/blob/master/book/src/requirements.md)

### Windows:

Requirements:
- vcpkg
- Clang v3.9:
    Download and install the official pre-built binary from [the LLVM download page](http://releases.llvm.org/download.html)

```
vcpkg install --triplet x64-windows lua
cargo build
```

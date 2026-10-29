# harmony-luau-sys

Raw Rust bindings and build integration for Luau.

## Source

Luau is pinned in `vendor/luau`. For a fresh repository checkout, initialize the
submodule before building:

```sh
git submodule update --init --recursive
```

Published crate packages include the Luau source files needed by `build.rs`, so
Cargo builds do not fetch Luau during compilation.

## Build

Requirements:

- Rust toolchain
- C++17 compiler
- platform C++ standard library

`build.rs` compiles Luau into static libraries and links them into dependents.
Set `CXX` to choose a C++ compiler, and `CXXSTDLIB` to override the linked C++
standard library when the target default is not correct.

## Included Luau Surface

Harmony compiles plugin source at runtime, so the compiler API is built
unconditionally. Unused Luau subsystems are not preserved as speculative
features: the build excludes the OS/debug/vector/utf8/buffer/int64 library
openers and Luau source groups that only support those libraries. Low-level
debugger and coverage/vector compiler hooks are omitted from the Rust surface
instead of being kept behind feature flags. Higher-level Harmony crates open
only the standard libraries they intentionally expose.

## Modules

- `vm`: raw VM API from `lua.h`
- `auxiliary`: auxiliary API and standard library openers from `lualib.h`
- `compiler`: compiler API from `luacode.h`
- `flags`: `harmony_luau_setfflag`, which sets boolean Luau fast flags by name
- `protected`: `harmony_luau_protectedcall`, a protected call that does not allocate

## C and C++ Dependents

The crate sets `links = "luau"` and exports `DEP_LUAU_INCLUDE` to dependent build
scripts: a list of include directories joined with the platform path separator
(split it with `std::env::split_paths`). Include `harmony_luau.h` instead of
`lua.h`, `lualib.h`, or `luacode.h`; it applies the configuration Luau is
compiled with, such as `LUAI_MAXCSTACK` and C linkage for the API, and declares
the `harmony_luau_*` functions.

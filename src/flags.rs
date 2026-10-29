// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::{c_char, c_int};

unsafe extern "C-unwind" {
    /// Sets the boolean Luau fast flag `name` for the whole process; returns 0 if no linked code
    /// defines it. Flags are unsynchronized globals: set them while no Luau code is running.
    pub fn harmony_luau_setfflag(name: *const c_char, value: c_int) -> c_int;
}

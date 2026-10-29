// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::{c_int, c_void};

use crate::vm::lua_State;

pub type harmony_luau_ProtectedFunction =
    Option<unsafe extern "C-unwind" fn(L: *mut lua_State, ud: *mut c_void)>;

unsafe extern "C-unwind" {
    /// Runs `func(L, ud)` in a protected environment without allocating. On success the values
    /// `func` pushed stay on the stack; on error the stack returns to its height at the call with
    /// the error value pushed. Returns a `lua_Status` code. `func` must not yield.
    pub fn harmony_luau_protectedcall(
        L: *mut lua_State,
        func: harmony_luau_ProtectedFunction,
        ud: *mut c_void,
    ) -> c_int;
}

// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::c_int;

use crate::vm::lua_State;

unsafe extern "C-unwind" {
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn luaL_openlibs(L: *mut lua_State);
    pub fn luaL_sandbox(L: *mut lua_State);
    pub fn luaL_sandboxthread(L: *mut lua_State);
    pub fn luaL_checkvector(L: *mut lua_State, narg: c_int) -> *const f32;
    pub fn luaL_optvector(L: *mut lua_State, narg: c_int, def: *const f32) -> *const f32;

    pub fn luaopen_base(L: *mut lua_State) -> c_int;
    pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
    pub fn luaopen_table(L: *mut lua_State) -> c_int;
    pub fn luaopen_os(L: *mut lua_State) -> c_int;
    pub fn luaopen_string(L: *mut lua_State) -> c_int;
    pub fn luaopen_math(L: *mut lua_State) -> c_int;
    pub fn luaopen_debug(L: *mut lua_State) -> c_int;
    pub fn luaopen_utf8(L: *mut lua_State) -> c_int;
    pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
    pub fn luaopen_buffer(L: *mut lua_State) -> c_int;
    pub fn luaopen_vector(L: *mut lua_State) -> c_int;
    pub fn luaopen_integer(L: *mut lua_State) -> c_int;
}

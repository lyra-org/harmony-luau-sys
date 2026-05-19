// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::{CStr, c_void};
use std::ptr;

use harmony_luau_sys::*;

#[test]
fn compiles_and_runs_luau() {
    unsafe {
        let state = lua_newstate(Some(test_alloc), ptr::null_mut());
        assert!(!state.is_null());

        let source = b"return 40 + 2";
        let mut bytecode_len = 0usize;
        let bytecode = luau_compile(
            source.as_ptr().cast(),
            source.len(),
            ptr::null_mut(),
            &mut bytecode_len,
        );
        assert!(!bytecode.is_null());
        assert!(bytecode_len > 0);

        let status = luau_load(state, c"=smoke".as_ptr(), bytecode, bytecode_len, 0);
        libc::free(bytecode.cast::<c_void>());
        assert_eq!(status, LUA_OK, "luau_load failed");

        let status = lua_pcall(state, 0, 1, 0);
        assert_eq!(status, LUA_OK, "lua_pcall failed");

        let mut isnum = 0;
        let value = lua_tonumberx(state, -1, &mut isnum);
        assert_eq!(isnum, 1);
        assert_eq!(value, 42.0);

        lua_close(state);
    }
}

#[test]
fn reports_luau_syntax_errors_as_bytecode_load_errors() {
    unsafe {
        let state = lua_newstate(Some(test_alloc), ptr::null_mut());
        assert!(!state.is_null());

        let source = b"return function(";
        let mut bytecode_len = 0usize;
        let bytecode = luau_compile(
            source.as_ptr().cast(),
            source.len(),
            ptr::null_mut(),
            &mut bytecode_len,
        );
        assert!(!bytecode.is_null());

        let status = luau_load(state, c"=syntax".as_ptr(), bytecode, bytecode_len, 0);
        libc::free(bytecode.cast::<c_void>());
        assert_ne!(status, LUA_OK);

        let message = lua_tostring(state, -1);
        assert!(!message.is_null());
        let message = CStr::from_ptr(message).to_string_lossy();
        assert!(message.contains("Expected identifier"));

        lua_close(state);
    }
}

unsafe extern "C-unwind" fn test_alloc(
    _ud: *mut c_void,
    ptr: *mut c_void,
    _osize: usize,
    nsize: usize,
) -> *mut c_void {
    if nsize == 0 {
        unsafe {
            libc::free(ptr);
        }
        ptr::null_mut()
    } else {
        unsafe { libc::realloc(ptr, nsize) }
    }
}

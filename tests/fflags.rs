// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

// Flags are process-wide, so this runs in its own test binary.

use std::ffi::c_void;
use std::ptr;

use harmony_luau_sys::{harmony_luau_setfflag, lua_close, lua_newstate};

#[test]
fn sets_flags_of_linked_code_and_reports_unknown_ones() {
    unsafe {
        // Using the VM links the code that defines the flag.
        let state = lua_newstate(Some(alloc), ptr::null_mut());
        assert!(!state.is_null());
        lua_close(state);

        assert_eq!(harmony_luau_setfflag(c"LuauTableRobustOom".as_ptr(), 1), 1);
        assert_eq!(harmony_luau_setfflag(c"NoSuchLuauFlag".as_ptr(), 1), 0);
    }
}

unsafe extern "C-unwind" fn alloc(
    _ud: *mut c_void,
    ptr: *mut c_void,
    _osize: usize,
    nsize: usize,
) -> *mut c_void {
    if nsize == 0 {
        unsafe { libc::free(ptr) };
        ptr::null_mut()
    } else {
        unsafe { libc::realloc(ptr, nsize) }
    }
}

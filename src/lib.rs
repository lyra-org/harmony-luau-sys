// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

pub mod auxiliary;
pub mod compiler;
pub mod protected;
pub mod vm;

pub use auxiliary::*;
pub use compiler::*;
pub use protected::*;
pub use vm::*;

pub const LUAU_GIT_REV: Option<&str> = option_env!("LUAU_GIT_REV");

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, c_void};
    use std::ptr;

    use super::*;

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

    #[test]
    fn userthread_callback_sees_threads_created_and_freed() {
        unsafe extern "C-unwind" fn count_threads(parent: *mut lua_State, thread: *mut lua_State) {
            unsafe {
                let counts = lua_callback_userdata(thread).cast::<[usize; 2]>();
                (*counts)[usize::from(parent.is_null())] += 1;
            }
        }

        unsafe {
            let state = lua_newstate(Some(test_alloc), ptr::null_mut());
            assert!(!state.is_null());
            let mut counts = [0usize; 2];
            lua_set_callback_userdata(state, (&raw mut counts).cast());
            (*lua_callbacks(state)).userthread = Some(count_threads);

            lua_newthread(state);
            lua_pop(state, 1);
            lua_gc(state, LUA_GCCOLLECT, 0);
            assert_eq!(counts, [1, 1]);

            lua_close(state);
        }
    }

    #[test]
    fn protected_call_keeps_results_and_reports_errors_at_the_call_height() {
        unsafe extern "C-unwind" fn push_answer(state: *mut lua_State, _ud: *mut c_void) {
            unsafe { lua_pushnumber(state, 42.0) };
        }

        unsafe extern "C-unwind" fn exhaust_memory(state: *mut lua_State, _ud: *mut c_void) {
            unsafe {
                lua_pushnil(state);
                lua_newbuffer(state, LIMITED_ALLOC_MAX + 1);
            }
        }

        unsafe {
            let state = lua_newstate(Some(limited_alloc), ptr::null_mut());
            assert!(!state.is_null());
            lua_pushnil(state);

            let status = harmony_luau_protectedcall(state, Some(push_answer), ptr::null_mut());
            assert_eq!(status, LUA_OK);
            assert_eq!(lua_gettop(state), 2);
            assert_eq!(lua_tonumberx(state, -1, ptr::null_mut()), 42.0);
            lua_pop(state, 1);

            let status = harmony_luau_protectedcall(state, Some(exhaust_memory), ptr::null_mut());
            assert_eq!(status, LUA_ERRMEM);
            assert_eq!(lua_gettop(state), 2);
            let message = CStr::from_ptr(lua_tostring(state, -1)).to_string_lossy();
            assert_eq!(message, "not enough memory");

            lua_close(state);
        }
    }

    const LIMITED_ALLOC_MAX: usize = 1 << 20;

    unsafe extern "C-unwind" fn limited_alloc(
        ud: *mut c_void,
        ptr: *mut c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut c_void {
        if nsize > LIMITED_ALLOC_MAX {
            return ptr::null_mut();
        }
        unsafe { test_alloc(ud, ptr, osize, nsize) }
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
}

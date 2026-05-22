// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::{c_char, c_double, c_int, c_void};

pub enum lua_State {}

pub type lua_Number = c_double;

pub type lua_CFunction = Option<unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int>;
pub type lua_Continuation =
    Option<unsafe extern "C-unwind" fn(L: *mut lua_State, status: c_int) -> c_int>;
pub type lua_Alloc = Option<
    unsafe extern "C-unwind" fn(
        ud: *mut c_void,
        ptr: *mut c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut c_void,
>;
pub type lua_UserdataDtor = Option<unsafe extern "C-unwind" fn(userdata: *mut c_void)>;
pub type lua_Destructor =
    Option<unsafe extern "C-unwind" fn(L: *mut lua_State, userdata: *mut c_void)>;
pub type lua_InterruptCallback = Option<unsafe extern "C-unwind" fn(L: *mut lua_State, gc: c_int)>;

pub const LUAI_MAXCSTACK: c_int = 1_000_000;
pub const LUA_UTAG_LIMIT: c_int = 128;
pub const LUA_VECTOR_SIZE: c_int = 3;

pub const LUA_MULTRET: c_int = -1;
pub const LUA_REGISTRYINDEX: c_int = -LUAI_MAXCSTACK - 2000;
pub const LUA_GLOBALSINDEX: c_int = -LUAI_MAXCSTACK - 2002;

pub const LUA_OK: c_int = 0;
pub const LUA_YIELD: c_int = 1;

pub const LUA_TNONE: c_int = -1;
pub const LUA_TNIL: c_int = 0;
pub const LUA_TBOOLEAN: c_int = 1;
pub const LUA_TNUMBER: c_int = 3;
pub const LUA_TINTEGER: c_int = 4;
pub const LUA_TVECTOR: c_int = 5;
pub const LUA_TSTRING: c_int = 6;
pub const LUA_TTABLE: c_int = 7;
pub const LUA_TFUNCTION: c_int = 8;
pub const LUA_TUSERDATA: c_int = 9;
pub const LUA_TTHREAD: c_int = 10;
pub const LUA_TBUFFER: c_int = 11;

pub const LUA_GCSTOP: c_int = 0;
pub const LUA_GCRESTART: c_int = 1;
pub const LUA_GCCOLLECT: c_int = 2;
pub const LUA_GCCOUNT: c_int = 3;
pub const LUA_GCCOUNTB: c_int = 4;
pub const LUA_GCISRUNNING: c_int = 5;
pub const LUA_GCSTEP: c_int = 6;
pub const LUA_GCSETGOAL: c_int = 7;
pub const LUA_GCSETSTEPMUL: c_int = 8;
pub const LUA_GCSETSTEPSIZE: c_int = 9;

#[repr(C)]
struct RawLuaCallbacks {
    userdata: *mut c_void,
    interrupt: lua_InterruptCallback,
}

unsafe extern "C-unwind" {
    pub fn lua_newstate(f: lua_Alloc, ud: *mut c_void) -> *mut lua_State;
    pub fn lua_close(L: *mut lua_State);
    pub fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    pub fn lua_absindex(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_checkstack(L: *mut lua_State, sz: c_int) -> c_int;
    pub fn lua_rawcheckstack(L: *mut lua_State, sz: c_int);
    pub fn lua_gettop(L: *mut lua_State) -> c_int;
    pub fn lua_settop(L: *mut lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *mut lua_State, idx: c_int);
    pub fn lua_replace(L: *mut lua_State, idx: c_int);
    pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

    pub fn lua_type(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_rawequal(L: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;
    pub fn lua_toboolean(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tointeger64(L: *mut lua_State, idx: c_int, isinteger: *mut c_int) -> i64;
    pub fn lua_tovector(L: *mut lua_State, idx: c_int) -> *const f32;
    pub fn lua_tolstring(L: *mut lua_State, idx: c_int, len: *mut usize) -> *const c_char;
    pub fn lua_touserdata(L: *mut lua_State, idx: c_int) -> *mut c_void;
    pub fn lua_touserdatatagged(L: *mut lua_State, idx: c_int, tag: c_int) -> *mut c_void;
    pub fn lua_userdatatag(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_tothread(L: *mut lua_State, idx: c_int) -> *mut lua_State;
    pub fn lua_tobuffer(L: *mut lua_State, idx: c_int, len: *mut usize) -> *mut c_void;

    pub fn lua_pushnil(L: *mut lua_State);
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_pushinteger64(L: *mut lua_State, n: i64);
    pub fn lua_pushvector(L: *mut lua_State, x: f32, y: f32, z: f32);
    pub fn lua_pushlstring(L: *mut lua_State, s: *const c_char, l: usize);
    pub fn lua_pushstring(L: *mut lua_State, s: *const c_char);
    pub fn lua_pushcclosurek(
        L: *mut lua_State,
        func: lua_CFunction,
        debugname: *const c_char,
        nup: c_int,
        cont: lua_Continuation,
    );
    pub fn lua_pushboolean(L: *mut lua_State, b: c_int);
    pub fn lua_pushthread(L: *mut lua_State) -> c_int;
    pub fn lua_newuserdatadtor(L: *mut lua_State, sz: usize, dtor: lua_UserdataDtor)
    -> *mut c_void;
    pub fn lua_newuserdatatagged(L: *mut lua_State, sz: usize, tag: c_int) -> *mut c_void;
    pub fn lua_newuserdatataggedwithmetatable(
        L: *mut lua_State,
        sz: usize,
        tag: c_int,
    ) -> *mut c_void;
    pub fn lua_newbuffer(L: *mut lua_State, sz: usize) -> *mut c_void;

    pub fn lua_rawgetfield(L: *mut lua_State, idx: c_int, k: *const c_char) -> c_int;
    pub fn lua_rawgeti(L: *mut lua_State, idx: c_int, n: c_int) -> c_int;
    pub fn lua_createtable(L: *mut lua_State, narr: c_int, nrec: c_int);
    pub fn lua_setreadonly(L: *mut lua_State, idx: c_int, enabled: c_int);
    pub fn lua_setsafeenv(L: *mut lua_State, idx: c_int, enabled: c_int);
    pub fn lua_getmetatable(L: *mut lua_State, objindex: c_int) -> c_int;

    pub fn lua_setfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_rawsetfield(L: *mut lua_State, idx: c_int, k: *const c_char);
    pub fn lua_rawset(L: *mut lua_State, idx: c_int);
    pub fn lua_rawseti(L: *mut lua_State, idx: c_int, n: c_int);
    pub fn lua_setmetatable(L: *mut lua_State, objindex: c_int) -> c_int;
    pub fn lua_setuserdatatag(L: *mut lua_State, idx: c_int, tag: c_int);
    pub fn lua_setuserdatadtor(L: *mut lua_State, tag: c_int, dtor: lua_Destructor);
    pub fn lua_getuserdatadtor(L: *mut lua_State, tag: c_int) -> lua_Destructor;
    pub fn lua_setuserdatametatable(L: *mut lua_State, tag: c_int);
    pub fn lua_getuserdatametatable(L: *mut lua_State, tag: c_int);

    pub fn luau_load(
        L: *mut lua_State,
        chunkname: *const c_char,
        data: *const c_char,
        size: usize,
        env: c_int,
    ) -> c_int;
    pub fn lua_call(L: *mut lua_State, nargs: c_int, nresults: c_int);
    pub fn lua_pcall(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;

    pub fn lua_yield(L: *mut lua_State, nresults: c_int) -> c_int;
    pub fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: c_int) -> c_int;
    pub fn lua_status(L: *mut lua_State) -> c_int;
    pub fn lua_isyieldable(L: *mut lua_State) -> c_int;

    pub fn lua_error(L: *mut lua_State) -> !;
    pub fn lua_next(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_ref(L: *mut lua_State, idx: c_int) -> c_int;
    pub fn lua_unref(L: *mut lua_State, reference: c_int);
    pub fn lua_gc(L: *mut lua_State, what: c_int, data: c_int) -> c_int;
    pub fn lua_totalbytes(L: *mut lua_State, category: c_int) -> usize;

    #[link_name = "lua_callbacks"]
    fn lua_callbacks_raw(L: *mut lua_State) -> *mut RawLuaCallbacks;
}

#[inline]
pub const fn lua_upvalueindex(i: c_int) -> c_int {
    LUA_GLOBALSINDEX - i
}

#[inline]
pub unsafe fn lua_pop(L: *mut lua_State, n: c_int) {
    unsafe { lua_settop(L, -n - 1) };
}

#[inline]
pub unsafe fn lua_newtable(L: *mut lua_State) {
    unsafe { lua_createtable(L, 0, 0) };
}

#[inline]
pub unsafe fn lua_pushcfunction(L: *mut lua_State, func: lua_CFunction, debugname: *const c_char) {
    unsafe { lua_pushcclosurek(L, func, debugname, 0, None) };
}

#[inline]
pub unsafe fn lua_pushcclosure(
    L: *mut lua_State,
    func: lua_CFunction,
    debugname: *const c_char,
    nup: c_int,
) {
    unsafe { lua_pushcclosurek(L, func, debugname, nup, None) };
}

#[inline]
pub unsafe fn lua_setglobal(L: *mut lua_State, name: *const c_char) {
    unsafe { lua_setfield(L, LUA_GLOBALSINDEX, name) };
}

#[inline]
pub unsafe fn lua_getref(L: *mut lua_State, reference: c_int) -> c_int {
    unsafe { lua_rawgeti(L, LUA_REGISTRYINDEX, reference) }
}

#[inline]
pub unsafe fn lua_tostring(L: *mut lua_State, idx: c_int) -> *const c_char {
    unsafe { lua_tolstring(L, idx, std::ptr::null_mut()) }
}

#[inline]
pub unsafe fn lua_set_interrupt_callback(L: *mut lua_State, callback: lua_InterruptCallback) {
    unsafe {
        (*lua_callbacks_raw(L)).interrupt = callback;
    }
}

#[inline]
pub unsafe fn lua_interrupt_callback(L: *mut lua_State) -> lua_InterruptCallback {
    unsafe { (*lua_callbacks_raw(L)).interrupt }
}

#[inline]
pub unsafe fn lua_set_callback_userdata(L: *mut lua_State, userdata: *mut c_void) {
    unsafe {
        (*lua_callbacks_raw(L)).userdata = userdata;
    }
}

#[inline]
pub unsafe fn lua_callback_userdata(L: *mut lua_State) -> *mut c_void {
    unsafe { (*lua_callbacks_raw(L)).userdata }
}

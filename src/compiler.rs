// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::ffi::{c_char, c_int};

#[repr(C)]
pub struct lua_CompileOptions {
    pub optimizationLevel: c_int,
    pub debugLevel: c_int,
    pub typeInfoLevel: c_int,
    pub mutableGlobals: *const *const c_char,
    pub disabledBuiltins: *const *const c_char,
}

#[repr(C)]
struct RawCompileOptions {
    optimizationLevel: c_int,
    debugLevel: c_int,
    typeInfoLevel: c_int,
    coverageLevel: c_int,
    vectorLib: *const c_char,
    vectorCtor: *const c_char,
    vectorType: *const c_char,
    vectorPrecision: c_int,
    mutableGlobals: *const *const c_char,
    userdataTypes: *const *const c_char,
    librariesWithKnownMembers: *const *const c_char,
    libraryMemberTypeCb:
        Option<unsafe extern "C-unwind" fn(library: *const c_char, member: *const c_char) -> c_int>,
    libraryMemberConstantCb: Option<
        unsafe extern "C-unwind" fn(
            library: *const c_char,
            member: *const c_char,
            constant: *mut std::ffi::c_void,
        ),
    >,
    disabledBuiltins: *const *const c_char,
}

impl lua_CompileOptions {
    fn to_raw(&self) -> RawCompileOptions {
        RawCompileOptions {
            optimizationLevel: self.optimizationLevel,
            debugLevel: self.debugLevel,
            typeInfoLevel: self.typeInfoLevel,
            coverageLevel: 0,
            vectorLib: std::ptr::null(),
            vectorCtor: std::ptr::null(),
            vectorType: std::ptr::null(),
            vectorPrecision: 0,
            mutableGlobals: self.mutableGlobals,
            userdataTypes: std::ptr::null(),
            librariesWithKnownMembers: std::ptr::null(),
            libraryMemberTypeCb: None,
            libraryMemberConstantCb: None,
            disabledBuiltins: self.disabledBuiltins,
        }
    }
}

unsafe extern "C-unwind" {
    #[link_name = "luau_compile"]
    fn luau_compile_raw(
        source: *const c_char,
        size: usize,
        options: *mut RawCompileOptions,
        outsize: *mut usize,
    ) -> *mut c_char;
}

pub unsafe fn luau_compile(
    source: *const c_char,
    size: usize,
    options: *mut lua_CompileOptions,
    outsize: *mut usize,
) -> *mut c_char {
    if options.is_null() {
        unsafe { luau_compile_raw(source, size, std::ptr::null_mut(), outsize) }
    } else {
        let mut raw = unsafe { (*options).to_raw() };
        unsafe { luau_compile_raw(source, size, &mut raw, outsize) }
    }
}

/// Re-evaluate before stabilizing this helper. It is useful for the current
/// safe wrapper, but a future sys API may expose ownership more explicitly.
#[inline]
pub unsafe fn luau_free_compiled_bytecode(bytecode: *mut c_char) {
    unsafe { libc::free(bytecode.cast()) };
}

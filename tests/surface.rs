// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::fs;
use std::path::Path;

#[test]
fn build_drops_unused_native_libraries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let build = fs::read_to_string(root.join("build.rs")).expect("read build.rs");

    for file in [
        "lbuflib.cpp",
        "ldblib.cpp",
        "lgcdebug.cpp",
        "linit.cpp",
        "lintlib.cpp",
        "loslib.cpp",
        "lutf8lib.cpp",
        "lveclib.cpp",
    ] {
        assert!(
            build.contains(file),
            "build.rs should explicitly exclude {file}"
        );
    }

    assert!(
        !build.contains("LUA_VECTOR_SIZE"),
        "vector support should not be compiled when Harmony does not expose it"
    );
}

#[test]
fn bindings_do_not_expose_dropped_luau_surfaces() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let vm = fs::read_to_string(root.join("src/vm.rs")).expect("read src/vm.rs");
    let auxiliary =
        fs::read_to_string(root.join("src/auxiliary.rs")).expect("read src/auxiliary.rs");
    let compiler = fs::read_to_string(root.join("src/compiler.rs")).expect("read src/compiler.rs");

    for symbol in [
        "lua_clock",
        "lua_getallocf",
        "lua_setmemcat",
        "lua_totalbytes",
        "LUA_MINSTACK",
        "LUA_BUFFERSIZE",
        "LUA_ENVIRONINDEX",
        "LUA_ERRRUN",
        "LUA_CORUN",
        "LUA_TLIGHTUSERDATA",
        "LUA_TUSERDATA",
        "LUA_GCSTOP",
        "LUA_NOREF",
        "LUA_BREAK",
        "lua_Debug",
        "lua_Callbacks",
        "lua_Hook",
        "lua_break",
        "lua_breakpoint",
        "lua_callbacks",
        "lua_debugtrace",
        "lua_getcoverage",
        "lua_getinfo",
        "lua_getlocal",
        "lua_getupvalue",
        "lua_getcounters",
        "LUA_TVECTOR",
        "lua_pushvector",
        "lua_setlocal",
        "lua_setupvalue",
        "lua_stackdepth",
        "lua_tovector",
        "LUA_UTAG_LIMIT",
        "LUA_LUTAG_LIMIT",
        "lua_UserdataDirectAccess",
        "lua_UserdataDirectNamecall",
        "lua_UserdataDirectFieldGet",
        "lua_isuserdata",
        "lua_tolightuserdata",
        "lua_tolightuserdatatagged",
        "lua_touserdatatagged",
        "lua_userdatatag",
        "lua_lightuserdatatag",
        "lua_pushlightuserdata",
        "lua_pushlightuserdatatagged",
        "fn lua_newuserdata(",
        "lua_newuserdatatagged",
        "lua_newuserdatataggedwithmetatable",
        "lua_setuserdatatag",
        "lua_setuserdatametatable",
        "lua_getuserdatametatable",
        "lua_registeruserdatadirectaccess",
        "lua_registeruserdatadirectfieldget",
        "lua_userdatadirectfield_setnumber",
        "lua_userdatadirectfield_setboolean",
        "lua_userdatadirectfield_setinteger64",
        "lua_userdatadirectfield_setnil",
        "lua_userdatadirectfield_setvector",
        "lua_setlightuserdataname",
        "lua_getlightuserdataname",
        "lua_rawgetp",
        "lua_rawsetp",
        "lua_rawgetptagged",
        "lua_rawsetptagged",
        "lua_mainthread",
        "lua_resetthread",
        "lua_isthreadreset",
        "lua_remove",
        "lua_insert",
        "lua_checkstack",
        "lua_rawcheckstack",
        "lua_xpush",
        "lua_isnumber",
        "lua_isstring",
        "lua_isinteger64",
        "lua_iscfunction",
        "lua_isLfunction",
        "lua_typename",
        "lua_equal",
        "lua_lessthan",
        "lua_tointegerx",
        "lua_tounsignedx",
        "lua_tostringatom",
        "lua_tolstringatom",
        "lua_namecallatom",
        "lua_objlen",
        "lua_tocfunction",
        "lua_topointer",
        "fn lua_pushinteger(",
        "lua_pushunsigned",
        "lua_gettable",
        "lua_getfield",
        "lua_getreadonly",
        "lua_getfenv",
        "lua_settable",
        "lua_setfenv",
        "lua_cpcall",
        "lua_resumeerror",
        "lua_getthreaddata",
        "lua_setthreaddata",
        "lua_costatus",
        "lua_gc",
        "lua_rawiter",
        "lua_concat",
        "lua_encodepointer",
        "lua_clonefunction",
        "lua_cleartable",
        "lua_clonetable",
        "lua_ispseudo",
        "fn lua_tonumber(",
        "fn lua_tointeger(",
        "fn lua_tounsigned(",
    ] {
        assert!(!vm.contains(symbol), "{symbol} should not be exposed");
    }

    for symbol in [
        "luaL_",
        "luaL_openlibs",
        "luaL_checkvector",
        "luaL_optvector",
        "luaopen_buffer",
        "luaopen_debug",
        "luaopen_integer",
        "luaopen_os",
        "luaopen_utf8",
        "luaopen_vector",
        "luaL_sandbox",
        "luaL_sandboxthread",
    ] {
        assert!(
            !auxiliary.contains(symbol),
            "{symbol} should not be exposed"
        );
    }

    assert!(
        !compiler.contains("luau_set_compile_constant_vector"),
        "luau_set_compile_constant_vector should not be exposed"
    );
    let public_compile_options = compiler
        .split("pub struct lua_CompileOptions")
        .nth(1)
        .and_then(|rest| rest.split("}").next())
        .expect("public lua_CompileOptions struct should exist");
    for symbol in [
        "coverageLevel: c_int",
        "vectorLib: *const c_char",
        "vectorCtor: *const c_char",
        "vectorType: *const c_char",
    ] {
        assert!(
            !public_compile_options.contains(symbol),
            "{symbol} should not be exposed through lua_CompileOptions"
        );
    }
}

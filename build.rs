// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=CXXSTDLIB");

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let luau_dir = manifest_dir.join("vendor/luau");

    if !luau_dir.join("VM/include/lua.h").exists() {
        panic!("Luau sources are missing; expected {}", luau_dir.display());
    }

    rerun_dir(&luau_dir.join("Common/include"));
    rerun_dir(&luau_dir.join("Common/src"));
    rerun_dir(&luau_dir.join("Bytecode/include"));
    rerun_dir(&luau_dir.join("Bytecode/src"));
    rerun_dir(&luau_dir.join("Ast/include"));
    rerun_dir(&luau_dir.join("Ast/src"));
    rerun_dir(&luau_dir.join("Compiler/include"));
    rerun_dir(&luau_dir.join("Compiler/src"));
    rerun_dir(&luau_dir.join("VM/include"));
    rerun_dir(&luau_dir.join("VM/src"));

    if let Some(rev) = luau_git_rev(&luau_dir) {
        println!("cargo:rustc-env=LUAU_GIT_REV={rev}");
    }

    compile_common(&luau_dir);

    compile_bytecode(&luau_dir);
    compile_ast(&luau_dir);
    compile_compiler(&luau_dir);

    compile_vm(&luau_dir);

    println!(
        "cargo:rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );

    println!("cargo:rustc-link-lib=static=harmony_luau_vm");
    println!("cargo:rustc-link-lib=static=harmony_luau_compiler");
    println!("cargo:rustc-link-lib=static=harmony_luau_ast");
    println!("cargo:rustc-link-lib=static=harmony_luau_bytecode");
    println!("cargo:rustc-link-lib=static=harmony_luau_common");

    if let Some(stdlib) = cxx_stdlib() {
        println!("cargo:rustc-link-lib={}", stdlib.to_string_lossy());
    }
}

fn base_build(luau_dir: &Path) -> cc::Build {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings(false)
        .cargo_metadata(false)
        .define("LUA_USE_LONGJMP", "1")
        .define("LUAI_MAXCSTACK", "1000000")
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .include(luau_dir.join("Common/include"));

    if target_is_msvc() {
        build
            .define("LUA_API", "extern \"C\"")
            .define("LUACODE_API", "extern \"C\"");
    } else {
        build
            .define(
                "LUA_API",
                "extern \"C\" __attribute__((visibility(\"default\")))",
            )
            .define(
                "LUACODE_API",
                "extern \"C\" __attribute__((visibility(\"default\")))",
            );
    }

    if !target_is_msvc() {
        build
            .flag_if_supported("-fvisibility=hidden")
            .flag_if_supported("-ffunction-sections")
            .flag_if_supported("-fdata-sections");
    }

    if cfg!(debug_assertions) {
        build.define("LUAU_ENABLE_ASSERT", None);
    }

    build
}

fn compile_common(luau_dir: &Path) {
    let mut build = base_build(luau_dir);
    add_cpp_files(&mut build, &luau_dir.join("Common/src"));
    build.compile("harmony_luau_common");
}

fn compile_bytecode(luau_dir: &Path) {
    let mut build = base_build(luau_dir);
    build.include(luau_dir.join("Bytecode/include"));
    add_cpp_files(&mut build, &luau_dir.join("Bytecode/src"));
    build.compile("harmony_luau_bytecode");
}

fn compile_ast(luau_dir: &Path) {
    let mut build = base_build(luau_dir);
    build.include(luau_dir.join("Ast/include"));
    add_cpp_files(&mut build, &luau_dir.join("Ast/src"));
    build.compile("harmony_luau_ast");
}

fn compile_compiler(luau_dir: &Path) {
    let mut build = base_build(luau_dir);
    build
        .include(luau_dir.join("Ast/include"))
        .include(luau_dir.join("Bytecode/include"))
        .include(luau_dir.join("Compiler/include"));
    add_cpp_files(&mut build, &luau_dir.join("Compiler/src"));
    build.compile("harmony_luau_compiler");
}

fn compile_vm(luau_dir: &Path) {
    let mut build = base_build(luau_dir);
    build.include(luau_dir.join("VM/include"));
    add_cpp_files_except(&mut build, &luau_dir.join("VM/src"), &["lgcdebug.cpp"]);
    build.compile("harmony_luau_vm");
}

fn add_cpp_files(build: &mut cc::Build, dir: &Path) {
    add_cpp_files_except(build, dir, &[]);
}

fn add_cpp_files_except(build: &mut cc::Build, dir: &Path, excluded: &[&str]) {
    let mut files = fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "cpp"))
        .filter(|path| {
            let Some(filename) = path.file_name().and_then(|name| name.to_str()) else {
                return true;
            };
            !excluded.contains(&filename)
        })
        .collect::<Vec<_>>();
    files.sort();

    for file in files {
        build.file(file);
    }
}

fn rerun_dir(dir: &Path) {
    let mut stack = vec![dir.to_path_buf()];

    while let Some(path) = stack.pop() {
        for entry in fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}

fn luau_git_rev(luau_dir: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(luau_dir)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    String::from_utf8(output.stdout)
        .ok()
        .map(|rev| rev.trim().to_owned())
        .filter(|rev| !rev.is_empty())
}

fn cxx_stdlib() -> Option<OsString> {
    if let Some(stdlib) = env::var_os("CXXSTDLIB") {
        return if stdlib.is_empty() {
            None
        } else {
            Some(stdlib)
        };
    }

    let target = env::var("TARGET").expect("TARGET is not set");

    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++".into())
    } else if target.contains("android") {
        Some("c++_shared".into())
    } else {
        Some("stdc++".into())
    }
}

fn target_is_msvc() -> bool {
    env::var("TARGET").is_ok_and(|target| target.contains("msvc"))
}

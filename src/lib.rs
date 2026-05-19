// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

pub mod auxiliary;
pub mod compiler;
pub mod vm;

pub use auxiliary::*;
pub use compiler::*;
pub use vm::*;

pub const LUAU_GIT_REV: Option<&str> = option_env!("LUAU_GIT_REV");

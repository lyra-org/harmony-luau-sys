// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

#include "harmony_luau.h"

#include "ldo.h"

// The machinery behind lua_pcall and lua_cpcall, without the closure lua_cpcall allocates.
LUA_API int harmony_luau_protectedcall(lua_State* L, harmony_luau_ProtectedFunction func, void* ud)
{
    return luaD_pcall(L, func, ud, savestack(L, L->top), 0);
}

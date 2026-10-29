// Copyright (c) 2026 StayBlue
// SPDX-License-Identifier: MIT

#include "harmony_luau.h"

#include "Luau/Common.h"

#include <string.h>

LUA_API int harmony_luau_setfflag(const char* name, int value)
{
    for (Luau::FValue<bool>* flag = Luau::FValue<bool>::list; flag; flag = flag->next)
    {
        if (strcmp(flag->name, name) == 0)
        {
            flag->value = value != 0;
            return 1;
        }
    }
    return 0;
}

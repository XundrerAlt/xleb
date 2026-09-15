// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"

#define MAX_MODULES 16

typedef struct {
    uint32_t start;
    uint32_t end;
    uint32_t size;
    uint32_t cmdline;
    const char *name;
} module_t;

extern module_t modules[MAX_MODULES];
extern uint32_t module_count;

void module_add(uint32_t start, uint32_t end, uint32_t cmdline);
module_t *module_get(uint32_t index);

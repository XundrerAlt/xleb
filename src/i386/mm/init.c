// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"

memory_map_t memory_map;

void mm_init(void) {
    temp_map_init();
    INFO("memory manager: hello world");
}

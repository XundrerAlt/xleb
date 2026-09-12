// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "halt.h"
#include "mm/kheap/mod.h"
#include "mm/mod.h"
#include "mm/ppage/mod.h"
#include "mm/vmm/lmem.h"

void mm_init(void) {
    INFO("memory manager: hello world");
    map_lmem();
    ppage_init();
    kheap_init();
}
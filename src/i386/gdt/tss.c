// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "tss.h"
#include "string.h"

struct tss_entry tss;
extern uint32_t stack_top;

void tss_init(void) {
    memset(&tss, 0, sizeof(tss));
    tss.ss0 = 0x10;
    tss.esp0 = stack_top;
    tss.iomap_base = sizeof(tss);
}

void tss_set_stack(uint32_t esp0) {
    tss.esp0 = esp0;
}

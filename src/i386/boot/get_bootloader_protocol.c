// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "stdint.h"
#include "halt.h"
extern uint32_t boot_magic;
extern void multiboot1_parse_data(void);

void get_bootloader_protocol(void) {
    if (boot_magic == 0x2BADB002) {
        multiboot1_parse_data();
    } else {
        ERROR("Unknown boot protocol");
        halt();
    }
}
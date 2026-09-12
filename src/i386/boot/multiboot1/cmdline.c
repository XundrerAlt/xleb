// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "boot/multiboot1/mod.h"
#include "debug.h"

void process_cmdline(const char *cmdline) {
    if (!cmdline) return;
    INFO("- cmdline: %s", cmdline);
}

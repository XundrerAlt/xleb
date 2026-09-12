// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "boot/multiboot1/info.h"

void process_cmdline(const char *cmdline);
void process_mmap(multiboot_info_t *mbi);
void process_modules(multiboot_info_t *mbi);
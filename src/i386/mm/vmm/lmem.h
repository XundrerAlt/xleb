// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#define LMEM_MAX_SIZE (32 * 1024 * 1024)
#define LMEM_START_PADDR 0x400000
#define LMEM_PDE_SIZE (4 * 1024 * 1024)

void map_lmem(void);
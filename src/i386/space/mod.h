// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once

#include "stdint.h"
#include "vfs/mod.h"
#include "mm/mod.h"

typedef struct space {
    uint32_t id;
    uint32_t pd_addr;
} space_t;

space_t *space_create(void);
void space_destroy(space_t *space);

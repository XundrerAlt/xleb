// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define MAX_MEMORY_REGIONS 64

typedef enum {
    MEMORY_AVAILABLE = 1,
    MEMORY_RESERVED = 2,
    MEMORY_ACPI_RECLAIMABLE = 3,
    MEMORY_ACPI_NVS = 4,
    MEMORY_BADRAM = 5
} memory_type_t;

typedef struct {
    uint32_t start;
    uint32_t end;
    uint32_t size;
    memory_type_t type;
} memory_region_t;

typedef struct {
    memory_region_t regions[MAX_MEMORY_REGIONS];
    uint32_t region_count;
    uint32_t total_memory;
    uint32_t available_memory;
} memory_map_t;

extern memory_map_t memory_map;

void mm_init(void);
uint32_t untyped_alloc(uint32_t size);

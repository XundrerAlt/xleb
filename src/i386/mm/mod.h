// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define MAX_MEMORY_REGIONS 64
#define PAGE_PRESENT 0x001
#define PAGE_WRITE 0x002
#define PAGE_USER 0x004
#define PAGE_SIZE 4096
#define TEMP_WINDOW 0xE0000000

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

typedef enum {
    OBJ_NONE = 0,
    OBJ_FRAME,
} obj_type_t;

extern memory_map_t memory_map;

void mm_init(void);
uint32_t untyped_alloc(uint32_t size);
void *temp_map(uint32_t phys);
void temp_unmap(void);
void temp_map_init(void);
uint32_t retype(obj_type_t type, uint32_t size);

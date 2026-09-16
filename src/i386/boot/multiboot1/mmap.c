// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "boot/multiboot1/info.h"
#include "boot/multiboot1/mod.h"
#include "debug.h"
#include "halt.h"
#include "mm/mod.h"
#include "mm/virtconv.h"

extern uint32_t _kernel_start;
extern uint32_t _kernel_end;

void process_mmap(multiboot_info_t *mbi) {
    if (!(mbi->flags & MULTIBOOT_INFO_MEM_MAP)) return;
    INFO("- memory map (%u bytes):", mbi->mmap_length);
    multiboot_memory_map_t *mmap = (multiboot_memory_map_t*)PHYS_TO_VIRT(mbi->mmap_addr);
    uint32_t mmap_end = (uint32_t)mmap + mbi->mmap_length;
    uint32_t kend_phys = VIRT_TO_PHYS((uint32_t)&_kernel_end);
    while ((uint32_t)mmap < mmap_end && memory_map.region_count < MAX_MEMORY_REGIONS) {
        uint64_t a = mmap->addr;
        uint64_t b = mmap->addr + mmap->len;
        if (a >= 0x100000000ULL) {
            mmap = (multiboot_memory_map_t*)((uint32_t)mmap + mmap->size + 4);
            continue;
        }
        if (b > 0x100000000ULL) b = 0x100000000ULL;
        uint32_t start = (uint32_t)a;
        uint32_t end   = (uint32_t)(b - 1);
        uint32_t size  = (uint32_t)(b - a);
        if (size == 0) {
            mmap = (multiboot_memory_map_t*)((uint32_t)mmap + mmap->size + 4);
            continue;
        }
        memory_region_t *region = &memory_map.regions[memory_map.region_count];
        region->start = start;
        region->end   = end;
        region->size  = size;
        const char *type_str = "unknown";
        switch (mmap->type) {
            case MULTIBOOT_MEMORY_AVAILABLE:
                if (end > kend_phys) {
                    if (start < kend_phys) start = kend_phys;
                    region->start = start;
                    region->size  = end - start + 1;
                    region->type  = MEMORY_AVAILABLE;
                    memory_map.available_memory += region->size;
                    type_str = "available";
                } else {
                    region->type = MEMORY_RESERVED;
                    type_str = "reserved";
                }
                break;
            case MULTIBOOT_MEMORY_RESERVED:
                region->type = MEMORY_RESERVED;
                type_str = "reserved";
                break;
            case MULTIBOOT_MEMORY_ACPI_RECLAIMABLE:
                region->type = MEMORY_ACPI_RECLAIMABLE;
                type_str = "ACPI reclaim";
                break;
            case MULTIBOOT_MEMORY_NVS:
                region->type = MEMORY_ACPI_NVS;
                type_str = "ACPI NVS";
                break;
            case MULTIBOOT_MEMORY_BADRAM:
                region->type = MEMORY_BADRAM;
                type_str = "bad RAM";
                break;
            default:
                region->type = MEMORY_RESERVED;
                type_str = "reserved";
                break;
        }
        INFO("  - [0x%x - 0x%x] %s (%u KB)",
             region->start, region->end, type_str, region->size / 1024);
        memory_map.region_count++;
        memory_map.total_memory += region->size;

        mmap = (multiboot_memory_map_t*)((uint32_t)mmap + mmap->size + 4);
    }

    if (memory_map.region_count == 0) {
        ERROR("memory_map is empty!");
        halt();
    }
    INFO("- total available memory: %d MB", memory_map.available_memory / (1024 * 1024));
}

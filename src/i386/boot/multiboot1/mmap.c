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
    multiboot_memory_map_t *mmap = (multiboot_memory_map_t *)PHYS_TO_VIRT(mbi->mmap_addr);
    uint32_t mmap_end = PHYS_TO_VIRT(mbi->mmap_addr) + mbi->mmap_length;
    while ((uint32_t)mmap < mmap_end && memory_map.region_count < MAX_MEMORY_REGIONS) {
        memory_region_t *region = &memory_map.regions[memory_map.region_count];
        region->start = (uint32_t)mmap->addr;
        region->end = (uint32_t)(mmap->addr + mmap->len - 1);
        region->size = (uint32_t)mmap->len;
        const char *type_str = "unknown";
        switch (mmap->type) {
            case MULTIBOOT_MEMORY_AVAILABLE:
                if (region->end > VIRT_TO_PHYS((uint32_t)&_kernel_end)) {
                    region->start = (region->start > VIRT_TO_PHYS((uint32_t)&_kernel_end)) ? region->start : VIRT_TO_PHYS((uint32_t)&_kernel_end);
                    region->size = region->end - region->start;
                    region->type = MEMORY_AVAILABLE;
                    memory_map.available_memory += region->size;
                    type_str = "available";
                } else {
                    region->type = MEMORY_RESERVED;
                    type_str = "reserved";
                }
                break;
            case MULTIBOOT_MEMORY_RESERVED:
                type_str = "reserved";
                region->type = MEMORY_RESERVED;
                break;
            case MULTIBOOT_MEMORY_ACPI_RECLAIMABLE:
                type_str = "ACPI reclaim";
                region->type = MEMORY_ACPI_RECLAIMABLE;
                break;
            case MULTIBOOT_MEMORY_NVS:
                type_str = "ACPI NVS";
                region->type = MEMORY_ACPI_NVS;
                break;
            case MULTIBOOT_MEMORY_BADRAM:
                type_str = "bad RAM";
                region->type = MEMORY_BADRAM;
                break;
            default:
                type_str = "reserved";
                region->type = MEMORY_RESERVED;
                break;
        }
        INFO("  - [0x%x - 0x%x] %s (%u KB)",
            region->start, region->end, type_str, region->size / 1024);
        memory_map.region_count++;
        memory_map.total_memory += region->size;
        mmap = (multiboot_memory_map_t *)((uint32_t)mmap + mmap->size + sizeof(mmap->size));
    }
    if (memory_map.region_count == 0) {
        ERROR("memory_map is empty!");
        halt();
    }
    INFO("- total available memory: %d MB", memory_map.available_memory / (1024 * 1024));
}

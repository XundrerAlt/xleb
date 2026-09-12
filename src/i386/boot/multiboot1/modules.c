// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "boot/multiboot1/info.h"
#include "boot/multiboot1/mod.h"
#include "mm/virtconv.h"
#include "debug.h"
#include "mm/mod.h"
#include "string.h"

#define MMAP_ENTRY_SIZE 20

static int module_in_region(uint64_t mod_start, uint64_t mod_end, 
                             uint64_t region_start, uint64_t region_end) {
    return mod_start >= region_start && mod_end <= region_end;
}
static void print_module_info(uint32_t index, multiboot_module_t *mod) {
    uint32_t start = mod->mod_start;
    uint32_t end = mod->mod_end;
    INFO("  - module %u:", index);
    INFO("    - address: 0x%x - 0x%x", start, end);
    INFO("    - size: %u bytes", end - start);
    if (mod->cmdline) {
        INFO("    - cmdline: %s", (char*)PHYS_TO_VIRT(mod->cmdline));
    } else {
        INFO("    - cmdline: (none)");
    }
}
static void add_mmap_entry(uint64_t addr, uint64_t len, uint32_t type,
                           uint32_t *mmap_end, uint32_t *mmap_length) {
    multiboot_memory_map_t *new_entry = (multiboot_memory_map_t *)*mmap_end;
    new_entry->size = MMAP_ENTRY_SIZE;
    new_entry->addr = addr;
    new_entry->len = len;
    new_entry->type = type;
    *mmap_end += MMAP_ENTRY_SIZE + 4;
    *mmap_length += MMAP_ENTRY_SIZE + 4;
}
static void remove_mmap_entry(multiboot_memory_map_t *entry, 
                               uint32_t *mmap_end, uint32_t *mmap_length) {
    uint32_t entry_size = entry->size + 4;
    uint32_t remaining = *mmap_end - ((uint32_t)entry + entry_size);
    if (remaining > 0) {
        memmove(entry, (uint8_t*)entry + entry_size, remaining);
    }
    *mmap_end -= entry_size;
    *mmap_length -= entry_size;
}
static void process_single_module(multiboot_memory_map_t *mmap, uint32_t *mmap_end, 
                                   uint32_t *mmap_length, multiboot_module_t *mod, 
                                   uint32_t mod_index) {
    uint64_t mod_start = mod->mod_start;
    uint64_t mod_end = mod->mod_end;
    
    print_module_info(mod_index, mod);
    
    multiboot_memory_map_t *entry = mmap;
    
    while ((uint32_t)entry < *mmap_end) {
        if (entry->type == MULTIBOOT_MEMORY_AVAILABLE &&
            module_in_region(mod_start, mod_end, entry->addr, entry->addr + entry->len)) {
            uint32_t region_start = entry->addr;
            uint32_t region_end = entry->addr + entry->len;
            uint32_t entry_size = entry->size + 4;
            uint32_t remaining = *mmap_end - ((uint32_t)entry + entry_size);
            if (remaining > 0) {
                memmove(entry, (uint8_t*)entry + entry_size, remaining);
            }
            *mmap_end -= entry_size;
            *mmap_length -= entry_size;
            if (mod_start > region_start) {
                multiboot_memory_map_t *before = (multiboot_memory_map_t *)*mmap_end;
                before->size = 20;
                before->addr = region_start;
                before->len = mod_start - region_start;
                before->type = MULTIBOOT_MEMORY_AVAILABLE;
                *mmap_end += 24;
                *mmap_length += 24;
            }
            multiboot_memory_map_t *mod_entry = (multiboot_memory_map_t *)*mmap_end;
            mod_entry->size = 20;
            mod_entry->addr = mod_start;
            mod_entry->len = mod_end - mod_start;
            mod_entry->type = MULTIBOOT_MEMORY_RESERVED;
            *mmap_end += 24;
            *mmap_length += 24;
            if (mod_end < region_end) {
                multiboot_memory_map_t *after = (multiboot_memory_map_t *)*mmap_end;
                after->size = 20;
                after->addr = mod_end;
                after->len = region_end - mod_end;
                after->type = MULTIBOOT_MEMORY_AVAILABLE;
                *mmap_end += 24;
                *mmap_length += 24;
            }
            break;
        }
        entry = (multiboot_memory_map_t *)((uint32_t)entry + entry->size + 4);
    }
}

void process_modules(multiboot_info_t *mbi) {
    if (!(mbi->flags & MULTIBOOT_INFO_MODS)) return;
    if (!(mbi->flags & MULTIBOOT_INFO_MEM_MAP)) return;
    INFO("- modules (%u):", mbi->mods_count);
    uint32_t base = PHYS_TO_VIRT(mbi->mmap_addr);
    uint32_t mmap_end = base + mbi->mmap_length;
    multiboot_memory_map_t *mmap = (multiboot_memory_map_t *)base;
    multiboot_module_t *mods = (multiboot_module_t *)PHYS_TO_VIRT(mbi->mods_addr);
    for (uint32_t i = 0; i < mbi->mods_count; i++) {
        process_single_module(mmap, &mmap_end, &mbi->mmap_length, &mods[i], i);
    }
}
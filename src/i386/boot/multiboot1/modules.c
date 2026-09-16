// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "boot/module.h"
#include "boot/multiboot1/info.h"
#include "boot/multiboot1/mod.h"
#include "mm/virtconv.h"
#include "debug.h"
#include "mm/mod.h"
#include "string.h"

#define MAX_MMAP_ENTRIES  64
#define MMAP_ENTRY_SIZE   24

module_t modules[MAX_MODULES];
uint32_t module_count = 0;

static multiboot_memory_map_t mmap_buf[MAX_MMAP_ENTRIES];

static int module_in_region(uint32_t mod_start, uint32_t mod_end,
                             uint32_t region_start, uint32_t region_end) {
    return mod_start >= region_start && mod_end <= region_end;
}

static void print_module_info(uint32_t index, multiboot_module_t *mod) {
    uint32_t start = mod->mod_start;
    uint32_t end   = mod->mod_end;
    INFO("  - module %u:", index);
    INFO("    - address: 0x%x - 0x%x", start, end);
    INFO("    - size: %u bytes", end - start);
    if (mod->cmdline) {
        INFO("    - cmdline: %s", (char*)PHYS_TO_VIRT(mod->cmdline));
    } else {
        INFO("    - cmdline: (none)");
    }
}

static int process_single_module(multiboot_memory_map_t *mmap,
                                  uint32_t *mmap_end_ptr,
                                  uint32_t *mmap_length_ptr,
                                  multiboot_module_t *mod,
                                  uint32_t mod_index)
{
    uint32_t mod_start = mod->mod_start;
    uint32_t mod_end   = mod->mod_end;

    print_module_info(mod_index, mod);
    module_add(mod_start, mod_end, mod->cmdline);

    uint8_t *p   = (uint8_t*)mmap;
    uint8_t *end = (uint8_t*)*mmap_end_ptr;

    while (p < end) {
        multiboot_memory_map_t *e = (multiboot_memory_map_t*)p;
        uint32_t esize = e->size + 4;

        uint32_t rstart = (uint32_t)e->addr;
        uint32_t rend   = (uint32_t)(e->addr + e->len);

        if (e->type == MULTIBOOT_MEMORY_AVAILABLE &&
            module_in_region(mod_start, mod_end, rstart, rend))
        {
            uint32_t has_before = (mod_start > rstart);
            uint32_t has_after  = (mod_end   < rend);
            uint32_t n_parts    = 1 + has_before + has_after;
            uint32_t new_size   = n_parts * MMAP_ENTRY_SIZE;
            int32_t  delta      = (int32_t)new_size - (int32_t)esize;

            uint8_t *src = p + esize;
            uint32_t src_len = end - src;

            if (delta > 0) {
                for (uint32_t i = src_len; i > 0; i--) {
                    src[i - 1 + delta] = src[i - 1];
                }
            } else if (delta < 0) {
                for (uint32_t i = 0; i < src_len; i++) {
                    src[i + delta] = src[i];
                }
            }

            uint8_t *q = p;
            if (has_before) {
                multiboot_memory_map_t *n = (multiboot_memory_map_t*)q;
                n->size = 20;
                n->addr = rstart;
                n->len  = mod_start - rstart;
                n->type = MULTIBOOT_MEMORY_AVAILABLE;
                q += MMAP_ENTRY_SIZE;
            }
            {
                multiboot_memory_map_t *n = (multiboot_memory_map_t*)q;
                n->size = 20;
                n->addr = mod_start;
                n->len  = mod_end - mod_start;
                n->type = MULTIBOOT_MEMORY_RESERVED;
                q += MMAP_ENTRY_SIZE;
            }
            if (has_after) {
                multiboot_memory_map_t *n = (multiboot_memory_map_t*)q;
                n->size = 20;
                n->addr = mod_end;
                n->len  = rend - mod_end;
                n->type = MULTIBOOT_MEMORY_AVAILABLE;
            }

            *mmap_end_ptr    += delta;
            *mmap_length_ptr += delta;
            return 1;
        }
        p += esize;
    }
    return 0;
}

void process_modules(multiboot_info_t *mbi) {
    if (!(mbi->flags & MULTIBOOT_INFO_MODS)) return;
    if (!(mbi->flags & MULTIBOOT_INFO_MEM_MAP)) return;
    INFO("- modules (%u):", mbi->mods_count);
    uint32_t orig_base = PHYS_TO_VIRT(mbi->mmap_addr);
    uint32_t orig_len  = mbi->mmap_length;
    uint32_t n = 0;
    uint8_t *p = (uint8_t*)orig_base;
    uint8_t *end = p + orig_len;
    while (p < end && n < MAX_MMAP_ENTRIES) {
        multiboot_memory_map_t *e = (multiboot_memory_map_t*)p;
        uint64_t a = e->addr;
        uint64_t b = e->addr + e->len;
        if (a >= 0x100000000ULL) {
            p += e->size + 4;
            continue;
        }
        if (b > 0x100000000ULL) b = 0x100000000ULL;
        memcpy(&mmap_buf[n], e, e->size + 4);
        mmap_buf[n].addr = a;
        mmap_buf[n].len  = b - a;
        p += e->size + 4;
        n++;
    }
    uint32_t new_len = n * MMAP_ENTRY_SIZE;
    uint32_t new_end = (uint32_t)mmap_buf + new_len;
    multiboot_module_t *mods = (multiboot_module_t*)PHYS_TO_VIRT(mbi->mods_addr);
    for (uint32_t i = 0; i < mbi->mods_count; i++) {
        process_single_module(mmap_buf, &new_end, &new_len, &mods[i], i);
    }
    mbi->mmap_addr = VIRT_TO_PHYS((uint32_t)mmap_buf);
    mbi->mmap_length = new_len;
}

// module table
void module_add(uint32_t start, uint32_t end, uint32_t cmdline) {
    if (module_count >= MAX_MODULES) {
        WARN("module_add: too many modules (max %d)", MAX_MODULES);
        return;
    }
    module_t *m = &modules[module_count++];
    m->start   = start;
    m->end     = end;
    m->size    = end - start;
    m->cmdline = cmdline;
    m->name    = 0;
}

module_t *module_get(uint32_t index) {
    if (index >= module_count) return 0;
    return &modules[index];
}

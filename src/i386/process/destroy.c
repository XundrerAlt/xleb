// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "process/mod.h"
#include "vfs/mod.h"
#include "vfs/ofile.h"
#include "mm/mod.h"
#include "vfs/kheap/mod.h"
#include "debug.h"

void process_destroy(process_t *process) {
    if (!process) return;
    for (int i = 0; i < MAX_FDS; i++) {
        if (process->fds[i]) {
            vfs_ofile_unref(process->fds[i]);
            process->fds[i] = 0;
        }
    }
    process->fd_count = 0;
    if (process->ns) {
        vfs_ns_destroy(process->ns);
        process->ns = 0;
    }
    if (process->pd_addr) {
        //destroy_page_directory(process->pd_addr); - TODO
        process->pd_addr = 0;
    }
    kfree(process);
}

// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "space/mod.h"
#include "vfs/mod.h"
#include "vfs/ofile.h"
#include "mm/mod.h"
#include "vfs/kheap/mod.h"
#include "debug.h"

void space_destroy(space_t *space) {
    if (!space) return;
    for (int i = 0; i < MAX_FDS; i++) {
        if (space->fds[i]) {
            vfs_ofile_unref(space->fds[i]);
            space->fds[i] = 0;
        }
    }
    space->fd_count = 0;
    if (space->ns) {
        vfs_ns_destroy(space->ns);
        space->ns = 0;
    }
    if (space->pd_addr) {
        //destroy_page_directory(space->pd_addr); - TODO
        space->pd_addr = 0;
    }
    kfree(space);
}

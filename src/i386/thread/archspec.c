// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "gdt/tss.h"
#include "mod.h"

void archspec_schedule(thread_t *next) {
    if (next->is_user) {
        tss_set_stack((uint32_t)next->kernel_stack_top);
    }
}

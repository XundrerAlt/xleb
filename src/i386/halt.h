// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt

#pragma once

static inline void halt(void) {
    while (1) {
        __asm__ volatile ("hlt");
    }
}
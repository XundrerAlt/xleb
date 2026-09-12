// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "iob.h"

void uart_putc(char c) {
    while (!(inb(0x3F8 + 5) & (1 << 5)));
    if (c == '\n') {
        uart_putc('\r');
    }
    outb(0x3F8, c);
}

void uart_puts(const char *str) {
    while (*str) {
        uart_putc(*str++);
    }
}
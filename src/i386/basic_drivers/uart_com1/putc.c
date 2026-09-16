// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/mod.h"
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

static int uart_write(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size) {
    (void)inode; (void)offset;
    const char *s = buf;
    for (uint32_t i = 0; i < size; i++) uart_putc(s[i]);
    return size;
}

__attribute__((section(".rodata.raw_devices"), used))
const raw_device_t uart_device = {
    .name = "serial0",
    .ops = {
        .read  = 0,
        .write = uart_write,
    },
};

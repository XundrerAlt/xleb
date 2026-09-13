// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/idt/mod.h"
#include "interrupt/init.h"
#include "interrupt/pic/mod.h"

extern uint32_t isr_entry_table[32];
extern uint32_t irq_entry_table[16];
extern void syscall_entry(void);

void interrupt_init(void) {
    for (int i = 0; i < 32; i++) {
        idt_set_gate(i, isr_entry_table[i], 0x08, 0x8E);
    }
    INFO("ISR: hello world");
    for (int i = 0; i < 16; i++) {
        idt_set_gate(32 + i, irq_entry_table[i], 0x08, 0x8E);
    }
    idt_set_gate(0x80, (uint32_t)syscall_entry, 0x08, 0x8E); // for ring 0 yet
    pic_remap(32, 40);
    pic_mask_all();
    INFO("IRQ: hello world");
    ptr.limit = sizeof(idt) - 1;
    ptr.base = (uint32_t)idt;
    __asm__ volatile("lidt %0" : : "m"(ptr));
    __asm__ volatile("sti");
    INFO("IDT: hello world");
}

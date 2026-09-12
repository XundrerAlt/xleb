// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "interrupt/idt/mod.h"

struct idt_entry idt[IDT_SIZE];
struct idt_ptr ptr;

void idt_set_gate(uint8_t num, uint32_t base, uint16_t selector, uint8_t type_attr) {
    idt[num].base_low = base & 0xFFFF;
    idt[num].base_high = (base >> 16) & 0xFFFF;
    idt[num].selector = selector;
    idt[num].zero = 0;
    idt[num].type_attr = type_attr;
}
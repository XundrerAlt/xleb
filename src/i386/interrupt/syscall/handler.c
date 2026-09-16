// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/idt/mod.h"
#include "mod.h"
#include "space/mod.h"
#include "scheduler/mod.h"
#include "vfs/mod.h"

static int check_user_ptr(uint32_t ptr, uint32_t size) {
    if (ptr >= 0xC0000000) return -1;
    if (ptr + size >= 0xC0000000) return -1;
    return 0;
}

static int sys_open(const char *path, int flags) {
    if (check_user_ptr((uint32_t)path, 1) != 0) return -1;
    space_t *space = current_thread->space;
    if (!space) return -1;
    return vfs_open(space, path, flags);
}

static int sys_close(int fd) {
    space_t *space = current_thread->space;
    if (!space) return -1;
    return vfs_close(space, fd);
}

static int sys_read(int fd, void *buf, uint32_t size) {
    space_t *space = current_thread->space;
    if (!space) return -1;
    if (check_user_ptr((uint32_t)buf, size) != 0) return -1;
    return vfs_read(space, fd, buf, size);
}

static int sys_write(int fd, const void *buf, uint32_t size) {
    space_t *space = current_thread->space;
    if (!space) return -1;
    if (check_user_ptr((uint32_t)buf, size) != 0) return -1;
    return vfs_write(space, fd, buf, size);
}

void syscall_handler(regs_t *regs) {
    uint32_t num = regs->eax;
    uint32_t arg0 = regs->ebx;
    uint32_t arg1 = regs->ecx;
    uint32_t arg2 = regs->edx;
    switch (num) {
        case SYS_OPEN:
            regs->eax = sys_open((const char*)arg0, (int)arg1);
            break;
        case SYS_CLOSE:
            regs->eax = sys_close((int)arg0);
            break;
        case SYS_READ:
            regs->eax = sys_read((int)arg0, (void*)arg1, arg2);
            break;
        case SYS_WRITE:
            regs->eax = sys_write((int)arg0, (const void*)arg1, arg2);
            break;
        default:
            WARN("UNKNOWN SYSCALL (%d)", num);
            regs->eax = -1;
            break;
    }
}

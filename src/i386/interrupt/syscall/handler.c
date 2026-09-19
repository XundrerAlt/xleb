// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "interrupt/idt/mod.h"
#include "mod.h"
#include "process/mod.h"
#include "scheduler/mod.h"
#include "thread/mod.h"
#include "vfs/mod.h"

static int check_user_ptr(uint32_t ptr, uint32_t size) {
    if (ptr >= 0xC0000000) return -1;
    if (ptr + size < ptr) return -1;
    if (ptr + size >= 0xC0000000) return -1;
    return 0;
}

static void sys_exit(int code) {
    (void)code;
    thread_destroy(current_thread);
}

static int sys_open(const char *path, int flags) {
    if (check_user_ptr((uint32_t)path, 1) != 0) return -1;
    process_t *process = current_thread->process;
    if (!process) return -1;
    return vfs_open(process, path, flags);
}

static int sys_close(int fd) {
    process_t *process = current_thread->process;
    if (!process) return -1;
    return vfs_close(process, fd);
}

static int sys_read(int fd, void *buf, uint32_t size) {
    process_t *process = current_thread->process;
    if (!process) return -1;
    if (check_user_ptr((uint32_t)buf, size) != 0) return -1;
    return vfs_read(process, fd, buf, size);
}

static int sys_write(int fd, const void *buf, uint32_t size) {
    process_t *process = current_thread->process;
    if (!process) return -1;
    if (check_user_ptr((uint32_t)buf, size) != 0) return -1;
    return vfs_write(process, fd, buf, size);
}

static int sys_mount(const char *path, const char *fs_name) {
    process_t *process = current_thread->process;
    if (!process || !process->ns) return -1;

    if (!path) return -1;
    if (check_user_ptr((uint32_t)path, 4) != 0) return -1;
    if (check_user_ptr((uint32_t)fs_name, 4) != 0) return -1;

    vfs_fs_t *fs = vfs_find_fs(fs_name);
    if (!fs) {
        DEBUG("sys_ns_mount: fs '%s' not found", fs_name);
        return -1;
    }

    vfs_inode_t *root = fs->get_root ? fs->get_root() : 0;
    if (!root) {
        DEBUG("sys_ns_mount: fs '%s' get_root failed", fs_name);
        return -1;
    }

    int r = vfs_ns_mount(process->ns, path, root, 0);
    if (r < 0) {
        DEBUG("sys_ns_mount: mount '%s' at '%s' failed", fs_name, path);
        return -1;
    }

    DEBUG("sys_ns_mount: mounted '%s' at '%s'", fs_name, path);
    return 0;
}

void syscall_handler(regs_t *regs) {
    uint32_t num = regs->eax;
    uint32_t arg0 = regs->ebx;
    uint32_t arg1 = regs->ecx;
    uint32_t arg2 = regs->edx;
    switch (num) {
        case SYS_EXIT:
            sys_exit((int)arg0);
            regs->eax = 0;
            break;
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
        case SYS_MOUNT:
            regs->eax = sys_mount((const char*)arg0, (const char*)arg1);
            break;
        default:
            WARN("UNKNOWN SYSCALL (%d)", num);
            regs->eax = -1;
            break;
    }
}

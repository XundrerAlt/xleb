// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "kheap/mod.h"
#include "string.h"

static int stdout_write(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size) {
    (void)inode;
    (void)offset;
    const char *s = (const char*)buf;
    for (uint32_t i = 0; i < size; i++) {
        uart_putc(s[i]);
    }
    return size;
}

struct chardev_ops stdout_ops = {
    .read = 0,
    .write = stdout_write,
};

vfs_namespace_t *vfs_ns_create(void) {
    vfs_namespace_t *ns = kmalloc(sizeof(vfs_namespace_t));
    if (!ns) return 0;
    ns->mounts = 0;
    ns->refcount = 1;
    return ns;
}

void vfs_ns_destroy(vfs_namespace_t *ns) {
    if (!ns) return;
    vfs_mount_t *m = ns->mounts;
    while (m) {
        vfs_mount_t *next = m->next;
        vfs_inode_unref(m->root);
        kfree(m->path);
        kfree(m);
        m = next;
    }
    kfree(ns);
}

void vfs_ns_ref(vfs_namespace_t *ns) {
    if (ns) ns->refcount++;
}

void vfs_ns_unref(vfs_namespace_t *ns) {
    if (!ns) return;
    if (ns->refcount > 0) ns->refcount--;
    if (ns->refcount == 0) {
        vfs_ns_destroy(ns);
    }
}

int vfs_ns_mount(vfs_namespace_t *ns, const char *path, vfs_inode_t *root, uint32_t rights) {
    if (!ns || !path || !root) return -1;
    vfs_mount_t *m = kmalloc(sizeof(vfs_mount_t));
    if (!m) return -1;

    uint32_t len = strlen(path);
    m->path = kmalloc(len + 1);
    if (!m->path) { kfree(m); return -1; }
    strcpy(m->path, path);

    m->root = root;
    vfs_inode_ref(root);
    m->rights = rights;
    m->next = ns->mounts;
    ns->mounts = m;

    return 0;
}

int vfs_ns_unmount(vfs_namespace_t *ns, const char *path) {
    if (!ns || !path) return -1;

    vfs_mount_t **p = &ns->mounts;
    while (*p) {
        if (strcmp((*p)->path, path) == 0) {
            vfs_mount_t *m = *p;
            *p = m->next;

            vfs_inode_unref(m->root);
            kfree(m->path);
            kfree(m);
            return 0;
        }
        p = &(*p)->next;
    }
    return -1;
}

vfs_inode_t *vfs_lookup_ns(vfs_namespace_t *ns, const char *path) {
    if (!ns || !path) return 0;
    vfs_mount_t *best = 0;
    uint32_t best_len = 0;
    uint32_t path_len = strlen(path);

    vfs_mount_t *m = ns->mounts;
    while (m) {
        uint32_t mlen = strlen(m->path);
        if (mlen <= path_len && strncmp(path, m->path, mlen) == 0) {
            int boundary_ok = (mlen == 0) ||
            (path[mlen] == 0) ||
            (path[mlen] == '/') ||
            (m->path[mlen - 1] == '/');

            if (boundary_ok && mlen >= best_len) {
                best = m;
                best_len = mlen;
            }
        }
        m = m->next;
    }

    if (!best) return 0;
    const char *rel = path + best_len;
    while (*rel == '/') rel++;
    if (*rel == 0) return best->root;

    return vfs_lookup(best->root, rel);
}

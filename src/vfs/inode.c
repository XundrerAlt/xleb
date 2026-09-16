// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "debug.h"
#include "mod.h"
#include "kheap/mod.h"
#include "string.h"

static int next_inode_id = 0;

vfs_inode_t *vfs_inode_alloc(vfs_type_t type, const char *name) {
    vfs_inode_t *inode = kmalloc(sizeof(vfs_inode_t));
    if (!inode) {
        ERROR("inode alloc: out of memory");
        return 0;
    }
    memset(inode, 0, sizeof(vfs_inode_t));
    inode->id = next_inode_id++;
    inode->type = type;
    inode->refcount = 1;
    if (name) {
        strncpy(inode->name, name, VFS_NAME_MAX - 1);
        inode->name[VFS_NAME_MAX - 1] = 0;
    }
    return inode;
}

void vfs_inode_free(vfs_inode_t *inode) {
    if (!inode) return;
    if (inode->data) kfree(inode->data);
    kfree(inode);
}

void vfs_inode_ref(vfs_inode_t *inode) {
    if (!inode) return;
    inode->refcount++;
}

void vfs_inode_unref(vfs_inode_t *inode) {
    if (!inode) return;
    if (inode->refcount == 0) {
        WARN("inode unref: refcount already 0 (id=%u)", inode->id);
        return;
    }
    inode->refcount--;
    if (inode->refcount == 0) {
        vfs_inode_free(inode);
    }
}

int vfs_inode_add_child(vfs_inode_t *dir, vfs_inode_t *child) {
    if (!dir || !child) return -1;
    if (dir->type != VFS_DIR) {
        ERROR("inode add child: '%s' is not a directory", dir->name);
        return -1;
    }

    child->parent = dir;
    child->next = dir->children;
    dir->children = child;
    vfs_inode_ref(child);
    return 0;
}

vfs_inode_t *vfs_inode_find_child(vfs_inode_t *dir, const char *name) {
    if (!dir || !name) return 0;
    if (dir->type != VFS_DIR) return 0;

    vfs_inode_t *child = dir->children;
    while (child) {
        if (strcmp(child->name, name) == 0) {
            return child;
        }
        child = child->next;
    }
    return 0;
}

vfs_inode_t *vfs_lookup(vfs_inode_t *root, const char *path) {
    if (!root || !path) return 0;
    if (*path == 0) return root;

    vfs_inode_t *cur = root;
    while (*path) {
        while (*path == '/') path++;
        if (*path == 0) break;

        char name[VFS_NAME_MAX];
        int i = 0;
        while (*path && *path != '/' && i < VFS_NAME_MAX - 1) {
            name[i++] = *path++;
        }
        name[i] = 0;

        cur = vfs_inode_find_child(cur, name);
        if (!cur) return 0;
    }
    return cur;
}

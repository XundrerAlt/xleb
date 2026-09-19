// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "stdint.h"
#define VFS_NAME_MAX 256

typedef enum {
    VFS_FILE,
    VFS_DIR,
    VFS_CHARDEV,
} vfs_type_t;

typedef struct vfs_inode vfs_inode_t;
struct chardev_ops;
typedef struct process process_t;
typedef struct vfs_fs vfs_fs_t;

typedef struct vfs_inode {
    uint32_t id;
    vfs_type_t type;
    uint32_t size;
    uint32_t refcount;
    uint8_t *data;
    void *fs_data;
    char name[VFS_NAME_MAX];
    const struct chardev_ops *ops;
    struct vfs_inode *parent;
    struct vfs_inode *children;
    struct vfs_inode *next;
} vfs_inode_t;

typedef struct vfs_fs {
    const char *name;
    struct vfs_inode *(*get_root)(void);
    struct vfs_fs *next;
} vfs_fs_t;

struct chardev_ops {
    int (*read)(vfs_inode_t *inode, uint32_t offset, void *buf, uint32_t size);
    int (*write)(vfs_inode_t *inode, uint32_t offset, const void *buf, uint32_t size);
};

typedef struct vfs_mount {
    char *path;
    struct vfs_inode *root;
    uint32_t rights;
    struct vfs_mount *next;
} vfs_mount_t;

typedef struct vfs_namespace {
    struct vfs_mount *mounts;
    uint32_t refcount;
} vfs_namespace_t;

vfs_inode_t *vfs_inode_alloc(vfs_type_t type, const char *name);
void vfs_inode_free(vfs_inode_t *inode);

vfs_namespace_t *vfs_ns_create(void);
void vfs_ns_destroy(vfs_namespace_t *ns);
int vfs_ns_mount(vfs_namespace_t *ns, const char *path, vfs_inode_t *root, uint32_t rights);
int vfs_ns_unmount(vfs_namespace_t *ns, const char *path);
vfs_inode_t *vfs_lookup_ns(vfs_namespace_t *ns, const char *path);

void vfs_inode_ref(vfs_inode_t *inode);
void vfs_inode_unref(vfs_inode_t *inode);
int vfs_inode_add_child(vfs_inode_t *dir, vfs_inode_t *child);
vfs_inode_t *vfs_inode_find_child(vfs_inode_t *dir, const char *name);
vfs_inode_t *vfs_lookup(vfs_inode_t *root, const char *path);
int vfs_read(process_t *process, int fd, void *buf, uint32_t size);
int vfs_write(process_t *process, int fd, const void *buf, uint32_t size);

void vfs_register_fs(vfs_fs_t *fs);
vfs_fs_t *vfs_find_fs(const char *name);

void vfs_init(void);

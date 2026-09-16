// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#pragma once
#include "vfs/mod.h"

struct raw_device;

typedef struct raw_file {
    const char *name;
    struct chardev_ops ops;
} raw_file_t;

typedef struct raw_device {
    const char *name;
    const raw_file_t *files;
    uint32_t file_count;
} raw_device_t;

void add_raw_devices(vfs_namespace_t *ns);

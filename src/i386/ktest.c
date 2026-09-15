// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "debug.h"
#include "halt.h"
#include "interrupt/init.h"
#include "interrupt/syscall/mod.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"
#include "scheduler/mod.h"
#include "test.h"

uint32_t tests_success = 0;
uint32_t tests_failed = 0;
uint8_t must_caught_exception = 0;
static uint8_t th_flag = 0;
extern uint32_t kernel_page_directory[1024];
extern void run_init(void);

void test_th(void) {
    INFO("test thread: hello world, i'm working");
    th_flag |= (1 << 0);
    while (1) {
        halt();
    }
}

void test2_th(void) {
    INFO("test2 thread: hello world, i'm working");
    th_flag |= (1 << 1);
    while (1) {
        halt();
    }
}

void test3_th(void) {
    int ret;
    __asm__ volatile(
        "int $0x80"
        : "=a" (ret)
        : "a" (SYS_PRINT)
    );
    while (1) {

    }
}

void ktest(void) {
    INFO("Start testing");
    INFO("Test 1: Memory manager");
    DEBUG("Retype 4 bytes of untyped memory");
    uint32_t addr = retype(OBJ_FRAME, 2);
    DEBUG("Address: 0x%x", addr);
    if (addr) {
        uint32_t *virt_addr = temp_map(addr);
        if (!virt_addr) {
            ERROR("temp_map failed");
            test_failed();
        } else {
            *virt_addr = 0xDEADBEEF;
            DEBUG("Written: 0x%x", *virt_addr);
            if (*virt_addr == 0xDEADBEEF) {
                DEBUG("Read/Write works");
                test_success();
            } else {
                ERROR("Read/Write failed");
                test_failed();
            }
            temp_unmap(virt_addr);
        }
    } else {
        test_failed();
    }
    INFO("Test 2: Timer");
    uint32_t ftm = timer_get_ticks();
    INFO("First time measurement: %d, wait 10000000 CPU ticks", ftm);
    for (volatile uint32_t i = 0; i < 10000000; i++) {}
    uint32_t stm = timer_get_ticks();
    INFO("Second time measurement: %d", stm);
    if (!ftm && !stm) {
        WARN("Timer ticks is still 0, timer is not working");
        test_failed();
    } else {
        INFO("Timer is working");
        test_success();
    }
    INFO("Test 3: Scheduler");
    INFO("Add, start test threads and wait 10 timer ticks");
    thread_t *th1 = thread_create(test_th, 0, 0);
    scheduler_add_thread(th1);
    thread_t *th2 = thread_create(test2_th, 0, 0);
    scheduler_add_thread(th2);
    uint32_t tstart = timer_get_ticks();
    while (timer_get_ticks() < tstart + 10) {
        __asm__ volatile ("hlt");
    }
    if (th_flag == 3) {
        INFO("Both threads are running");
        test_success();
    } else if (th_flag == 2 || th_flag == 1) {
        WARN("Only one thread is running");
        test_failed();
    } else if (!th_flag) {
        WARN("Threads isn't started");
        test_failed();
    } else {
        WARN("Invalid th_flag");
        test_failed();
    }
    INFO("End testing");
    if (tests_failed) {
        WARN("%d success; %d failed", tests_success, tests_failed);
    } else {
        INFO("%d success; 0 failed", tests_success);
    }
    run_init();
    halt();
}

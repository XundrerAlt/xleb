// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "debug.h"
#include "halt.h"
#include "interrupt/init.h"
#include "mm/mod.h"
#include "mm/virtconv.h"
#include "stdint.h"
#include "task/scheduler/mod.h"
#include "test.h"

uint32_t tests_success = 0;
uint32_t tests_failed = 0;
uint8_t must_caught_exception = 0;
static uint8_t th_flag = 0;
extern uint32_t kernel_page_directory[1024];

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

extern void get_bootloader_protocol(void);
void ktest(void) {
    INFO("Start testing");
    INFO("Test 1: Capability");
    DEBUG("Allocate two pages of untyped memory");
    uint32_t addr1 = untyped_alloc(4096);
    DEBUG("Address 1: 0x%x", addr1);
    uint32_t addr2 = untyped_alloc(4096);
    DEBUG("Address 2: 0x%x", addr2);
    if (addr1 && addr2 && (addr2 == (addr1 + 4096))) {
        test_success();
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
    /*
    INFO("Test 3: Scheduler");
    scheduler_init();
    INFO("Add, start test threads and wait 10 timer ticks");
    scheduler_add_thread(test_th);
    scheduler_add_thread(test2_th);
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
    */
    INFO("End testing");
    if (tests_failed) {
        WARN("%d success; %d failed", tests_success, tests_failed);
    } else {
        INFO("%d success; 0 failed", tests_success);
    }
    halt();
}

// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 XundrerAlt
#include "basic_drivers/timer/mod.h"
#include "debug.h"
#include "halt.h"
#include "interrupt/init.h"
#include "mm/vmm/mod.h"
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

void test_th() {
    INFO("test thread: hello world, i'm working");
    th_flag |= (1 << 0);
    while (1) {
        halt();
    }
}

void test2_th() {
    INFO("test2 thread: hello world, i'm working");
    th_flag |= (1 << 1);
    while (1) {
        halt();
    }
}

extern void get_bootloader_protocol();
void ktest() {
    INFO("Start testing");
    INFO("Test 1: ISR");
    interrupt_init();
    INFO("Divide by zero...");
    volatile uint32_t a = 1;
    a--;
    must_caught_exception = 1;
    volatile uint32_t b = 1 / a;
    must_caught_exception = 0;
    test_success();
    INFO("Not test: Boot protocol setup");
    get_bootloader_protocol();
    INFO("Test 2: Memory Management");
    mm_init();
    INFO("Allocating some page...");
    uint32_t *num = vmalloc(kernel_page_directory, 4, 0);
    INFO("Writing 31 into *num...");
    *num = 31;
    INFO("Reading *num: %d", *num);
    if (*num == 31) {
        test_success();
    } else {
        test_failed();
    }
    INFO("Freeing *num");
    vfree(kernel_page_directory, num, 4);
    INFO("Test 3: Timer");
    timer_init();
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
    INFO("Test 4: Scheduler");
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
    INFO("End testing");
    if (tests_failed) {
        WARN("%d success; %d failed", tests_success, tests_failed);
    } else {
        INFO("%d success; 0 failed", tests_success);
    }
    halt();
}
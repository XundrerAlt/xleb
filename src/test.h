#pragma once
#include "debug.h"
#include "stdint.h"

#ifdef ENABLE_TESTS
extern uint32_t tests_success;
extern uint32_t tests_failed;
extern uint8_t must_caught_exception;
static void test_failed() {
    WARN("Test failed");
    tests_failed++;
}
static void test_success() {
    INFO("Test success");
    tests_success++;
}
#endif

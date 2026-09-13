#pragma once
#include "debug.h"
#include "stdint.h"

#ifdef ENABLE_TESTS
extern uint32_t tests_success;
extern uint32_t tests_failed;
extern uint8_t must_caught_exception;
static inline void test_failed(void) {
    WARN("Test failed");
    tests_failed++;
}
static inline void test_success(void) {
    INFO("Test success");
    tests_success++;
}
#endif

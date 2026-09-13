#pragma once
#include "stdarg.h"
#include "string.h"
#include "basic_drivers/uart_com1/mod.h" // TODO: delete hardcode

static inline void print_num(int num, int is_signed) {
    if (is_signed && num < 0) {
        uart_putc('-');
        num = -num;
    }

    if (num >= 10) {
        print_num(num / 10, 0);
    }
    uart_putc('0' + (num % 10));
}

static inline void print_hex(unsigned int num) {
    static const char hex[] = "0123456789abcdef";

    for (int i = 28; i >= 0; i -= 4) {
        uart_putc(hex[(num >> i) & 0xF]);
    }
}

static inline void debug_print(const char *fmt, ...) {
    va_list args;
    va_start(args, fmt);
    while (*fmt) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
                case 's': {
                    const char *str = va_arg(args, const char*);
                    while (*str) {
                        uart_putc(*str++);
                    }
                    break;
                }
                case 'd': {
                    int num = va_arg(args, int);
                    print_num(num, 1);
                    break;
                }
                case 'u': {
                    int num = va_arg(args, unsigned int);
                    print_num(num, 0);
                    break;
                }
                case 'x': {
                    unsigned int num = va_arg(args, unsigned int);
                    print_hex(num);
                    break;
                }
                case 'c': {
                    char c = (char)va_arg(args, int);
                    uart_putc(c);
                    break;
                }
                case '%': {
                    uart_putc('%');
                    break;
                }
                default:
                    uart_putc('%');
                    uart_putc(*fmt);
                    break;
            }
        } else {
            uart_putc(*fmt);
        }
        fmt++;
    }
    va_end(args);
}

#define __SHORT_FILE__ \
    (__builtin_strstr(__FILE__, "src/") ? \
    __builtin_strstr(__FILE__, "src/") + 4 : \
    __FILE__)

#define DEBUG(fmt, ...) \
debug_print("\033[36m[DEBUG] (%s:%d) \033[0m" fmt "\n", __SHORT_FILE__, __LINE__, ##__VA_ARGS__)

#define INFO(fmt, ...) \
debug_print("\033[32m[INFO ] (%s:%d) \033[0m" fmt "\n", __SHORT_FILE__, __LINE__, ##__VA_ARGS__)

#define WARN(fmt, ...) \
debug_print("\033[33m[WARN ] (%s:%d) \033[0m" fmt "\n", __SHORT_FILE__, __LINE__, ##__VA_ARGS__)

#define ERROR(fmt, ...) \
debug_print("\033[31m[ERROR] (%s:%d) \033[0m" fmt "\n", __SHORT_FILE__, __LINE__, ##__VA_ARGS__)

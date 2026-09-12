# SPDX-License-Identifier: GPL-3.0-or-later
# SPDX-FileCopyrightText: 2026 XundrerAlt
cmake_minimum_required(VERSION 4.1)
set(CMAKE_C_COMPILER_WORKS ON CACHE BOOL "Skip compiler test" FORCE)
set(CMAKE_ASM_COMPILER_WORKS ON CACHE BOOL "Skip compiler test" FORCE)

project(xleb VERSION 0.0.1 LANGUAGES C ASM)
add_executable(kernel)
configure_file(
    ${CMAKE_SOURCE_DIR}/src/config.h.in
    ${CMAKE_BINARY_DIR}/config.h
    @ONLY
)
target_include_directories(kernel PRIVATE
    ${CMAKE_SOURCE_DIR}/src
    ${CMAKE_BINARY_DIR}
)

option(ENABLE_TESTS "Build tests and ktest instead kmain" OFF)
if(ENABLE_TESTS)
    message(STATUS "Tests enabled")
endif()
add_subdirectory(src)

target_link_options(kernel PRIVATE
    -nostdlib
    -static
    -T ${CMAKE_SOURCE_DIR}/src/${ARCH}/linker.ld
)

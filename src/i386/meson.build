sources += files(
    'gdt.S',
)

if get_option('enable_tests')
    sources += files('ktest.c')
endif

subdir('basic_drivers')
subdir('boot')
subdir('interrupt')
subdir('mm')
subdir('task')

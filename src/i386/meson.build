sources += files(
    'kinit.c',
)

if get_option('enable_tests')
    sources += files('ktest.c')
endif

subdir('basic_drivers')
subdir('boot')
subdir('gdt')
subdir('interrupt')
subdir('mm')
subdir('thread')

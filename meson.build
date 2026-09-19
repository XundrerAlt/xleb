project('xleb', 'c',
    version: '0.1.0',
    default_options: [
        'b_staticpic=false',
        'b_pie=false',
        'default_library=static',
        'c_std=c11',
    ]
)

src_dir = meson.project_source_root() / 'src'
add_project_arguments(
    '-ffreestanding',
    '-nostdlib',
    '-fno-pic',
    '-fno-pie',
    '-fno-stack-protector',
    '-fmacro-prefix-map=' + src_dir + '/=',
    language: 'c',
)

add_project_link_arguments(
    '-nostdlib',
    '-static',
    language: 'c',
)

arch = get_option('arch')
enable_tests = get_option('enable_tests')
if enable_tests
    message('Tests enabled')
    add_project_arguments('-DENABLE_TESTS', language: 'c')
endif

sources = []
subdir('src')

global_inc = include_directories(
    'src',
    'src/' + arch,
    '.',
)

kernel = executable('kernel',
    sources: sources,
    include_directories: global_inc,
    link_args: ['-T', meson.current_source_dir() / 'src' / arch / 'linker.ld'],
    link_depends: meson.current_source_dir() / 'src' / arch / 'linker.ld',
)

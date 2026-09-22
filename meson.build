project('xleb', 'c',
    version: '0.1.1',
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

rust_lib = custom_target('rust_lib',
    output: 'librustpart.a',
    command: [
        'sh', '-c',
        'cargo +nightly build -Zjson-target-spec -Zbuild-std=core,alloc --release ' +
        '--manifest-path ' + meson.current_source_dir() / 'rust_src/Cargo.toml' + ' ' +
        '--target ' + meson.current_source_dir() / 'toolchains/i386-unknown-none.json' + ' ' +
        '--target-dir ' + meson.current_build_dir() / 'rust-build' + ' && ' +
        'cp ' + meson.current_build_dir() / 'rust-build/i386-unknown-none/release/librustpart.a ' +
        '@OUTPUT@',
    ],
    build_by_default: true,
    build_always_stale: true,
)

global_inc = include_directories(
    'src',
    'src/' + arch,
    '.',
)

kernel = executable('kernel',
    sources: sources,
    include_directories: global_inc,
    link_with: [rust_lib],
    link_args: ['-T', meson.current_source_dir() / 'src' / arch / 'linker.ld'],
    link_depends: meson.current_source_dir() / 'src' / arch / 'linker.ld',
)

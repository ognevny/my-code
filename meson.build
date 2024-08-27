project(
    'ognevny-my-code',
    version: '0.0.0',
    meson_version: '>= 1.5.0',
    default_options: [
        'warning_level=everything',
        'cpp_std=gnu++11,c++11',
        'c_std=gnu11,c11',
        'rust_std=2021',
        'crossbeam-utils-0.8-rs:feature-default=true',
        'aho-corasick-1-rs:feature-perf-literal=true',
    ],
    license: 'WTFPL',
)

build_c = get_option('build_c_cpp')
build_py = get_option('build_py')
build_rust = get_option('build_rust')
build_all = get_option('build_all')
tests = get_option('tests')

if build_c or build_all
    add_languages(['c', 'cpp'], native: false)
    subdir('dad-is-great-in-C')
endif

if build_rust or build_all
    add_languages(['rust'], native: false)
    subproject('rusted')
    if tests
        custom_target(
            'cargo-test',
            build_by_default: true,
            build_always_stale: true,
            console: true,
            output: 'rusted-tests',
            command: [
                find_program('cargo'),
                'test',
                '--manifest-path',
                meson.project_source_root() / 'subprojects' / 'rusted' / 'Cargo.toml',
            ],
        )
    endif
endif

if build_py or build_all
    subdir('pie')
endif

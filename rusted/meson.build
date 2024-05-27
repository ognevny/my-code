cargo_bin = find_program('cargo')
cargo_opt = ['--manifest-path', meson.project_source_root() / 'rusted' / 'Cargo.toml']

buildt = get_option('buildtype')

if buildt == 'release' or buildt == 'plain'
    cargo_opt += ['--release']
endif

cargo_build = custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    output: 'rusted',
    console: true,
    command: [cargo_bin, 'build', cargo_opt],
)

if tests
    cargo_test = custom_target(
        'cargo-test',
        build_by_default: true,
        build_always_stale: true,
        output: 'rusted-tests',
        console: true,
        command: [cargo_bin, 'test', cargo_opt],
    )
endif

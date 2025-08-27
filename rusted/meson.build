cargo = find_program('cargo')

cargo_build = custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    console: true,
    output: 'rusted-build',
    command: [
        cargo,
        'build',
        '--manifest-path',
        meson.project_source_root() / 'rusted' / 'Cargo.toml',
    ],
)

if tests
    test_build = custom_target(
        'cargo-test',
        build_by_default: true,
        build_always_stale: true,
        console: true,
        output: 'rusted-tests',
        depends: cargo_build,
        command: [
            cargo,
            'test',
            '--manifest-path',
            meson.project_source_root() / 'rusted' / 'Cargo.toml',
        ],
    )
endif

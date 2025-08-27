cargo = find_program('cargo')
cargo_opts = [
    '--manifest-path',
     meson.project_source_root() / 'rusted' / 'Cargo.toml',
]

if get_option('ci_env')
    cargo_opts += ['--config', 'net.git-fetch-with-cli=true']
endif

cargo_build = custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    console: true,
    output: 'rusted-build',
    command: [
        cargo,
        'build',
        cargo_opts,
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
            cargo_opts,
        ],
    )
endif

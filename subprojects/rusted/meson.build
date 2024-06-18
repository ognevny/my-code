# cargo_bin = find_program('cargo')
# cargo_opt = ['--manifest-path', meson.project_source_root() / 'rusted' / 'Cargo.toml']

# buildt = get_option('buildtype')

# if buildt == 'release' or buildt == 'plain'
#     cargo_opt += ['--release']
# endif

# cargo_build = custom_target(
#     'cargo-build',
#     build_by_default: true,
#     build_always_stale: true,
#     output: 'rusted',
#     console: true,
#     command: [cargo_bin, 'build', cargo_opt],
# )

# if tests
#     custom_target(
#         'cargo-test',
#         build_by_default: true,
#         build_always_stale: true,
#         output: 'rusted-tests',
#         console: true,
#         command: [cargo_bin, 'test', cargo_opt],
#         depends: cargo_build
#     )
# endif

project('rusted', ['rust'], version: '0.0.0', default_options: ['rust_std=2021'])

src_rust = [
  'ege1/src/main.rs',
  'first_word/src/main.rs',
  'integral/src/main.rs',
  'last_word/src/main.rs',
  'longman/src/main.rs',
  'longman2/src/main.rs',
  'mask1/src/main.rs',
  'mcko/src/main.rs',
  'pop/src/main.rs',
  'probnik/src/main.rs',
  'resheto/src/main.rs',
  'speedometer/src/main.rs',
  'tumba-umba/src/main.rs',
]

foreach file : src_rust
  executable(file.split('/')[0], file)
endforeach

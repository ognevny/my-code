project(
    'ognevny-my-code',
    version: '0.0.0',
    meson_version: '>= 1.3.0',
    default_options: [
        'warning_level=3',
        'werror=true',
        'cpp_std=gnu++11,c++11',
        'c_std=gnu11,c11',
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
    add_languages('rust', native: false)
    subdir('rusted')
endif

if build_py or build_all
    subdir('pie')
endif

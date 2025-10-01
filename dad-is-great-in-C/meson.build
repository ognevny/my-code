cc = meson.get_compiler('c')
cxx = meson.get_compiler('cpp')

add_project_arguments(
    cc.get_supported_arguments('-fdiagnostics-color=always', '-pipe'),
    language: 'c',
)

add_project_arguments(
    cxx.get_supported_arguments(
        '-Wno-c++98-compat-pedantic',
        '-Wno-unsafe-buffer-usage',
        '-fdiagnostics-color=always',
        '-pipe',
    ),
    language: 'cpp',
)

src_c = [
    'b_sort.cpp',
    'firstword.cpp',
    'hords.cpp',
    'i_sort.cpp',
    'lastword.cpp',
    'longman.cpp',
    'm_sort.cpp',
    'middles.cpp',
    'resheto.cpp',
    's_sort.cpp',
    'speedometer++.cpp',
    'speedometer.c',
    'tumba-umba.cpp',
]
foreach file : src_c
    executable(file.split('.')[0], file)
endforeach

if tests
    ogntest = executable('ogntest', 'ognfunc.cpp', 'ogntest.cpp')
    test('run ogntest', ogntest)
endif

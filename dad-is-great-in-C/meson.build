project(
  'dad-is-great-in-C',
  'c',
  'cpp',
  version: '0.0.0',
  meson_version: '>= 1.3.0',
  default_options: [
    'warning_level=everything',
    'werror=true',
    'cpp_std=gnu++11,c++11',
    'c_std=gnu11,c11',
  ],
)

cc = meson.get_compiler('cpp').get_id()
if cc == 'clang'
  add_project_arguments(
    '-Wno-c++98-compat-pedantic',
    '-Wno-unsafe-buffer-usage',
    '-fdiagnostics-color=always',
    '-pipe',
    language: 'cpp',
  )
  add_project_arguments('-fdiagnostics-color=always', '-pipe', language: 'c')
elif cc != 'msvc'
  add_project_arguments(
    '-fdiagnostics-color=always',
    '-pipe',
    language: ['cpp', 'c'],
  )
endif

if build_machine.system() == 'darwin'
  add_project_arguments(
    '-Wno-poison-system-directories',
    language: ['c', 'cpp'],
  )
endif

src_cpp = [
  'b_sort',
  'firstword',
  'hords',
  'i_sort',
  'lastword',
  'longman',
  'm_sort',
  'middles',
  'resheto',
  's_sort',
  'speedometer++',
  'tumba-umba',
]
foreach file : src_cpp
  executable(file, file + '.cpp')
endforeach

executable('speedometer_c', 'speedometer.c')
executable('ogntest', 'ognfunc.cpp', 'ogntest.cpp')

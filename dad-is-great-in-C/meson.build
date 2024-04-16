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

executable('speedometer', 'speedometer.c')
executable('ogntest', 'ognfunc.cpp', 'ogntest.cpp')

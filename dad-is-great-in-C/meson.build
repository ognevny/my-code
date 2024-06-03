cc_id = meson.get_compiler('cpp').get_id()

if cc_id == 'clang'
  add_project_arguments('-fdiagnostics-color=always', '-pipe', language: ['c', 'cpp'])
  add_project_arguments(
    '-Wno-c++98-compat-pedantic',
    '-Wno-unsafe-buffer-usage',
    language: 'cpp',
  )
elif cc_id != 'msvc'
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

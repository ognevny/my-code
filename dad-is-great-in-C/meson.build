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
    'c_args=-fdiagnostics-color=always -Wp,-D_FORTIFY_SOURCE=2 -fstack-protector-strong -pipe',
    'cpp_args=-fdiagnostics-color=always -Wp,-D_FORTIFY_SOURCE=2 -fstack-protector-strong -pipe'
  ]
)

cc = meson.get_compiler('cpp').get_id()
if cc == 'clang'
  add_global_arguments('-Wno-c++98-compat-pedantic', '-Wno-unsafe-buffer-usage', language : 'cpp')
endif

if build_machine.system() == 'darwin'
  add_global_arguments('-Wno-poison-system-directories', language : ['c', 'cpp'])
endif

src_files=[
    'b_sort',
    'firstword',
    'hords',
    'i_sort'
    'lastword'
    'longman'
    'm_sort'
    'middles'
    'resheto'
    's_sort'
    'tumba-umba'
]
foreach file : src_files
    executable(file, file + '.cpp')
endforeach

executable('speedometer_cpp', 'speedometer.cpp')
executable('speedometer_c', 'speedometer.c')
executable('ogntest', 'ognfunc.cpp', 'ogntest.cpp')

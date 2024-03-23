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
  add_global_arguments('-Wno-poison-system-directories', language : 'c')
  add_global_arguments('-Wno-poison-system-directories', language : 'cpp')
endif

executable('b_sort', 'b_sort.cpp')
executable('firstword', 'firstword.cpp')
executable('hords', 'hords.cpp')
executable('i_sort', 'i_sort.cpp')
executable('lastword', 'lastword.cpp')
executable('longman', 'longman.cpp')
executable('m_sort', 'm_sort.cpp')
executable('middles', 'middles.cpp')
executable('resheto', 'resheto.cpp')
executable('s_sort', 's_sort.cpp')
executable('speedometer_cpp', 'speedometer.cpp')
executable('speedometer_c', 'speedometer.c')
executable('tumba-umba', 'tumba-umba.cpp')
executable('ogntest', 'ognfunc.cpp', 'ogntest.cpp')

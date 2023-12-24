## rusted

**RU**: на данный момент я самостоятельно учу Rust, иногда пишу "поделки" для школы, но больше
всё-таки для себя. Название директория банально означает "ржавый". вы можете скомпилировать
все пакеты за исключением тех, что используют SFML с помощью команды `make rust`,
`make rust-with-sfml` скомпилирует абсолютно все проекты. если вы хотите использовать линкер mold,
вы можете использовать его, задав `MOLD=1`. если вам нужны оптимизированные бинарники, то можете
задать `PROFILE=release`. также можно собрать SFML из исходного кода, задав `SFML_SOURCE=1`.

**ENG**: at the moment I'm learning Rust myself, sometimes write some code for school, but
especially for myself. Name of directory literally means "rusted". you can run `make rust` to
compile all projects excluding SFML ones, `make rust-with-sfml` will compile everything. if you wish
to use mold linker, you can enable it by setting `MOLD=1`. if you need to have optimized binaries,
you can set `PROFILE=release`. also you can build SFML from source by setting `SFML_SOURCE=1`

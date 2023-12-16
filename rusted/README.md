## rusted

**RU**: на данный момент я самостоятельно учу Rust, иногда пишу "поделки" для школы, но больше
всё-таки для себя. Название директория банально означает "ржавый". вы можете также скомпилировать
все пакеты за исключением тех, что используют SFML с помощью команды `make rust`, также можно
испоьзовать `make nightly`, чтобы скомпилировать все проекты, в том числе SFML последней версии
(используется для CI). если вы хотите использовать линкер mold, вы можете использовать его так
`make rust MOLD=true` или `make nightly MOLD=true`. если вам нужны оптимизированные бинарники, то
можете написать `PROFILE=release` для каждой из команд.

**ENG**: at the moment I'm learning Rust myself, sometimes write some code for school, but
especially for myself. Name of directory literally means "rusted". you can also run `make rust` to
compile all prjects excluding SFML ones. `make nightly` will compile everything, including latest
version of SFML (used for CI). if you wish to use mold linker, you can enable it by running
`make rust MOLD=true` or `make nighlty MOLD=true`. if you need to have optimized binaries, you can
write `PROFILE=release`.

# About speedometer

**RU**: speedometer - это программы, которые тестируют ЯП на скоррость. В моём репозитории
представлены коды на C, C++, Rust и Python. Данная программа не имеет практической пользы.
**ENG**: speedometer is a program that tests the speed of the programming languages. My repo
contains code in C, C++, Rust, and Python. This program has no practical use.

## What it does?

**RU**: создаётся функция `s()`, ничего не принимающая на вход, но возвращающая строго одну и ту же
величину - миллиард. Она это делает следущим образом: создаётся переменная `n`, равная 1, и пока `n`
меньше миллиарда, прибавляется 1.
**ENG**: creates a function `s()` that takes nothing as input, but returns strictly the same value -
a billion. It does this as follows: It creates a variable `n` equal to 1, and while `n` is less than
a billion, it adds 1.

## Manual testing

**RU**: Вы можете самостоятельно протестировать эти программы. Я покажу Вам, как это делаю я:
Оборудование:
ноутбук на базе AMD Radeon 5 5600H, 16 Гб ОЗУ, программы хранятся на SSD 512 ГБ *(напишите, если
видеокарта влияет тоже)*.
Инструменты:
OS - Windows 11 последней версии;
[дистрибутив MSYS2](https://msys2.org), который предоставляет мне:
clang 17.0.6 - компилятор C и C++;
Python 3.11.6 с numba 0.58.1;
Rust 1.74.0;
Командная оболочка `fish` 3.6.1, и её команда `time`.
Ниже (под ENG) будет представлен лог команд, которые я вписывал в терминал
**ENG**: You can test these programs yourself. I will show you how I do it:
Hardware:
AMD Radeon 5 5600H based laptop, 16GB RAM, programs stored on 512GB SSD
*(write me if video card affects too)*.
Tools:
OS - Windows 11 latest version;
[MSYS2 distribution](https://msys2.org) which provides me:
clang 17.0.6 - C and C++ compiler;
Python 3.11.6 with numba 0.58.1;
Rust 1.74.0;
Command line shell `fish` 3.6.1 and its command `time`.
Below will be a log of the commands I typed into the terminal

#### C

```console
$ clang speedometer.c
$ time ./a
1000000000
________________________________________________________
Executed in  525,38 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

#### C++

```console
$ clang++ speedometer.cpp
$ time ./a
1000000000
________________________________________________________
Executed in  531,55 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

#### Python

speedometer.py

```console
$ time python speedometer.py
1000000000

________________________________________________________
Executed in   25,53 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

numbed.py

```console
$ time python numbed.py
1000000000

________________________________________________________
Executed in  603,01 millis    fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis
```

#### Rust

```console
$ cargo b
   Compiling speedometer v0.1.0 (C:\...\my-code\rusted\speedometer)
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
$ time ../target/debug/speedometer
1000000000
________________________________________________________
Executed in  754,14 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

### Reference

**RU**: данные коды вдохновлены видео на ютуб от The Builder
**ENG**: these codes are inspired by The Builder's video on YouTube
[ссылка/link](https://www.youtube.com/watch?v=VioxsWYzoJk)

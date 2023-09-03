# About speedometer

**RU**: speedometer - это программы, которые тестируют ЯП на скоррость. В моём репозитории представлены коды на C, C++, Rust и Python. Данная программа не имеет практической пользы, да и не выполняет какое-либо полезное действие.
**ENG**: speedometer is a program that tests the speed of the programming languages. My repo contains code in C, C++, Rust, and Python. This program has no practical use, and does not perform any useful action.

## What it does?

**RU**: создаётся функция `s()`, ничего не принимающая на вход, но возвращающая строго одну и ту же величину - миллиард. Она это делает следущим образом: создаётся переменная `n`, равная 1, и пока `n` меньше миллиарда, прибавляется 1.
**ENG**: creates a function `s()` that takes nothing as input, but returns strictly the same value - a billion. It does this as follows: It creates a variable `n` equal to 1, and while `n` is less than a billion, it adds 1.

## Manual testing

**RU**: Вы можете самостоятельно протестировать эти программы. Я покажу Вам, как это делаю я:
Оборудование:
ноутбук на базе AMD Radeon 5 5600H, 16 Гб ОЗУ, программы хранятся на SSD 512 ГБ *(напишите, если видеокарта влияет тоже)*.
Инструменты:
OS - Windows 11 последней версии;
[дистрибутив MSYS2](https://msys2.org), который предоставляет мне:
clang 16.0.5 - компилятор C и C++;
Python 3.11.5 с numba 0.57.0;
Rust 1.72.0;
Командная оболочка `fish` 3.6.1, и её команда `time`.
Ниже (под ENG) будет представлен лог команд, которые я вписывал в терминал
**ENG**: You can test these programs yourself. I will show you how I do it:
Hardware:
AMD Radeon 5 5600H based laptop, 16GB RAM, programs stored on 512GB SSD *(write me if video card affects too)*.
Tools:
OS - Windows 11 latest version;
[MSYS2 distribution](https://msys2.org) which provides me:
clang 16.0.5 - C and C++ compiler;
Python 3.11.5 with numba 0.57.0;
Rust 1.72.0;
Command line shell `fish` 3.6.1 and its command `time`.
Below will be a log of the commands I typed into the terminal

#### C

```console
~/m/dad-is-great-in-C> clang speedometer.c
~/m/dad-is-great-in-C> time ./a
1000000000
________________________________________________________
Executed in  522,78 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

#### C++

```console
~/m/dad-is-great-in-C> clang++ speedometer.cpp
~/m/dad-is-great-in-C> time ./a
1000000000
________________________________________________________
Executed in  547,84 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

#### Python

speedometer.py

```console
~/m/pie> time python speedometer.py
1000000000

________________________________________________________
Executed in   27,44 secs      fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis
```

numbed.py
*now msys2 numba fails as numpy has version greater than supported*

```console
~/m/pie> time python numbed.py
1000000000

________________________________________________________
Executed in    3,01 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

#### Rust

```console
~/m/r/speedometer> cargo b
   Compiling speedometer v0.1.0 (C:\...\my-code\rusted\speedometer)
    Finished dev [unoptimized] target(s) in 0.00s
~/m/r/speedometer> time cargo r
    Finished dev [unoptimized] target(s) in 0.00s
     Running `target\debug\speedometer.exe`
1000000000
________________________________________________________
Executed in  800,30 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```

### Reference

**RU**: данные коды вдохновлены видео на ютуб от The Builder
**ENG**: these codes are inspired by The Builder's video on YouTube
[ссылка/link](https://www.youtube.com/watch?v=VioxsWYzoJk)

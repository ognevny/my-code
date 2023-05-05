# About speedometer
**RU**: speedometer - это программы, которые тестируют ЯП на скоррость. В моём репозитории представлены коды на C, C++, Rust и Python. Данная программа не имеет практической пользы, да и не выполняет какое-либо полезное действие.  
**ENG**: speedometer is a program that tests the speed of the programming languages. My repo contains code in C, C++, Rust, and Python. This program has no practical use, and does not perform any useful action.
## What it does?
**RU**: создаётся функция `s()`, ничего не принимающая на вход, но возвращающая строго одну и ту же величину - миллиард. Она это делает следущим образом: создаётся переменная `n`, равная 1, и пока `n` меньше миллиарда, прибавляется 1.  
**ENG**: creates a function `s()` that takes nothing as input, but returns strictly the same value - a billion. It does this as follows: It creates a variable `n` equal to 1, and while `n` is less than a billion, it adds 1.
## Manual testing
**RU**: Вы можете самостоятельно протестировать эти программы. Я покажу Вам, как это делаю я:  
Оборудование:  
ноутбук на базе AMD Radeon 5 5600H, 16 Гб ОЗУ, программы хранятся на ссд 512 ГБ *(напишите, если видеокарта влияет тоже)*.  
Инструменты:  
OS - Windows 11 последней версии;  
[дистрибутив MSYS2](https://msys2.org), который предоставляет мне:  
clang 16.0.1 - компилятор C и C++  
Python 3.10.11 с numba 0.57.0dev0.r1074.gc12bbf4bb-1  
Rust 1.68.2  
Командная оболочка `fish` 3.6.1, и её команда `time`.  
Ниже (под ENG) будет представлен лог команд, которые я вписывал в терминал  
**ENG**: You can test these programs yourself. I will show you how I do it:  
Hardware:  
AMD Radeon 5 5600H based laptop, 16GB RAM, programs stored on 512GB SSD *(write me if video card affects too)*.  
Tools:  
OS - Windows 11 latest version;  
[MSYS2 distribution](https://msys2.org) which provides me:  
clang 16.0.1 - C and C++ compiler;  
Python 3.10.11 with numba 0.57.0dev0.r1074.gc12bbf4bb-1;  
Rust 1.68.2;  
Command line shell `fish` 3.6.1 and its command `time`.  
Below will be a log of the commands I typed into the terminal
#### C
```fish
~/m/dad-is-great-in-C> clang -O3 speedometer.c
~/m/dad-is-great-in-C> time ./a
1000000000
________________________________________________________
Executed in   64,89 millis    fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis
```
#### C++
```fish
~/m/dad-is-great-in-C> clang++ -O3 speedometer.cpp
~/m/dad-is-great-in-C> time ./a
1000000000
________________________________________________________
Executed in   55,05 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
#### Python
speedometer.py
```fish
~/m/pie> time python speedometer.py
1000000000

________________________________________________________
Executed in   26,65 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
numbed.py
```fish
~/m/pie> time python numbed.py
1000000000

________________________________________________________
Executed in    3,01 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
#### Rust
```fish
~/m/r/speedometer> cargo b -r
   Compiling speedometer v0.1.0 (C:\...\my-code\rusted\speedometer)
    Finished release [optimized] target(s) in 0.96s
~/m/r/speedometer> time cargo r -r
    Finished release [optimized] target(s) in 0.00s
     Running `target\release\speedometer.exe`
1000000000
________________________________________________________
Executed in   57,09 millis    fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis
```
### Reference
**RU**: данные коды вдохновлены видео на ютуб от The Builder 
**ENG**: these codes are inspired by The Builder's video on YouTube  
[ссылка/link](https://www.youtube.com/watch?v=VioxsWYzoJk)

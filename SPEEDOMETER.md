# About speedometers
**RU**: speedometer - это программы, которые тестируют ЯП на скоррость. В моём репозитории представлены коды на C, C++, Rust и Python. Данная программа не имеет практической пользы, да и не выполняет какое-либо полезное действие.  
**ENG**: speedometer is a program that tests the speed of the programming languages. My repo contains code in C, C++, Rust, and Python. This program has no practical use, and does not perform any useful action.
## What it does?
**RU**: создаётся функция ```s()```, ничего не принимающая на вход, но возвращающая строго одну и ту же величину - миллиард. Она это делает следущим образом: создаётся переменная ```n```, равная 1, и пока ```n``` меньше миллиарда, прибавляется 1.  
**ENG**: creates a function ```s()``` that takes nothing as input, but returns strictly the same value - a billion. It does this as follows: It creates a variable ```n``` equal to 1, and while ```n``` is less than a billion, it adds 1.
## Manual testing
**RU**: Вы можете самостоятельно протестировать эти программы. Я покажу Вам, как это делаю я:  
Инструменты:  
OS - Windows 11 последней версии;  
[дистрибутив MSYS2](msys2.org), который предоставляет мне:  
clang 16.0.0 - компилятор C и C++  
Python 3.10.10 with numba 0.40.0dev  
Rust 1.68.2  
Командная оболчочка ```fish``` 3.6.1, и её команда ```time```.  
Ниже (под ENG) будет представлен лог команд, которые я вписывал в терминал  
**ENG**: You can test these programs yourself. I will show you how I do it:  
Tools:  
OS - Windows 11 latest version;  
[MSYS2 distribution](msys2.org) which provides me:  
clang 16.0.0 - C and C++ compiler;  
Python 3.10.10 with numba 0.40.0dev;  
Rust 1.68.2;  
Command shell ```fish``` 3.6.1 and its command ```time```.  
Below will be a log of the commands I typed into the terminal
#### C
```
dad-is-great-in-C> clang speedometer.c
dad-is-great-in-C> time ./a
1000000000

________________________________________________________
Executed in  557,65 millis    fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis 
```
#### C++
```
dad-is-great-in-C> clang++ speedometer.cpp
dad-is-great-in-C> time ./a
1000000000

________________________________________________________
Executed in  533,74 millis    fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
#### Python
speedometer.py
```
pie> time python speedometer.py
1000000000

________________________________________________________
Executed in   29,21 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
speedometer_p.py
```
pie> time python speedometer_p.py
1000000000

________________________________________________________
Executed in   27,22 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
numbed.py
```
pie> time python numbed.py
1000000000

________________________________________________________
Executed in    3,09 secs      fish           external
   usr time    0,00 micros    0,00 micros    0,00 micros
   sys time    0,00 micros    0,00 micros    0,00 micros
```
#### Rust
```
r/speedometer> cargo build --release
   Compiling speedometer v0.1.0 (C:\...\rusted\speedometer)
    Finished release [optimized] target(s) in 0.96s
r/speedometer> time cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target\release\speedometer.exe`
1000000000
________________________________________________________
Executed in   62,77 millis    fish           external
   usr time    0,00 millis    0,00 micros    0,00 millis
   sys time   15,00 millis    0,00 micros   15,00 millis
```
### Reference
**RU**: данные коды влохновлены видео на ютуб от Top Schelf Technology  
**ENG**: these codes are inspired by Top Schelf Technology's video on YouTube  
[ссылка/link](https://www.youtube.com/watch?v=VioxsWYzoJk)

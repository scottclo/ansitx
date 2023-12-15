# ansitx (Work in progress)
Converts strings containing ANSI escapes to plain text while retaining cursor movements.
Currently, it only supports a limited number of ANSI functions but is functional for neofetch style programs.

## Example
plain text using ansi2txt
```console
$ pfetch | ansi2txt
    _______
 _ \______ -
| \  ___  \ |
| | /   \ | |
| | \___/ | |
| \______ \_|
 -_______\
scott@shop-pc
osVoid Linux
hostMS-7C91 2.0
kernel6.5.13_1
uptime9d 29m
pkgs794
memory5663M / 15920M
```

plain text using ansitx
```console
$ pfetch | ansitx
    _______      scott@shop-pc
 _ \______ -     os     Void Linux
| \  ___  \ |    host   MS-7C91 2.0
| | /   \ | |    kernel 6.5.13_1
| | \___/ | |    uptime 9d 15m
| \______ \_|    pkgs   794
 -_______\       memory 5457M / 15920M
```

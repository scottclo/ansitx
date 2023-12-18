# ansitx (Work in progress)
Converts text containing ANSI escapes to plain text while retaining cursor movements.
Currently, it only supports a limited number of ANSI functions but it's functional for neofetch and other uses.

## Examples
With ansitx
```console
$ pfetch | ansitx > plain.txt
$ vi plain.txt
    _______      scott@shop-pc
 _ \______ -     os     Void Linux
| \  ___  \ |    host   MS-7C91 2.0
| | /   \ | |    kernel 6.5.13_1
| | \___/ | |    uptime 11d 22h 5m
| \______ \_|    pkgs   808
 -_______\       memory 3621M / 15920M

```
Without ansitx
```console
$ pfetch > plain.txt
$ vi plain.txt
^[[?7l^[[1m^[[32m    _______
^[[32m _ \______ -
^[[32m| \  ___  \ |
^[[32m| | /   \ | |
^[[32m| | \___/ | |
^[[32m| \______ \_|
^[[32m -_______\
^[[m^[[7A^[[17C^[[32;1m^[[33mscott^[[37m@^[[33mshop-pc^[[m^[[28D^[[7C^[[37m^[[m
^[[17C^[[32;1mos^[[m^[[2D^[[7C^[[37mVoid Linux^[[m
^[[17C^[[32;1mhost^[[m^[[4D^[[7C^[[37mMS-7C91 2.0^[[m
^[[17C^[[32;1mkernel^[[m^[[6D^[[7C^[[37m6.5.13_1^[[m
^[[17C^[[32;1muptime^[[m^[[6D^[[7C^[[37m11d 22h 5m^[[m
^[[17C^[[32;1mpkgs^[[m^[[4D^[[7C^[[37m808^[[m
^[[17C^[[32;1mmemory^[[m^[[6D^[[7C^[[37m3621M / 15920M^[[m

^[[?7h

```

## Planned/Supported ANSI commands
- [x] A - Cursor Up
- [x] B - Cursor Down
- [x] b - Cursor Vertical Absolute
- [x] C - Cursor Forward
- [x] D - Cursor Back
- [ ] E - Curosr Cursor Next Line 
- [ ] F - Cursor Previus Line
- [ ] f - Horizontal Vertical Position
- [x] G - Cursor Horizontal Absolute
- [x] H - Cursor Poition
- [x] J - Erase in Display
- [ ] K - Erase in Line
## Planned/Suported ASCII control charactars
- [ ] BS - Back Space
- [ ] HT(\t) - Horizontal Tab
- [x] LF(\n) - Line Feed
- [x] CR(\r) - Carrage Return
- [x] ESC(\e) - Escape

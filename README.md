# ansitx (Work in progress)
Converts input containing ANSI escapes to plain text while retaining cursor movements.

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
## Instalation
Install ```rust```, ```cargo``` and ```git``` and run:
```console
git clone https://github.com/ScottCLo/ansitx.git
cd ansitx
cargo build -r
cd target/release/
```
You can run the program from here with:
```console
./ansitx
```
Or move it to ```/usr/local/bin/``` to be able to run it globaly.
```console
sudo mv ./ansitx /usr/local/bin/
```

## Supported Commands
### ANSI
- [x] A - Cursor Up
- [x] B - Cursor Down
- [x] C - Cursor Forward
- [x] D - Cursor Back
- [x] d - Cursor Vertical Absolute
- [x] E - Curosr Cursor Next Line 
- [x] F - Cursor Previus Line
- [x] f - Horizontal Vertical Position
- [x] G - Cursor Horizontal Absolute
- [x] H - Cursor Poition
- [x] J - Erase in Display
- [x] K - Erase in Line
### ASCII Control Charactars
- [x] BS - Back Space
- [x] HT - Horizontal Tab
- [x] HT - Horizontal Tab
- [x] LF - Line Feed
- [x] CR - Carrage Return
- [x] ESC - Escape

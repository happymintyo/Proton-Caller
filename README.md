# Proton-Caller
Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

[![Language grade: C/C++](https://img.shields.io/lgtm/grade/cpp/g/caverym/Proton-Caller.svg?logo=lgtm&logoWidth=18)](https://lgtm.com/projects/g/caverym/Proton-Caller/context:cpp)

[Usage](https://github.com/caverym/Proton-Caller#usage)

Please create an issue if you want added features or have an issue.

https://aur.archlinux.org/packages/proton-caller/

## Problem Reporting:
Please create an issue on the [Github](https://github.com/caverym/Proton-Caller) page which lists: system, kernel version, game, shell, and if it is or isn't a Steam game – provide how you had installed it and where it is installed. Additionally provide screenshots of the shell. Try many methods to get it to work and describe what you did in your issue.


## Usage:

-c \- custom mode<br>
-h \- help information<br>
help \- same as -h
```
proton-call 5 SpaceEngine.exe
```

```
proton-call -c "/home/avery/.steam/steam/steamapps/common/Proton 5.13" SpaceEngine.exe
```
## Install:

To install `proton-call`
```
yay -S proton-caller
 ``` 

or: (with makepkg)

```
git clone https://aur.archlinux.org/proton-caller.git
cd proton-caller
makepkg -si
```
or:
```
git clone https://github.com/caverym/Proton-Caller.git
cd Proton-Caller
make
sudo make install
```

### Space Engine example:
   Make a .desktop launcher. [example file](Space%20Engine.desktop)
   
   ```
[Desktop Entry]
Type=Application
Name=Space Engine
Comment=Space Engine
Exec=proton-call 5.13 SpaceEngine.exe
Path=/home/avery/Documents/games/SpaceEngine/system
Terminal=false
StartupNotify=false
   ```

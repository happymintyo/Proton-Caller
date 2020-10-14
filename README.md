# Proton-Caller
A script to automatically run programs using Steam's Proton


Please create an issue if you want added features or have an issue.

https://aur.archlinux.org/packages/proton-caller/

## Usage:

-c \- custom mode<br>
-h \- help information<br>
help \- same as -h
```
proton-call 5 SpaceEngine.exe
```

```
proton-call -c '/home/avery/.steam/steam/steamapps/common/Proton\\ 5.0/' SpaceEngine.exe
```

## Install:

To install `proton-call`
```
yay -S proton-caller
 ``` 

or:

```
git clone https://aur.archlinux.org/proton-caller.git
cd proton-caller
makepkg -si
```

### Space Engine example:
   To make a .desktop launcher create a shell script such as `/usr/bin/SpaceEngine`:
   ```
   #!/usr/bin/env zsh
   export STEAM_COMPAT_DATA_PATH=/home/avery/proton
   cd /home/avery/proton/pfx/drive_c/SpaceEngine/system
   proton-call 5 SpaceEngine.exe
   ```
   Point it at the executable, then run it using proton-call.
   ```
   chmod +x /usr/bin/SpaceEngine
   ```
   
   Then create a .desktop file which points at `/usr/bin/SpaceEngine`
   ```
   Exec=SpaceEngine
   Terminal=false
   Type=Application
   Categories=Game;;
   ```

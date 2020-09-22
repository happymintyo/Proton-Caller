# Proton-Caller
A script to automatically run programs using Steam's Proton


Please create an issue if you want added features or have an issue.

## Install:

   To install Proton Caller to path, run install.sh.<br>
   `sudo sh ./install.sh`<br>
   This will install `proton-call` as a command on your system.<br>

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
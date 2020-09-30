#!/usr/bin/env sh

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

python3 -c "import _pyinstaller_hooks_contrib"

if [ "$?" -ne "0"]
  then echo "Install pyinstaller"
  exit
fi

pyinstaller --onefile protoncall.py
cp dist/protoncall /usr/bin/proton-call
chmod +x /usr/bin/proton-call
install -g 0 -o 0 -m 0644 proton-call.6 /usr/share/man/man6/
gzip /usr/share/man/man6/proton-call.6
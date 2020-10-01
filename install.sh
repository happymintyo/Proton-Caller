#!/usr/bin/env sh

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

pyinstaller --onefile protoncall.py
cp dist/protoncall /usr/bin/proton-call
chmod +x /usr/bin/proton-call
install -g 0 -o 0 -m 0644 proton-call.6 /usr/share/man/man6/
gzip -f /usr/share/man/man6/proton-call.6
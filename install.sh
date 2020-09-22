#!/usr/bin/env sh

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

cp ./protoncall.py /usr/bin/proton-call
chmod +x /usr/bin/proton-call
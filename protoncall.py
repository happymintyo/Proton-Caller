#!/usr/bin/env python3
import os
import sys


n = 'null'
steam = 'STEAM_COMPAT_DATA_PATH'
custom = False
version = '0.5.3'

# Set defaults
proton = sys.argv[1]
program = n
_help = True
_home = os.environ['HOME']


print(f"Proton Caller by Avery Murray, version: {version}.\n")


def help_message():
    print(f"Proton Caller by Avery Murray, version: {version}.\n"
          f"Use this to easily run any Windows executable using Steam's Proton\n\n"
          f"Usage:\n [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n"
          f"'proton-call 5 ./foo.exe'\n'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n\n"
          f"Working directory must be the same as the Windows executable. Proton Caller will fail without arguments.")
    sys.exit(err_val)


def setup():
    print(f'Custom mode: {custom}')
    if steam in os.environ:
        print(f'{steam} exists at {os.environ[steam]}\b')
        return
    else:
        print(f'{steam} was not found')
        sys.exit(1)


def proton_call():
    if _help:
        help_message()
    setup()
    if custom:
        os.system(f'{proton_path}/proton run {program}')
    else:
        os.system(f'{_home}/.steam/steam/steamapps/common/Proton\\ {proton}/proton run {program}')
    return


# check mode
if len(sys.argv[1]) > 4:
    err_val = 255
    proton_call()
    sys.exit(0)
elif sys.argv[1] == 'help':
    err_val = 0
    proton_call()
    sys.exit(0)
elif sys.argv[1] == '-h':
    err_val = 0
    proton_call()
    sys.exit(0)
elif sys.argv[1] == '-c':
    _help = False
    custom = True
    proton_path = sys.argv[2]
    program = sys.argv[3]
    proton_call()
    sys.exit(0)
else:
    _help = False
    proton = sys.argv[1]
    if sys.argv[1] == '5':
        proton = '5.0'
    program = sys.argv[2]
    proton_call()
    sys.exit(0)

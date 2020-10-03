#!/usr/bin/env python3
import os
import sys


n = 'null'
steam = 'STEAM_COMPAT_DATA_PATH'
custom = False
version = '0.6.1a'

# Set defaults
proton = sys.argv[1]
program = n
_help = True
_home = os.environ['HOME']


print(f"Proton Caller {version}.\n")


def help_message():
    print(f"Proton Caller by Avery Murray, version: {version}.\n"
          f"Use this to easily run any Windows executable using Steam's Proton\n\n"
          f"Usage:\n [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n"
          f"'proton-call 5 ./foo.exe'\n'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n\n"
          f"Working directory must be the same as the Windows executable. Proton Caller will fail without arguments.")
    sys.exit(err_val)


def proton_addition():
    os.system(f"mkdir {_home}/proton")
    profile = input(f"where is your shell profile file (.bash_profile, .zshrc): ")
    os.system(f"echo 'export {steam}={_home}/proton >>! {profile}")
    return


def find_proton():
    exists = os.path.isdir(f'{_home}/.steam/steam/steamapps/common/Proton {proton}')
    if exists:
        print(f'{proton} exists.\n')
        return
    else:
        print(f'{proton} does not exist.')
        sys.exit(1)


def set_type():
    if custom:
        _type = sys.argv[2]
    if not custom:
        _type = f'{_home}/.steam/steam/steamapps/common/Proton\\ {proton}'
    return _type


def setup():
    print(f'Custom mode: {custom}')
    find_proton()
    if steam in os.environ:
        print(f'{steam} exists at {os.environ[steam]}')
        a = set_type()
        return a
    else:
        proton_add = input(f"Would you like to add {steam}? [y/n]")
        if proton_add == 'Y':
            proton_add = 'y'
        if proton_add == 'y':
            proton_addition()
            return
        else:
            print(f"Please manually add {steam} to your environment.")
            sys.exit(1)


def proton_call():
    if _help:
        help_message()
    proton_path = setup()
    if custom:
        os.system(f'{proton_path}/proton run {program}')
    else:
        os.system(f'{proton_path}/proton run {program}')
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
    #  proton_path = sys.argv[2]
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

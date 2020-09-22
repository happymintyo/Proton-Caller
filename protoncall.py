#!/usr/bin/env python3
import os
import sys

scdp = 'STEAM_COMPAT_DATA_PATH'
custom = 'unknown'
version = '0.4'


# setup mode

def help_message():
	print(f"Proton Caller by Avery Murray, version: {version}.\n"
		f"Use this to easily run any Windows executable using Steam's Proton\n\n"
		f"Usage:\n [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n"
		f"'proton-call 5 ./foo.exe'\n'proton-call -c '/Proton\ 5.0/' ./foo.exe'\n\n"
		f"Working directory must be the same as the Windows executable. Proton Caller will fail without arguments.")
	sys.exit(a)


def proton_addition():
	os.system('mkdir ~/proton')
	os.system("echo 'export STEAM_COMPAT_DATA_PATH=$HOME/proton' >> ~/.profile")
	print("If this did not work, manually add it to your shell profile.")
	return


def proton_select():
	prtn = sys.argv[1]
	if prtn == '5':
		prtn = '5.0'
		return prtn
	elif prtn > '3':
		return prtn
	else:
		print('\n Select proton version.')
		sys.exit(1)


#  launcher mode

def setup():
	print(f'Custom mode: {custom}')
	if scdp in os.environ:
		print(f'{scdp} exists at: {os.environ[scdp]} \n')
	else:
		print(f'{scdp} was not found')
		add_var = input('Would you like this added? [y/n]: ')
		if add_var == 'y':
			proton_addition()
		elif add_var == 'yes':
			proton_addition()
		else:
			print(f'Please add {scdp} to your environment variables')
			sys.exit(1)


def program_select():
	if custom == 'True':
		program = sys.argv[3]
	else:
		program = sys.argv[2]
	print(program)
	return program


def proton_list():
	print('Finding Proton installs…')
	os.system('ls ~/.steam/steam/steamapps/common/ | grep Pr')
	return


def proton_call():
	setup()
	proton_list()
	prtn = proton_select()
	print(f'Using Proton {prtn}')
	program = program_select()
	os.system(f'~/.steam/steam/steamapps/common/Proton\ {prtn}/proton run {program}')


# custom mode

def custom_call():
	setup()
	proton_path = sys.argv[2]
	program = program_select()
	os.system(f'{proton_path}/proton run {program}')
	sys.exit(0)


# check mode

if len(sys.argv[1]) > 4:
	a = 1
	help_message()
elif sys.argv[1] == 'help':
	a = 0
	help_message()
elif sys.argv[1] == '-h':
	a = 0
	help_message()
elif sys.argv[1] == '-c':
	custom = 'True'
	custom_call()
else:
	custom = 'False'
	proton_call()


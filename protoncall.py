#!/usr/bin/env python3
import os
import sys
import time


scdp = 'STEAM_COMPAT_DATA_PATH'


# setup mode

def help_message():
	print("usage: [5, 5.0, 4.11, 4.3, 3.16, 3.7] [help, ./.exe]\n example: \n")
	print("'proton-call 4.11 ./foo.exe' will run foo.exe with Proton version 4.11. \n")
	print("You must also be in the same directory as the Windows executable.")
	print("Proton caller will error without any arguments. \n")
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
	if scdp in os.environ:
		print(f'{scdp} exists at: {os.environ[scdp]}')
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
	program = sys.argv[2]
	print(program)
	return program


def proton_list():
	print('Finding Proton installs…')
	os.system('ls ~/.steam/steam/steamapps/common/ | grep Pr')
	return


def proton_call():
	print("Warning: the script currently only supports Proton installed in the user's home directory \n")
	time.sleep(1)
	proton_list()
	prtn = proton_select()
	print(f'Using Proton {prtn}')
	program = program_select()
	os.system(f'~/.steam/steam/steamapps/common/Proton\ {prtn}/proton run {program}')


if len(sys.argv[1]) > 4:
	a = 1
	help_message()
elif sys.argv[1] == 'help':
	a = 0
	help_message()
elif sys.argv[1] == '-h':
	a = 0
	help_message()


proton_call()

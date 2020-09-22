import os
import sys
import time


scdp = 'STEAM_COMPAT_DATA_PATH'


if sys.argv[1] == 'help':
	print("usage: [help, ./.exe] [5, 5.0, 4.11] \n"
	      " example: \n"
	      "'proton-call ./foo.exe 4.11' will run foo.exe with Proton version 4.11. \n"
	      "If you do not specify a version, proton-call will ask. \n"
	      "You must also be in the same directory as the Windows executable. \n")
	exit(0)

print("Warning: the script currently only supports Proton installed in the user's home directory \n")
time.sleep(0.5)



if scdp in os.environ:
	print(f'{scdp} exists at: {os.environ[scdp]}')
else:
	print(f'Please add {scdp} to environment variables')
	sys.exit(1)


def proton_list():
	print('Finding Proton installs…')
	os.system('ls ~/.steam/steam/steamapps/common/ | grep Pr')
	return


def proton_select():
	prtn = input(f'Select version: ')
	if prtn == '5':
		prtn = '5.0'
		return prtn
	elif prtn == '5.0':
		return prtn
	elif prtn == '4.11':
		return prtn
	elif prtn == '':
		print('Select proton version.')
		sys.exit(1)
	else:
		print(f"Sorry, '{prtn}' isn't supported")
		sys.exit(1)


def program_select():
	program = sys.argv[1]
	print(program)
	return program


def proton_call():
	proton_list()
	prtn = proton_select()
	print(f'Using Proton {prtn}')
	program = program_select()
	os.system(f'~/.steam/steam/steamapps/common/Proton\ {prtn}/proton run {program}')


proton_call()

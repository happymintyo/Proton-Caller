import os
import sys
import time


scdp = 'STEAM_COMPAT_DATA_PATH'


if sys.argv[1] == 'help':
	print("usage: [5, 5.0, 4.11, 4.3, 3.16, 3.7] [help, ./.exe]\n"
	      " example: \n"
	      "'proton-call 4.11 ./foo.exe' will run foo.exe with Proton version 4.11. \n"
	      "You must also be in the same directory as the Windows executable. \n")
	exit(0)

print("Warning: the script currently only supports Proton installed in the user's home directory")
time.sleep(1)


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
	prtn = sys.argv[1]
	if prtn == '5':
		prtn = '5.0'
		return prtn
	elif prtn == '5.0':
		return prtn
	elif prtn == '4.11':
		return prtn
	elif prtn == '4.3':
		return prtn
	elif prtn == '3.16':
		return prtn
	elif prtn == '3.7':
		return prtn
	else:
		print('\n Select proton version.')
		sys.exit(1)


def program_select():
	program = sys.argv[2]
	print(program)
	return program


def proton_call():
	proton_list()
	prtn = proton_select()
	print(f'Using Proton {prtn}')
	program = program_select()
	os.system(f'~/.steam/steam/steamapps/common/Proton\ {prtn}/proton run {program}')


proton_call()

#!/usr/bin/env python3
import os
import sys

n = 'null'
scdp = 'STEAM_COMPAT_DATA_PATH'
custom = False
version = '0.5a'
_argv1 = sys.argv[1]

# Set defaults
err_val = 255
proton = n
program = n
proton_path = "~/.steam/steam/steamapps/common/Proton\\ 5.0/"
_help = True


def help_message():
	print(f"Proton Caller by Avery Murray, version: {version}.\n"
	f"Use this to easily run any Windows executable using Steam's Proton\n\n"
	f"Usage:\n [-c, -h][5, 5.0, 4.11, 4.3, 3.16, 3.7][./*.exe]\n"
	f"'proton-call 5 ./foo.exe'\n'proton-call -c '/Proton\\ 5.0/' ./foo.exe'\n\n"
	f"Working directory must be the same as the Windows executable. Proton Caller will fail without arguments.")
	sys.exit(_vars.err_val)


def proton_addition():
	add_dir = input('Directory you will like {scdp} added.'
					'this will add a directory.\nadd_dir: ')
	os.system(f'mkdir {add_dir}/proton')
	_profile = input('Shell profile file: ')
	os.system(f"echo 'export STEAM_COMPAT_DATA_PATH={add_dir}/proton' > {_profile}")
	_vars.scdp = f'{add_dir}/proton'
	return _vars.scdp


def setup():
	print(f'Custom mode: {custom}')
	if scdp in os.environ:
		print(f'{scdp} exists at {os.environ[scdp]}\b')
		return
	else:
		print(f'{scdp} was not found')
		add_var = input(f'Would you like to add {scdp}? [y/n]: ')
		if add_var == 'y':
			_vars.scdp = proton_addition()
			return _vars.scdp
		elif add_var == 'n':
			sys.exit(f"Won't add {scdp}. Add it to your environment")
		else:
			sys.exit(_vars.err_val)


def proton_call():
	print(f'\n {_vars}\n Made it to proton_call !')
	if _help:
		help_message()
	setup()
	if custom:
		print(f'Using Proton {_vars.proton}')
		print(f'Program: {_vars.program}')
		os.system(f"{_vars.proton_path}/proton run {_vars.program}")
	elif not custom:
		print("made it to 'not custom'")
		os.system(f'~/.steam/steam/steamapps/common/Proton\\ {_vars.proton}/proton run {program}')
	else:
		print(f'How did we get here?')
		sys.exit(_vars.err_val)


class ProtonCaller:
	def __init__(self, _argv1, custom, err_val, version, scdp, proton, program, proton_path, _help):
		self._argv1 = _argv1
		self.custom = custom
		self.err_val = err_val
		self.version = version
		self.scdp = scdp
		self.proton = proton
		self. program = program
		self.proton_path = proton_path
		self._help = _help


pc = ProtonCaller
if len(sys.argv[1]) > 4:
	_vars = pc(sys.argv[1], custom, err_val, version, scdp, proton, program, proton_path, _help)
	print(_vars)
	proton_call()
elif sys.argv[1] == 'help':
	err_val = 0
	_vars = pc(sys.argv[1], custom, err_val, version, scdp, proton, program, proton_path, _help)
	proton_call()
elif sys.argv[1] == '-h':
	err_val = 0
	_vars = pc(sys.argv[1], custom, err_val, version, scdp, proton, program, proton_path, _help)
	proton_call()
elif sys.argv[1] == '-c':
	_help = False
	custom = True
	proton_path = sys.argv[2]
	program = sys.argv[3]
	_vars = pc(sys.argv[1], custom, err_val, version, scdp, proton, sys.argv[3], sys.argv[2], _help)
	proton_call()
else:
	_help = False
	print('Passed first checks')
	if sys.argv[1] == '5':
		print(sys.argv[1])
		program = sys.argv[2]
		_vars = pc(sys.argv[1], custom, err_val, version, scdp, '5.0', sys.argv[2], proton_path, _help)
		proton_call()
	else:
		print(sys.argv[1])
		proton = sys.argv[1]
		program = sys.argv[2]
		_vars = pc(sys.argv[1], custom, err_val, version, scdp, sys.argv[1], sys.argv[2], proton_path, _help)
		proton_call()
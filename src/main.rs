use std::env;
use std::process::Command;

use env::VarError;

struct Proton {
    program: String,
    path: String,
    pass: String,
    extra: bool,
}

// logic
fn if_arg(the_arg: String) -> bool {
    let arg: Vec<char> = the_arg.chars().collect();
    if arg[0] == '-' {
        if arg[1] == 'c' {
            return true;
        }
        return false;
    }
    return true;
}

fn execute_proton(p: Proton) {
    println!("________Proton________");
    match p.extra {
        true => {
            let mut child = Command::new(p.path)
                .arg("run")
                .arg(p.program)
                .arg(p.pass)
                .spawn()
                .expect("failed to launch Proton");
            let ecode = child.wait()
                .expect("failed to wait for Proton");
            assert!(ecode.success());
        }
        false => {
            let mut child = Command::new(p.path)
                .arg("run")
                .arg(p.program)
                .spawn()
                .expect("failed to launch Proton");
            let ecode = child.wait()
                    .expect("failed to wait for Proton");
            assert!(ecode.success());
        }
    }
    println!("______________________\n");
    println!("Proton exited");
}

fn custom_mode(args: Vec<String>) -> Proton {
    println!("custom mode enabled");
    let mut _path: String = String::new();
    let p: Proton;

    _path = args[2].to_string();
    _path.push_str("/proton");

    if args.len() == 5 {
        p = Proton {
            program: args[3].to_string(),
            path: _path,
            pass: args[4].to_string(),
            extra: true,
        };
    } else {
        p = Proton {
            program: args[3].to_string(),
            path: _path,
            pass: String::new(),
            extra: false,
        }
    }
    return p;
}

fn normal_mode(args: Vec<String>) -> Result<Proton, VarError> {
    let version: String;
    let program;
    let mut _path: String = String::new();
    let common: String;
    let pr: Proton;

    debug_assert!(if_arg(args[1].to_string()), "invalid option");
    if if_arg(args[1].to_string()) == false {
        println!("Invalid option: '{}'", args[1]);
        println!("Try 'proton-call --help'");
        return Err(VarError::NotPresent);
    }

    match env::var("PC_COMMON") {
        Ok(val) => common = val,
        Err(_e) => {
            setup();
            return Err(VarError::NotPresent);
        }
    }

    version = args[1].to_string();
    program = args[2].to_string();
    _path = common;
    _path.push_str("/Proton ");
    _path.push_str(version.as_str());
    _path.push_str("/proton");
    println!("Proton:       {}", version);

    if args.len() == 4 {
        pr = Proton {
            program: program,
            path: _path,
            pass: args[3].to_string(),
            extra: true,
        };
    } else {
        pr = Proton {
        program: program,
        path: _path,
        pass: String::new(),
        extra: false,
        };
    }
    Ok(pr)
}

fn proton(args: Vec<String>) {
    let p: Proton;

    match args[1].as_str() {
        "--help" => {
            help();
            return;
        }
        "--version" => {
            pc_version();
            return;
        }
        "--setup" => {
            setup();
            return;
        }
        "-c" =>  p = custom_mode(args),
        "--custom" => p = custom_mode(args),
        _ => {
            match normal_mode(args) {
                Ok(val) => p = val,
                Err(_e) => {
                    println!("error running Proton");
                    return;
                },
            }
        }
    }

    println!("program:      {}\n", p.program);
    debug_assert!(std::path::Path::new(&p.program).exists(),
    "invalid program or directory");
    debug_assert!(std::path::Path::new(&p.path).exists(),
    "invalid version or directory");
    if std::path::Path::new(&p.program).exists() == false {
        println!("Program executable {} does not exist", p.program);
        return;
    }
    if std::path::Path::new(&p.path).exists() == false {
        println!("Invalid Proton version or path");
        return;
    }
    execute_proton(p);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => missing_args(),
        2 => {
            match args[1].as_str() {
                "--help" => help(),
                "--version" => pc_version(),
                "--setup" => setup(),
                "-c" => missing_args(),
                _ => {
                    debug_assert!(if_arg(args[1].to_string()), "invalid option");
                    if if_arg(args[1].to_string()) == false {
                        println!("Invalid option: '{}'", args[1]);
                        println!("Try 'proton-call --help'");
                    }
                    return;
                }
            }
        }
        3 => proton(args),
        4 => proton(args),
        _ => {
            println!("proton-call: too many arguments");
            println!("Try 'proton-call --help' for more information");
        }
    }
}

// messaging
fn help() {
    println!("Usage: proton-call VERSION PROGRAM");
    println!("   or: basename OPTION PATH PROGRAM");
    println!("Execute PROGRAM with Proton VERSION");
    println!("If specified, run proton PATH\n");
    println!("  -c, --custom PATH   use proton from PATH");
    println!("  --help              display this help message");
    println!("  --version           display version information\n");
}

fn pc_version() {
    println!("  proton-caller 2.0.1 Copyright (C) 2021  Avery Murray");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.")
}

fn missing_args() {
    println!("proton-call: missing arguments");
    println!("Try 'proton-call --help' for more information");
}

fn setup() {
    match env::var("STEAM_COMPAT_DATA_PATH") {
        Ok(val) => println!("STEAM_COMPAT_DATA_PATH =   {}", val),
        Err(_e) => {
            println!("'export STEAM_COMPAT_DATA_PATH=<path to common'");
        }
    }
    match env::var("PC_COMMON") {
        Ok(val) => println!("PC_COMMON  =   {}", val),
        Err(_e) => {
            println!("'export PC_COMMON=$HOME/proton'");
        }
    }
}

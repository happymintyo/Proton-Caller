use std::env;
use std::process::Command;

fn if_arg(the_arg: String) -> bool {
    let arg: Vec<char> = the_arg.chars().collect();
    if arg[0] == '-' {
        if arg[1] == 'c' {
            return false;
        }
        println!("proton-call: invalide option -- '{}'", arg[1]);
        println!("Try 'proton-call --help' for more information");
        return true;
    }
    return false;
}

fn help() {
    println!("Usage: proton-all VERSION PROGRAM");
    println!("   or: basename OPTION PATH PROGRAM");
    println!("Execute PROGRAM with Proton VERSION");
    println!("If specified, run proton PATH\n");
    println!("  -c, --custom PATH   use proton from PATH");
    println!("  --help              display this help message");
    println!("  --version           display version information\n");
    pc_version();
}

fn pc_version() {
    println!("  proton-caller 2.0 Copyright (C) 2021  Avery Murray");
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
            println!("'export STEAM_COMPAT_DATA_PATH=<path to steamapps/common>'");
        }
    }
    match env::var("PC_COMMON") {
        Ok(val) => println!("PC_COMMON  =   {}", val),
        Err(_e) => println!("'export PC_COMMON=$HOME/proton'"),
    }
}

fn execute_proton(path: String, program: String) {
    println!("________Proton________");

    let mut child = Command::new(path)
        .arg("run")
        .arg(program)
        .spawn()
        .expect("failed to launch Proton");
    
    let ecode = child.wait()
        .expect("failed to wait for Proton");
    
    assert!(ecode.success());

    println!("______________________\n");
    println!("Proton exited");
}

fn proton(args: Vec<String>) {
    let version: String;
    let mut _program: String = String::new();
    let mut _path: String = String::new();
    let common: String;
    // let _argus: Vec<String>; setup for TODO #2

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
        "-c" => {
            println!("Custom mode enabled");
            _path = args[2].to_string();
            _path.push_str("/proton");
            _program = args[3].to_string();
        }
        "--custom" => {
            println!("Custom mode enabled");
            _path = args[2].to_string();
            _path.push_str("/proton");
            _program = args[3].to_string();

        }
        _ => {
            if if_arg(args[1].to_string()) == true {
                return;
            }

            match env::var("PC_COMMON") {
                Ok(val) => common = val,
                Err(_e) => {
                    println!("PC_COMMON variable does not exist");
                    setup();
                    return;
                }
            }

            version = args[1].to_string();
            _program = args[2].to_string();
            _path = common;
            _path.push_str("/Proton ");
            _path.push_str(version.as_str());
            _path.push_str("/proton");
            println!("Proton:       {}", version);
        }
    }

    println!("program:      {}\n", _program);
    execute_proton(_path, _program);
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
                _ => if if_arg(args[1].to_string()) == true {
                    return;
                },
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

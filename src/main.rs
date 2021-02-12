use std::env;
use std::process::Command;
extern crate shell_words;

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

    _path = args[2].to_string();
    _path.push_str("/proton");

    if args.len() > 3 {
        let p = Proton {
            program: args[3].to_string(),
            path: _path,
            pass: args[4].to_string(),
            extra: true,
        };
        return p;
    }

    let p = Proton {
        program: args[3].to_string(),
        path: _path,
        pass: String::new(),
        extra: false,
    };
    return p;
}

fn normal_mode(args: Vec<String>) -> Proton {
    let version: String;
    let program;
    let mut _path: String = String::new();
    let common: String;
    let _p: Proton;

    assert!(if_arg(args[1].to_string()), "invalid option");

    match env::var("PC_COMMON") {
        Ok(val) => common = val,
        _ => {
            setup();
            std::process::exit(1);
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
        let pr = Proton {
            program: program,
            path: _path,
            pass: args[3].to_string(),
            extra: true,
        };
        return pr;
    }

    let pr = Proton {
        program: program,
        path: _path,
        pass: String::new(),
        extra: false,
    };
    return pr;
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
        _ => p = normal_mode(args),
    }

    println!("program:      {}\n", p.program);
    assert!(std::path::Path::new(&p.program).exists(),
    "invalid program or directory");
    assert!(std::path::Path::new(&p.path).exists(),
    "invalid version or directory");
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
                _ => assert!(if_arg(args[1].to_string()), "invalid option"),
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
            println!("'export STEAM_COMPAT_DATA_PATH=<path to common'");
        }
    }
    match env::var("PC_COMMON") {
        Ok(val) => println!("PC_COMMON  =   {}", val),
        Err(_e) => println!("'export PC_COMMON=$HOME/proton'"),
    }
}

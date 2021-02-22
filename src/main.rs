use std::env;
mod proton;

// logic
fn if_arg(the_arg: String) -> Option<()> {
    let arg: Vec<char> = the_arg.chars().collect();
    if arg[0] == '-' {
        if arg[1] == 'c' {
            return Some(());
        }
        println!("proton-call: invalid argument: '{}'", the_arg);
        println!("Try 'proton-call --help");
        return None;
    }
    Some(())
}

/*/
fn execute_proton(p: Proton) {
    println!("{:?}", p.arguments);
    println!("________Proton________");

    let mut child = Command::new(p.path)
        .args(p.arguments)
        .spawn()
        .expect("Failed to launch Proton");
    let ecode = child.wait().unwrap();
    if !ecode.success() {
        eprintln!("Proton exited with error")
    }
    println!("______________________\n");
    println!("Proton exited");
}

fn custom_mode(args: Vec<String>) -> Proton {
    println!("custom mode enabled");
    let mut _path: String = String::new();

    _path = args[2].to_string();
    _path.push_str("/proton");

    let len = args.len();
    let mut pr_ar = vec![std::string::String::new(); len-2];
    pr_ar[0] = std::string::String::from("run");
    pr_ar[1] = args[3].to_string();
    for i in 4..args.len() {
        pr_ar[i-2] = args[i].to_string();
    }

    Proton {program: args[3].to_string(), path: _path, arguments: pr_ar,}
}

fn normal_mode(args: Vec<String>) -> Option<Proton> {
    let version: String;
    let program;
    let mut _path: String = String::new();
    let common: String;

    match env::var("PC_COMMON") {
        Ok(val) => common = val,
        Err(_e) => {
            setup();
            return None;
        }
    }

    version = args[1].to_string();
    program = args[2].to_string();
    _path = common;
    _path.push_str("/Proton ");
    _path.push_str(version.as_str());
    _path.push_str("/proton");
    println!("Proton:       {}", version);

    let len = args.len();
    let mut pr_ar = vec![std::string::String::new(); len-1];
    pr_ar[0] = std::string::String::from("run");
    pr_ar[1] = program.to_string();
    for i in 3..args.len() {
        pr_ar[i-1] = args[i].to_string();
    }

    Some (Proton {
        program: program,
        path: _path,
        arguments: pr_ar,})
}
*/

fn proton(args: Vec<String>) {
    let proton: proton::Proton;
    let custom: bool;

    match args[1].as_str() {
        "--custom" => custom = true,
        "-c" => custom = true,
        _ => {
            if if_arg(args[1].to_string()) == None {return}
            custom = false}
    }

    

    match custom {
        true => {
            match proton::Proton::custom_mode(&args) {
                Ok(s) => proton = s,
                Err(e) => {
                    eprintln!("proton-call: {}", e);
                    return;
                }
            }
        }
        false => {
            match proton::Proton::normal_mode(&args) {
                Ok(s) => proton = s,
                Err(e) => {
                    eprintln!("proton-call: {}", e);
                    return;
                }
            }
        }
    }

    match proton.execute() {
        Ok(_) => (),
        Err(e) => {
            println!("proton-call: {}", e);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    if len == 1 {missing_args(); return}

    match args[1].as_str() {
        "--help" => {help(); return}
        "--version" => {pc_version(); return}
        "--setup" => {setup(); return}
        _ => (),
    }

    proton(args);
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
    println!("  proton-caller 2.1.0 Copyright (C) 2021  Avery Murray");
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
            println!("'export STEAM_COMPAT_DATA_PATH=<path to common>'");
        }
    }
    match env::var("PC_COMMON") {
        Ok(val) => println!("PC_COMMON  =   {}", val),
        Err(_e) => {
            println!("'export PC_COMMON=$HOME/proton'");
        }
    }
}

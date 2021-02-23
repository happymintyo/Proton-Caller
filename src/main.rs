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

    if custom {
        match proton::Proton::custom_mode(&args) {
            Ok(s) => proton = s,
            Err(e) => {
                eprintln!("proton-call: {}", e);
                return;
            }
        }
    } else {
        match proton::Proton::normal_mode(&args) {
            Ok(s) => proton = s,
            Err(e) => {
                eprintln!("proton-call: {}", e);
                return;
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

    if args.len() == 1 {missing_args(); return}

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
    println!("  proton-caller 2.2.0 Copyright (C) 2021  Avery Murray");
    println!("This program comes with ABSOLUTELY NO WARRANTY.");
    println!("This is free software, and you are welcome to redistribute it");
    println!("under certain conditions.")
}

fn missing_args() {
    println!("proton-call: missing arguments");
    println!("Try 'proton-call --help' for more information");
}

fn setup() {
    println!("Configuration of proton-call requires a config file located at");
    println!("`~/.config/proton.conf` which is formatted like:\n");
    println!("  data = \"\"");
    println!("  common = \"\"");
    println!("`data` is used to give proton a directory used for compatibility data.\n");
    println!("`common` is a directory pointing to steam's common directory, where Proton");
    println!("and games are installed");
}

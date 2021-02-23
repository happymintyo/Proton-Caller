#![warn(clippy::all, clippy::pedantic)]
pub(crate) struct Proton {
    program: String,
    version: String,
    proton: String,
    arguments: Vec<String>,
    conf: Config,
}

fn push_proton(string: &mut String, version: &str) {
    string.push_str("/Proton ");
    string.push_str(version);
    string.push_str("/proton");
}

impl Proton {
    pub fn init(args: &[String], custom: bool) -> Result<Proton, &str> {
        if custom {return Proton::init_custom(args);}

        let config: Config;
        let version: String = args[1].to_string();
        let program: String = args[2].to_string();
        let mut path: String;

        match Config::new() {
            Ok(val) => config = val,
            Err(e) => return Err(e),
        }

        path = config.common.to_string();
        push_proton(&mut path, &version);

        if !Proton::check(&[&path, &args[2]]) {
            return Err("error: invalid Proton or executable");
        }

        let a: Vec<String> = Proton::arguments(3, args.len(), &args, &program);

        Ok(Proton {
                program,
                version,
                proton: path,
                arguments: a,
                conf: config,
        })
    }

    fn arguments(start: usize, end: usize, args: &[String], program: &str) -> Vec<String> {
        let mut vector: Vec<String> = vec![std::string::String::new(); end-(start-2)];
        vector[0] = std::string::String::from("run");
        vector[1] = program.to_string();

        for i in start..end {
            vector[i-(start-2)] = args[i].to_string();
        }

        vector
    }

    fn check(file: &[&String]) -> bool {
        for i in file {
            if !std::path::Path::new(i).exists() {
                return false;
            }
        }
        true
    }

    fn init_custom(args: &[String]) -> Result<Proton, &str> {
        let config: Config;
        let mut path: String = args[2].to_string();
        path.push_str("/proton");

        if let false = Proton::check(&[&path, &args[3]]) {
            return Err("error: invalid Proton or executable");
        }

        let a: Vec<String> = Proton::arguments(4, args.len(), &args, &args[3].to_string());

        match Config::new() {
            Ok(val) => config = val,
            Err(e) => return Err(e),
        }

        Ok(Proton {
            program: args[3].to_string(),
            version: std::string::String::from("custom"),
            proton: path,
            arguments: a,
            conf: config,
        })
    }

    pub fn execute(self) -> Result<(), &'static str> {
        println!("{:?}", self.arguments);
        println!("Proton:   {}", self.version);
        println!("Program:  {}\n", self.program.split('/').last().unwrap());
        let ecode: std::process::ExitStatus;
        println!("________Proton________");

        let child= std::process::Command::new(self.proton)
            .args(self.arguments)
            .env("STEAM_COMPAT_DATA_PATH", self.conf.data)
            .spawn();
        if child.is_err() {
            return Err("failed to launch Proton")
        }
        let mut out = child.unwrap();
        match out.wait() {
            Ok(val) => ecode = val,
            Err(_) => return Err("error: failed to wait for Proton"),
        }
        if !ecode.success() {
            return Err("error: Proton exited with an error")
        }
        println!("______________________\n");
        Ok(())
    }
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub(crate) struct Config {
    data: String,
    common: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let config: Config;
        let mut file: String;

        file = std::env::var("HOME").unwrap();

        file.push_str("/.config/proton.conf");

        if !std::path::Path::new(&file).exists() {
            return Err("error: proton.conf does not exist");
        }

        let strr: String;

        match std::fs::read_to_string(file) {
            Ok(string) => strr = string,
            Err(_) => return Err("error: failed to read config"),
        }

        match toml::from_str(strr.as_str()) {
            Ok(o) => config = o,
            Err(_) => return Err("error: failed to read config"),
        }

        Ok(config)
    }
}

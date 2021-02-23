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

fn check(file: &[&String]) -> bool {
    let len = file.len();
    for i in 0..len {
        if !std::path::Path::new(file[i]).exists() {
            return false;
        }
    }
    true
}

impl Proton {
    pub fn init(args: &[String], custom: bool) -> Result<Proton, &str> {
        if custom {
            return Proton::init_custom(args);
        }
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

        if let false = check(&[&path, &args[2]]) {
            return Err("error: invalid Proton or executable");
        }

        let len: usize = args.len();
        let mut a: Vec<String> = vec![std::string::String::new(); len-1];
        a[0] = std::string::String::from("run");
        a[1] = program.to_string();

        for i in 3..len {
            a[i-1] = args[i].to_string();
        }

        Ok(Proton {
                program,
                version,
                proton: path,
                arguments: a,
                conf: config,
        })
    }

    fn init_custom(args: &[String]) -> Result<Proton, &str> {
        let len: usize = args.len();
        let config: Config;
        let mut path: String = args[2].to_string();
        path.push_str("/proton");

        if let false = check(&[&path, &args[3]]) {
            return Err("error: invalid Proton or executable");
        }

        let mut a: Vec<String> = vec![std::string::String::new(); len-2];
        a[0] = std::string::String::from("run");
        a[1] = args[3].to_string();

        for i in 4..len {
            a[i-2] = args[i].to_string();
        }

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

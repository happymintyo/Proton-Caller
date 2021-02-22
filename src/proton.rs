pub(crate) struct Proton {
    program: String,
    version: String,
    proton: String,
    arguments: Vec<String>,
}

impl Proton {
    pub fn normal_mode(args: &[String]) -> Result<Proton, &str> {
        let version: String = args[1].to_string();
        let program: String = args[2].to_string();
        let mut path: String;
        let common: String;

        match std::env::var("PC_COMMON") {
            Ok(val) => common = val,
            Err(_) => return Err("error: 'PC_COMMON' does not exist"),
        }

        path = common;
        path.push_str("/Proton ");
        path.push_str(version.as_str());
        path.push_str("/proton");

        if !std::path::Path::new(&path).exists() {
            return Err("error: invalid path");
        }

        if !std::path::Path::new(&args[2]).exists() {
            return Err("error: invalid program");
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
                arguments: a,})
    }

    pub fn custom_mode(args: &[String]) -> Result<Proton, &str> {
        let len: usize = args.len();
        let mut path: String = args[2].to_string();
        path.push_str("/proton");

        if !std::path::Path::new(&path).exists() {
            return Err("error: invalid path");
        }

        if !std::path::Path::new(&args[2]).exists() {
            return Err("error: invalid program");
        }

        let mut a: Vec<String> = vec![std::string::String::new(); len-2];
        a[0] = std::string::String::from("run");
        a[1] = args[3].to_string();

        for i in 4..len {
            a[i-2] = args[i].to_string();
        }

        Ok(Proton {
            program: args[3].to_string(),
            version: std::string::String::from("custom"),
            proton: path,
            arguments: a,
        })
    }

    pub fn execute(self) -> Result<(), &'static str> {
        println!("Proton:   {}", self.version);
        println!("Program:  {}\n", self.program.split('/').last().unwrap());
        let ecode: std::process::ExitStatus;
        println!("________Proton________");

        let child= std::process::Command::new(self.proton)
            .args(self.arguments)
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

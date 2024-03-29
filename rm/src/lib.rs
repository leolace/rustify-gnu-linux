use std::fs;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
enum Modes {
    Standard,
    RecursivelyAndForce,
    OnlyRecursively,
    OnlyForce,
}

fn get_mode(config: &Config) -> Modes {
    if config.recursively && config.force {
        Modes::RecursivelyAndForce
    } else if config.force {
        Modes::OnlyForce
    } else if config.recursively {
        Modes::OnlyRecursively
    } else {
        Modes::Standard
    }
}

pub fn run(config: Config) -> Result<(), String> {
    let mode = get_mode(&config);

    match mode {
        Modes::RecursivelyAndForce => Functions::recursively_and_force(config.directory),
        Modes::OnlyForce => Functions::force(config.directory),
        Modes::OnlyRecursively => Functions::recursively(config.directory)?,
        Modes::Standard => Functions::standard(config.directory)?,
    };

    Ok(())
}

struct Functions;

impl Functions {
    fn recursively_and_force(path: String) {
        let is_directory = fs::metadata(&path).unwrap().is_dir();

        if is_directory {
            let _ = fs::remove_dir_all(path).is_ok();
        } else {
            let _ = fs::remove_file(path).is_ok();
        }
    }

    fn force(path: String) {
        let is_directory = fs::metadata(&path).unwrap().is_dir();

        if is_directory {
            let _ = fs::remove_dir(path).is_ok();
        } else {
            let _ = fs::remove_file(path).is_ok();
        }
    }

    fn recursively(path: String) -> Result<(), String> {
        let is_directory = fs::metadata(&path).unwrap().is_dir();
        let auth = auth_operation(&path);

        if !auth {
            return Err(String::from("Operation canceled"));
        }

        if is_directory {
            if let Err(e) = fs::remove_dir_all(path) {
                return Err(e.kind().to_string());
            }
            Ok(())
        } else {
            if let Err(e) = fs::remove_file(path) {
                return Err(e.kind().to_string());
            }
            Ok(())
        }
    }

    fn standard(path: String) -> Result<(), String> {
        let is_directory = fs::metadata(&path).unwrap().is_dir();
        let auth = auth_operation(&path);

        if !auth {
            return Err(String::from("Operation canceled"));
        }

        if is_directory {
            if let Err(e) = fs::remove_dir(path) {
                return Err(e.kind().to_string());
            }
            Ok(())
        } else {
            if let Err(e) = fs::remove_file(path) {
                return Err(e.kind().to_string());
            }
            Ok(())
        }
    }
}

#[derive(Default, Debug)]
pub struct Config {
    pub directory: String,
    pub recursively: bool,
    pub force: bool,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, String> {
        let mut configs = Config::default();
        let mut flags: Vec<&str> = Vec::new();

        if args.is_empty() {
            return Err(String::from("Invalid args length"));
        };

        let full_args = args.join(" ");
        let chars = full_args.split("").skip(1).collect::<Vec<&str>>();

        let mut index = 0;
        while index < chars.len() {
            if chars[index] == "-" {
                for j in index..chars.len() {
                    if chars[j] == " " {
                        break;
                    }
                    index += 1;
                    flags.push(chars[j]);
                }
                continue;
            }

            if configs.directory.is_empty() {
                for j in index..chars.len() {
                    if chars[j] == " " {
                        break;
                    }
                    configs.directory += chars[j];
                }
            }

            index += 1;
        }

        for flag in flags.iter() {
            match *flag {
                "r" => configs.recursively = true,
                "f" => configs.force = true,
                _ => (),
            }
        }

        if configs.directory.is_empty() {
            return Err(String::from("Directory missing"));
        }

        Ok(configs)
    }
}

fn auth_operation(path: &String) -> bool {
    const LOWERCASE_Y_AS_BYTES: &[u8] = &[121, 10];
    const UPPERCASE_Y_AS_BYTES: &[u8] = &[89, 10];

    print!("remove directory: '{}'? (Y/n): ", path);
    stdout().flush().unwrap();

    let mut auth = String::new();
    stdin().read_line(&mut auth).unwrap();

    matches!(auth.as_bytes(), LOWERCASE_Y_AS_BYTES | UPPERCASE_Y_AS_BYTES)
}

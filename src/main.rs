use std::{env, fmt};

struct Arguments {
    file_path: String,
    name: String,
    email: String,
    single_folder: bool,
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.file_path, self.name, self.email, self.single_folder
        )
    }
}

fn get_arguments() -> Arguments {
    let args: env::Args = env::args();

    if args.len() == 1 {
        return Arguments {
            file_path: String::from("."),
            name: String::from(""),
            email: String::from(""),
            single_folder: false,
        };
    }

    let mut file_path = String::from(".");
    let mut name = String::from("");
    let mut email = String::from("");
    let mut single_folder = false;

    for arg in args.skip(1) {
        let parts: Vec<&str> = arg.split("=").collect();

        if parts.len() == 1 {
            panic!("Invalid arugment provided.\nUsage: ./gitstats arg1=value1 arg2=value2\nArguments:\nfile_path - Which folder to scan.\nname - Git name to count commits for.\nemail - Git email to count commits for.\nsingle_folder - 'true' to only scan this folder. 'false' to include subfolders.");
        }

        let arg_value: String;

        if parts.len() > 2 {
            arg_value = parts[1..parts.len()].join("=");
        } else {
            arg_value = parts[1].to_string();
        }

        match parts[0] {
            "name" => name = arg_value,
            "email" => email = arg_value,
            "file_path" => file_path = arg_value,
            "single_folder" => single_folder = arg_value.parse::<bool>().unwrap_or_default(),
            _ => (),
        }
    }

    Arguments {
        file_path,
        name,
        email,
        single_folder,
    }
}

fn main() {
    let args: Arguments = get_arguments();

    println!("{}", args);
}

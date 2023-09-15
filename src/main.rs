use std::{
    env, fmt,
    path::PathBuf,
    process::{exit, Command},
};

use git2::{Error, ObjectType, Repository};

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

struct RepositoryInfo {
    path: PathBuf,
    total_commits: i64,
    user_commits: i64,
    contribution_percentage: f64,
}

impl fmt::Display for RepositoryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Repository: {:?}. Total commits: {}.\nYour contribution: {:.2}%, {} commits.",
            self.path.parent().unwrap().file_name().unwrap(),
            self.total_commits,
            self.contribution_percentage * 100.0,
            self.user_commits
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

fn get_git_config_property(property: &str) -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg(property)
        .output()
        .expect("Failed to get config value from git command. Make sure user.name and user.email are set or supply them as arguments with name=<name> and email=<email>.");

    if !output.status.success() {
        panic!("Failed to get config value from git command. Make sure user.name and user.email are set or supply them as arguments with name=<name> and email=<email>.");
    }

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn find_repositories(path: PathBuf) -> Vec<PathBuf> {
    let mut repositories = Vec::new();

    for entry in path.read_dir().expect("Failed to read file path") {
        if let Ok(entry) = entry {
            let entry_path = entry.path();

            if !entry_path.is_dir() {
                continue;
            }

            if entry_path.file_name().unwrap() == ".git" {
                repositories.push(entry_path);
            } else {
                repositories.extend(find_repositories(entry_path));
            }
        }
    }

    repositories
}

fn analyze_repository(
    path: PathBuf,
    match_name: &String,
    match_email: &String,
) -> Result<RepositoryInfo, Error> {
    let repo = Repository::open(&path)?;
    let odb = repo.odb()?;

    let mut total_commits: i64 = 0;
    let mut user_commits: i64 = 0;

    let user_commit_prefix = format!("committer {} <{}>", match_name, match_email);

    odb.foreach(|oid| {
        let obj = odb.read(*oid).unwrap();

        if obj.kind() == ObjectType::Commit {
            let obj_data = String::from_utf8_lossy(&obj.data());
            let committer = obj_data
                .split("\n")
                .skip_while(|line| !line.starts_with("committer"))
                .next()
                .unwrap_or_default();

            total_commits += 1;

            if committer.starts_with(&user_commit_prefix) {
                user_commits += 1;
            }
        }

        true
    })?;

    Ok(RepositoryInfo {
        path: path.canonicalize().unwrap_or(path),
        total_commits,
        user_commits,
        contribution_percentage: user_commits as f64 / total_commits as f64,
    })
}

fn main() {
    let mut args: Arguments = get_arguments();

    if args.email == "" {
        args.email = get_git_config_property("user.email");
    }

    if args.name == "" {
        args.name = get_git_config_property("user.name")
    }

    let current_dir = env::current_dir().unwrap();
    let repositories: Vec<PathBuf> = find_repositories(current_dir.join(&args.file_path));

    if repositories.len() == 0 {
        println!("No git repository was found.");
        exit(0);
    }

    for repository in repositories {
        let info = analyze_repository(repository, &args.name, &args.email);

        match info {
            Ok(info) => println!("{}", info),
            Err(err) => {
                println!("There was an error when analyzing repository");
                println!("Error: {}", err);
            }
        }
    }
}

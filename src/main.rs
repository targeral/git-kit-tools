use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use std::str;


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(version = "0.1", author = "targeral")]
    ClearFreeBranch,
}

fn get_first_word_in_output(output: &str) -> Vec<&str> {
    let mut words = Vec::new();
    for line in output.lines() {
        let first_word = line.split_whitespace().next();
        match first_word {
            Some(value) => words.push(value),
            None => {}
        }
    } 

    words
}

fn clear_free_branch() {
    Command::new("git")
        .arg("fetch")
        .arg("--prune")
        .output()
        .expect("Failed to fetch");

    let git_branches_output = Command::new("git")
        .arg("branch")
        .arg("-vv")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to get branch information")
        .stdout
        .expect("Failed to capture git command`s stdout");

    let grep_output = Command::new("grep")
        .arg("origin/.*: gone]")
        .stdin(git_branches_output)
        .output()
        .expect("Failed to spawn grep process");

    let free_branches_log = str::from_utf8(&grep_output.stdout).expect("Output was not UTF-8");

    let branches = get_first_word_in_output(free_branches_log);

    for branch in branches {
        println!("delete branch {}", branch);
        Command::new("git")
            .arg("branch")
            .arg("-d")
            .arg(branch)
            .output()
            .expect("Failed to execute command");
        
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ClearFreeBranch => {
            println!("Clear remotely deleted branches in local");
            clear_free_branch()
        }
    }
    
}



use std::error::Error;

use clap::{Parser, Subcommand};
use rmd;

#[derive(Parser)]
#[clap(author = "Todd Moneypenny", version = "0.1.0")]
#[clap(about="Document what works", long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Group commands together
    #[clap(short, long)]
    group: bool,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Document last N commands
    #[clap(parse(try_from_str))]
    Last { lines: Option<usize> },
    /// Set a marker to save the last command
    S { save: Option<usize> },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let group = match cli.group {
        true => true,
        false => false,
    };

    let hist_file =
        std::env::var_os("HISTFILE").unwrap_or_else(|| std::ffi::OsString::from(".zsh_history"));
    #[allow(deprecated)]
    let home_dir = std::env::home_dir().expect("could not determine a home directory");
    let path = home_dir.join(&hist_file);

    match &cli.command {
        Commands::Last { lines } => {
            println!("Documenting last {:?} commands", &lines.unwrap_or(10));
            match lines {
                Some(l) => rmd::run(*l, path, group)?,
                None => rmd::run(10, path, group)?,
            }
        }
        Commands::S { save: _ } => (),
    }
    Ok(())
}

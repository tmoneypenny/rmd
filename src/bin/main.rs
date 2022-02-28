use std::error::Error;

use clap::{Parser, Subcommand};

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
    /// Document last N markers
    #[clap(parse(try_from_str))]
    Last { lines: Option<usize> },
    /// Set a marker to save the last N commands
    S { save: Option<usize> },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let group = match cli.group {
        true => true,
        false => false,
    };

    let cmd_name = std::env::args().next().unwrap();
    let hist_file =
        std::env::var_os("HISTFILE").unwrap_or_else(|| std::ffi::OsString::from(".zsh_history"));
    #[allow(deprecated)]
    let home_dir = std::env::home_dir().expect("could not determine a home directory");
    let path = home_dir.join(&hist_file);

    match &cli.command {
        Commands::Last { lines } => match lines {
            Some(l) => rmd::run(cmd_name, *l, path, group)?,
            None => rmd::run(cmd_name, 5, path, group)?,
        },
        Commands::S { save: _ } => (),
    }
    Ok(())
}

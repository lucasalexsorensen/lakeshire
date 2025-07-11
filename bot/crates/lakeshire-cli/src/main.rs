pub mod commands;
use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::{bench_command, face_command, parse_command, snap_command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Bench {
        #[arg(short = 'n', long, default_value_t = 100)]
        count: u32,
    },
    Snap {
        output: Option<String>,
    },
    Parse,
    Face,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Bench { count } => bench_command(count),
        Command::Parse => parse_command(),
        Command::Snap { output } => snap_command(output),
        Command::Face => face_command(),
    }
}

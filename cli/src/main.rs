use clap::Parser;
use z2l_cli::{run_quick, Command, Z2LCli};

fn main() {
    let cli = Z2LCli::parse();

    match cli.command {
        Command::RunQuick(args) => run_quick::execute(args),
    }
}

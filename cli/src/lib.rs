//! Z2L Command Line Interface (CLI) & Terminal User Interface (TUI).
//!
//! This is the main entry point to Z2L. The CLI defines the available modes of operation for Z2L,
//! and the required command-line arguments to execute each one. The TUI is an interactive user
//! interface which displays the current state of the processor and allows the user to control it.

pub mod run_quick;
pub mod tui;

use clap::{Parser, Subcommand};
use run_quick::RunQuickArgs;

/// Z2L: A RISC-V emulator.
#[derive(Clone, Debug, Hash, Parser)]
#[command(name = "Z2L")]
pub struct Z2LCli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Clone, Debug, Hash, Subcommand)]
pub enum Command {
    /// Run a single RISC-V binary in a reasonable default configuration.
    ///
    /// The run-quick command is used to quickly run a RISC-V binary, with little-to-no
    /// configuration. You just specify a ROM, and Z2L will run this in an emulated RISC-V system,
    /// like so: `z2l run-quick my_rom.bin`
    ///
    /// The ROM will be loaded at address `0x00000000` of the address space, and execution will also
    /// start at this point. By default, 32KiB of RAM will be accessible from address `0x80000000`,
    /// but the size of this RAM is customisable.
    RunQuick(RunQuickArgs),
}

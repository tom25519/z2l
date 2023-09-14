//! `z2l run-quick`: Run a single RISC-V binary in a reasonable default configuration.

use crate::tui;
use bus::{Bus, BusReader};
use clap::Args;
use cursive::CursiveExt;
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;
use z2l_core::clock::{Clock, FixedClock, FreeClock, ManualClock};
use z2l_core::{Config, ControlMessage, ExecutionEnvironment};
use z2l_isa::rv32i::RV32I;

/// Arguments for the `run-quick` command.
#[derive(Args, Clone, Debug, Hash)]
pub struct RunQuickArgs {
    /// Path to RISC-V binary to execute.
    rom: PathBuf,

    /// Amount of memory to allocate for RAM.
    ///
    /// This is normally interpreted as a number of bytes, but you can add a specifier ("K", "M", or
    /// "G") to indicate a different unit.
    ///
    /// The RAM is accessible from memory address 0x80000000.
    #[arg(short, long, default_value_t = String::from("32K"))]
    memory: String,

    /// Clock to use.
    ///
    /// By default, the processor is advanced manually by pressing the Enter key. Alternatively, the
    /// processor can be run at a fixed clock rate, by specifying this value as the frequency of the
    /// clock in HZ; or run as fast as possible, by specifying this value as "free".
    #[arg(short, long, default_value_t = String::from("manual"))]
    clock: String,
}

/// Parse memory size.
///
/// This allows use of the suffixes "K", "M", or "G" to specify a value is in KiB, MiB, or GiB
/// respectively, rather than just bytes.
pub fn parse_memory(memory: &str) -> usize {
    let shift = match memory.chars().last().expect("Invalid memory specification") {
        'K' | 'k' => 10,
        'M' | 'm' => 20,
        'G' | 'g' => 30,
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => 0,
        _ => panic!("Invalid memory specification"),
    };

    let value: usize = if shift == 0 {
        memory.parse()
    } else {
        let mut memory = memory.chars();
        memory.next_back();
        memory.as_str().parse()
    }
    .expect("Invalid memory specification");

    value << shift
}

/// Parse a clock selection.
///
/// The user may specify "manual", "free", or a number of Hz for a fixed clock.
pub fn parse_clock(clock: &str, control_rx: BusReader<ControlMessage>) -> Box<dyn Clock> {
    if clock == "manual" {
        Box::new(ManualClock::new(control_rx))
    } else if clock == "free" {
        Box::new(FreeClock::new())
    } else {
        let freq: u128 = clock.parse().expect("Invalid clock specification");
        let period = 1_000_000_000u128 / freq;
        Box::new(FixedClock::new(Duration::from_nanos(period as u64)))
    }
}

/// Create the [`ExecutionEnvironment`] to run the ROM.
pub fn create_execution_env(
    args: &RunQuickArgs,
    control_bus: &mut Bus<ControlMessage>,
) -> ExecutionEnvironment<Box<dyn Clock>> {
    let rom = File::open(&args.rom).expect("Failed to open ROM file");
    let ram_size = parse_memory(&args.memory);
    let clock = parse_clock(&args.clock, control_bus.add_rx());

    let config = Config {
        harts: 1,
        extensions: vec![Box::new(RV32I)],
        rom,
        ram_size,
        clock,
        control_rx: control_bus.add_rx(),
    };

    ExecutionEnvironment::new(config).unwrap()
}

/// Execute the `run-quick` command.
pub fn execute(args: RunQuickArgs) {
    let mut control_bus = bus::Bus::new(0xffff);

    let mut env = create_execution_env(&args, &mut control_bus);
    let log_rx = env.add_rx();

    let env_handle = std::thread::spawn(move || {
        env.run();
    });
    let tui_handle = std::thread::spawn(move || {
        let mut tui = tui::create(control_bus, log_rx);
        tui.run();
    });

    tui_handle.join().unwrap();
    env_handle.join().unwrap();
}

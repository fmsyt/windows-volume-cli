mod mods;
mod device;

use clap::Parser;
use device::print_devices;
use mods::Cli;

fn main() {
    let _args = Cli::parse();
    futures::executor::block_on(print_devices());
}

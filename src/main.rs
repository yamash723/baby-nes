#[macro_use]
extern crate anyhow;
use nes::Nes;

pub mod cli;
pub mod nes;

fn main() {
    let mut nes = Nes::new("rom/hello_world.nes").unwrap();
    nes.run();
}

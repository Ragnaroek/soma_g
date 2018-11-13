extern crate clap;
extern crate somag;

use clap::{Arg, App};
use std::fs;

use somag::z80;

fn main() {
    let matches = App::new("soma_g")
                    .version("0.0.1")
                    .about("gameboy emulator")
                    .arg(Arg::with_name("ROMFILE")
                               .help("ROM file input")
                               .required(true)
                               .index(1))
                .get_matches();

    let rom_file = matches.value_of("ROMFILE").unwrap();
    let rom = fs::read(rom_file).unwrap();

    let term = z80::start(rom, 0x100);
    println!("terminated: {:?}", term);
}

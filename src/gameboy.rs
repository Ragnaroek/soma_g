use super::z80;

pub fn gameboy_init(mem: Vec<u8>) -> z80::State {
    z80::initial_state(mem, 127, 0x100)
}

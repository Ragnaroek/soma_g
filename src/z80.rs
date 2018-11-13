
use std::collections::HashMap;
use std::fmt;


pub fn start(mem: Vec<u8>, start_addr: usize) -> Result<(), String> {

    let mut state = initial_state(mem, start_addr as u16);
    let instr_set = instruction_set();

    //TODO model state as a struct with register
    //TODO add function over state to instr to code effect
    loop {
        let op = state.mem[state.register.pc as usize];
        let instr_lookup = instr_set.get(op);
        if instr_lookup.is_none() {
            return Err(format!("not an instruction: 0x{:x}", op));
        }
        let instr = instr_lookup.unwrap();
        (instr.effect)(&mut state);
        //TODO: modify PC, Registers, Memory accordingly
        println!("instr = {:?}", instr);
        state.register.pc = state.register.pc + 1;
    }
}

struct State {
    mem: Vec<u8>,
    register: Register,
}

struct Register {
    pc: u16,
    a: u8,
    b: u8,
    c: u8,
    l: u8,
}

fn initial_state(mem: Vec<u8>, start_pc: u16) -> State {
    return State{mem, register:
        Register{pc: start_pc,
                 a: 0,
                 b: 0,
                 c: 0,
                 l: 0}};
}

struct Instr {
    mnemonic: String,
    op: u8,
    effect: fn(&mut State)
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Instr {{ mnemonic: {}, op: {} }}", self.mnemonic, self.op)
    }
}

struct InstrSet {
    set: HashMap<u8, Instr>,
}

impl InstrSet {

    fn new() -> InstrSet {
        let set = HashMap::new();
        return InstrSet{set};
    }

    fn get<'a>(&'a self, op: u8) -> Option<&'a Instr> {
        return self.set.get(&op);
    }

    fn add_instr(&mut self, op: u8, mnemonic: &str, effect: fn(&mut State)) {
        let instr = Instr{op, mnemonic: mnemonic.to_string(), effect};
        self.set.insert(op, instr);
    }
}

fn instruction_set() -> InstrSet {
    let mut instr_set = InstrSet::new();
    instr_set.add_instr(0x00, "NOP", nop);
    instr_set.add_instr(0xC3, "JP", jp);
    instr_set.add_instr(0x0C, "INC A", inc_a);
    instr_set.add_instr(0x2C, "INC L", inc_l);
    instr_set.add_instr(0x02, "LD (BC),A", ld_bc_a);
    return instr_set;
}

pub fn u16_le(pc: u16, mem: &[u8]) -> u16 {
    let a1 = mem[pc as usize + 2] as u16;
    let a2 = mem[pc as usize + 1] as u16;
    return (a1 << 8) | a2;
}

fn nop(s: &mut State) {}

fn jp(s: &mut State) {
    let address = u16_le(s.register.pc, &s.mem);
    s.register.pc = address;
}

fn inc_a(s: &mut State) {
    s.register.a = s.register.a + 1;
}

fn inc_l(s: &mut State) {
    s.register.l = s.register.l + 1;
}

fn ld_bc_a(s: &mut State) {
    s.register.b = s.register.a;
}

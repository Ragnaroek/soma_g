
use std::collections::HashMap;
use std::fmt;


pub fn start(mut state: State) -> Result<(), String> {

    let instr_set = instruction_set();

    //TODO model state as a struct with register
    //TODO add function over state to instr to code effect
    loop {
        let op = state.mem[state.reg.pc as usize];
        let instr_lookup = instr_set.get(op);
        if instr_lookup.is_none() {
            return Err(format!("not an instruction: 0x{:x}", op));
        }
        let instr = instr_lookup.unwrap();
        (instr.effect)(&mut state);
        //TODO: modify PC, Registers, Memory accordingly
        println!("instr = {:?}", instr);
        state.reg.pc = state.reg.pc + 1;
    }
}

#[derive(Debug, PartialEq)]
pub struct State {
    pub mem: Vec<u8>,
    pub stack: Vec<u8>,
    pub reg: Register,
}

#[derive(Debug, PartialEq)]
pub struct Register {
    pub pc: u16,
    pub sp: u16,

    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub l: u8,
}

pub fn initial_state(mem: Vec<u8>, stack_size: usize, start_pc: u16, start_sp: u16) -> State {
    return State{mem,
                 stack: vec![0; stack_size],
                 reg: Register{pc: start_pc,
                     sp: start_sp,
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
    instr_set.add_instr(0xCD, "CALL", call);
    //TODO Impl 0xCD instruction (interesting, this is CALL)
    return instr_set;
}

pub fn u16_le(pc: u16, mem: &[u8]) -> u16 {
    let a1 = mem[pc as usize + 2] as u16;
    let a2 = mem[pc as usize + 1] as u16;
    return (a1 << 8) | a2;
}

pub fn u16_reg(reg1: u8, reg2: u8) -> u16 {
    return ((reg1 as u16) << 8) | reg2 as u16;
}

pub fn nop(_s: &mut State) {}

pub fn jp(s: &mut State) {
    let address = u16_le(s.reg.pc, &s.mem);
    s.reg.pc = address;
}

pub fn inc_a(s: &mut State) {
    s.reg.a = s.reg.a + 1;
}

pub fn inc_l(s: &mut State) {
    s.reg.l = s.reg.l + 1;
}

pub fn ld_bc_a(s: &mut State) {
    let ptr = u16_reg(s.reg.b, s.reg.c) as usize;
    s.reg.a = s.mem[ptr];
}

pub fn call(s: &mut State) {

}

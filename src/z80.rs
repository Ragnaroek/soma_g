
use std::collections::HashMap;

pub fn start(mem: &[u8], start_addr: usize) -> Result<(), String> {

    let mut pc = start_addr;
    let instr_set = instruction_set();

    loop {
        let op = mem[pc];
        let instr_lookup = instr_set.get(op);
        if instr_lookup.is_none() {
            return Err(format!("not an instruction: {}", op));
        }
        let instr = instr_lookup.unwrap();
        //TODO: modify PC, Registers, Memory accordingly
        println!("instr = {:?}", instr);
        pc = pc + 1;
    }
}

#[derive(Debug)]
struct Instr {
    mnemonic: String,
    op: u8,
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

    fn add_instr(&mut self, op: u8, mnemonic: &str) {
        let instr = Instr{op: op, mnemonic: mnemonic.to_string()};
        self.set.insert(op, instr);
    }
}

fn instruction_set() -> InstrSet {
    let mut instr_set = InstrSet::new();
    instr_set.add_instr(0, "NOP");
    return instr_set;
}

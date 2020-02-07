use std::num::Wrapping;

use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    pub bytes: usize,
    pub cycles: u64,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, bytes: usize, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, bytes, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })

    }
}

// ADD opcodes
pub fn build_add_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD HL, {:?}", rr),
        bytes: 1,
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = Wrapping(state.reg.get16(Reg16::HL));
            v = v + Wrapping(state.reg.get16(rr));
            state.reg.set16(Reg16::HL, v.0); 
            // TODO: flags
        })
    }
}

// INC, DEC opcodes
pub fn build_inc_dec_rr(rr: Reg16, inc: bool) -> Opcode {
    let delta = if inc {1} else {65535};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {:?}", mnemonic, rr),
        bytes: 1,
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let mut v = Wrapping(state.reg.get16(rr));
            v = v + Wrapping(delta);
            state.reg.set16(rr, v.0);
            // Note: flags not affected
        })
    }    
}    

pub fn build_inc_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("INC {:?}", r),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(r);
            v = if v == 255 {0} else {v+1};

            state.reg.set8(r, v); 
            state.reg.update_sz53_flags(v);
            state.reg.clear_flag(Flag::N);
            state.reg.put_flag(Flag::P, v == 0x80);
            state.reg.put_flag(Flag::H, (v & 0x0F) == 0x00);
            // Flag::C is not affected
        })
    }        
}

pub fn build_dec_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("DEC {:?}", &r),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(r);
            v = if v == 0 {255} else {v-1};

            state.reg.set8(r, v);
            state.reg.update_sz53_flags(v);
            state.reg.set_flag(Flag::N);
            state.reg.put_flag(Flag::P, v == 0x7F);
            state.reg.put_flag(Flag::H, (v & 0x0F) == 0x0F);
            // Flag::C is not affected
        })
    }        
}


use crate ::bus::Bus;
use crate ::opcodes::*;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub stkp : u8,
    pub program_counter: u16,
    bus : Bus,
}
pub enum AddressingMode {
    ACCUMULATOR,
    RELATIVE,
    IMMEDIATE,
    ZEROPAGE,
    ZEROPAGEx,
    ZEROPAGEy,
    ABSOLUTE,
    ABSOLUTEx,
    ABSOLUTEy,
    INDIRECT, // JMP ONLY
    INDIRECTx,
    INDIRECTy,
    ZERO,
    UNIMPLEMENTED,
}

impl CPU {
    pub fn new(bus : Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            stkp : 0,
            program_counter: 0,
            bus,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        loop {
            let read_code = self.read(self.program_counter, None);
            self.program_counter += 1;
            let cycles_taken = self.run_operation(read_code);


        }
    }

    fn run_operation(&mut self, operation : u8) -> u8 {
        match operation {
            LDA_IMM | LDA_ZP | LDA_ZP_X | LDA_ABS | LDA_ABS_X | LDA_ABS_Y | 
            LDA_IND_X | LDA_IND_Y => self.LDA(&ITEM_TABLE[operation as usize].addressing_mode),
            _ => panic!()
        }

        0x00
    }

    fn get_addressed_data(&self, mode: &AddressingMode) -> u8 {
        match mode {
            AddressingMode::ACCUMULATOR => {
                return self.register_a
            }
            _ => panic!()
        }
        
    }

    fn LDA(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_addressed_data(mode);
        self.register_a = loaded_data;
        if loaded_data == 0x00 {
            self.set_status_bit(Self::ZERO_BIT);
        } 

        if (loaded_data & 0b1000_0000) != 0 {
            self.set_status_bit(Self::NEGATIVE_BIT);
        }
        else {
            self.clear_status_bit(Self::NEGATIVE_BIT);
        }

    }

    fn write(&mut self, addr : u16, data : u8) -> () {
        self.bus.write(addr, data);
    
    }
    fn read(&mut self,addr : u16, _bReadOnly : Option<bool>) -> u8 {
        return self.bus.read(addr, _bReadOnly);
    }
// ----------------STATUS----------------------
const CARRY_BIT : u8 = 0b1000_0000;
const ZERO_BIT : u8 = 0b1000_0000;
const DISABLE_INTERRUPTS_BIT : u8 = 0b1000_0000;
const DECIMAL_MODE_BIT : u8 = 0b1000_0000;
const BREAK_BIT : u8 = 0b1000_0000;
const UNUSED_BIT : u8 = 0b1000_0000;
const OVERFLOW_BIT : u8 = 0b1000_0000;
const NEGATIVE_BIT : u8 = 0b1000_0000;

fn set_status_bit (&mut self, bit : u8) {
    self.status |= bit
}
fn clear_status_bit (&mut self, bit : u8) {
    self.status &= !bit
}

}

// C - Carry Bit 0b1000_0000
// Z - Zero 0b0100_0000
// I - Disable Interrupts 0b0010_0000
// D - Decimal Mode (unused) 0b0001_0000
// B - Break 0b0000_1000
// U - Unused 0b0000_0100
// V - Overflow 0b0000_0010
// N - Negative 0b0000_0001


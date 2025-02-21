#![allow(non_snake_case)]
use crate ::bus::Bus;
use crate ::opcodes::*;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
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
            register_y: 0,
            status: 0,
            stkp : 0,
            program_counter: 0,
            bus,
        }
    }

    pub fn interpret(&mut self) {
        self.bus.write(0x0000, LDA_IMM);
        self.bus.write(0x0001, 4);
        self.bus.write(0x0002, STA_ZP);
        self.bus.write(0x0003, 0x00);
        self.bus.write(0x0004, LDA_IMM);
        self.bus.write(0x0005, 7);
        self.bus.write(0x0006, LDA_ZP);
        self.bus.write(0x0007, 0x00);

        println!("Register A: {}\nRegister X : {}", self.register_a, self.register_x);
        loop {
            let read_code = self.read(self.program_counter, None);
            println!("{}", read_code);
            self.program_counter += 1;
            let cycles_taken = self.run_operation(read_code);
            println!("Register A: {}\nRegister X : {}", self.register_a, self.register_x);

        }
    }

    fn run_operation(&mut self, operation : u8) -> u8 {
        match operation {
            LDA_IMM | LDA_ZP | LDA_ZP_X | LDA_ABS | LDA_ABS_X | LDA_ABS_Y | 
            LDA_IND_X | LDA_IND_Y => self.LDA(&ITEM_TABLE[operation as usize].addressing_mode),

            STA_ZP | STA_ZP_X | STA_ABS | STA_ABS_X | STA_ABS_Y | 
            STA_IND_X | STA_IND_Y => self.STA(&ITEM_TABLE[operation as usize].addressing_mode),

            LDX_IMM | LDX_ZP | LDX_ZP_Y | LDX_ABS | LDX_ABS_Y => self.LDX(&ITEM_TABLE[operation as usize].addressing_mode),

            STX_ZP | STX_ZP_Y | STX_ABS => self.STX(&ITEM_TABLE[operation as usize].addressing_mode),

            LDY_IMM | LDY_ZP | LDY_ZP_X | LDY_ABS | LDY_ABS_X => self.LDY(&ITEM_TABLE[operation as usize].addressing_mode),

            STY_ZP | STY_ZP_X | STY_ABS => self.STY(&ITEM_TABLE[operation as usize].addressing_mode),
            _ => panic!()
        }

        0x00
    }

    fn get_addressed_data(&mut self, mode: &AddressingMode) -> u8 {
        match mode {
            AddressingMode::ACCUMULATOR => {
                return self.register_a
            }
            AddressingMode::IMMEDIATE => {
                let value = self.read(self.program_counter, None);
                self.program_counter += 1;
                return value
            }
            AddressingMode::ZEROPAGE | AddressingMode::ZEROPAGEx | AddressingMode::ZEROPAGEy | AddressingMode::INDIRECTx | AddressingMode::INDIRECTy => {
                let address = self.get_address_from_mode(mode);
                self.read(address, None)
            }
            
            _ => panic!()
        }
        
    }

    fn get_address_from_mode(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::ZEROPAGE => {
               let val = self.read(self.program_counter, None) as u16;
               self.program_counter += 1;
               val
                // $0000 to $00FF
            }
            AddressingMode::ZEROPAGEx => {
                let val = self.register_x.wrapping_add(self.read(self.program_counter, None)) as u16;
                self.program_counter += 1;
                val
            }
            AddressingMode::ZEROPAGEy => {
                let val = self.register_y.wrapping_add(self.read(self.program_counter, None)) as u16;
                self.program_counter += 1;
                val
            }
            AddressingMode::ABSOLUTE => {
                let a = self.read(self.program_counter, None);
                self.program_counter += 1;
                let b = self.read(self.program_counter, None);
                self.program_counter += 1;
                Self::combine_u8(a, b)
            }
            AddressingMode::ABSOLUTEx => {
                let a = self.read(self.program_counter, None);
                self.program_counter += 1;
                let b = self.read(self.program_counter, None);
                self.program_counter += 1;
                Self::combine_u8(a, b).wrapping_add(self.register_x as u16)
            }
            AddressingMode::ABSOLUTEy => {
                let a = self.read(self.program_counter, None);
                self.program_counter += 1;
                let b = self.read(self.program_counter, None);
                self.program_counter += 1;
                Self::combine_u8(a, b).wrapping_add(self.register_y as u16)
            }
            AddressingMode::INDIRECTx => {
                let address = self.read(self.program_counter, None);
                let address_2 = address.wrapping_add(self.register_x);
                self.get_address_indirect(address_2 as u16)
            }
            AddressingMode::INDIRECTy => {
                let address = self.read(self.program_counter, None);
                let address_2 = address.wrapping_add(self.register_y);
                self.get_address_indirect(address_2 as u16)
            }
            _ => panic!()
        }
        
        // IMPLEMENT
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

    fn STA(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_address_from_mode(mode);
        self.write(loaded_data, self.register_a);
    }

    fn LDX(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_addressed_data(mode);
        self.register_x = loaded_data;
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

    fn STX(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_address_from_mode(mode);
        self.write(loaded_data, self.register_x);
    }

    fn LDY(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_addressed_data(mode);
        self.register_y = loaded_data;
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

    fn STY(&mut self, mode : &AddressingMode) {
        let loaded_data = self.get_address_from_mode(mode);
        self.write(loaded_data, self.register_y);
    }

    fn write(&mut self, addr : u16, data : u8) -> () {
        self.bus.write(addr, data);
    
    }
    fn read(&mut self,addr : u16, _bReadOnly : Option<bool>) -> u8 {
        return self.bus.read(addr, _bReadOnly);
    }
    fn combine_u8(a : u8, b : u8) -> u16 {
        ((a as u16) << 8) | b as u16
    }
    fn get_address_indirect(&mut self, addr : u16) -> u16 {
        let val1 = self.bus.read(addr, None);
        let val2 = self.bus.read(addr + 1, None);
        Self::combine_u8(val1, val2)
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


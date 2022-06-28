// The MIT License (MIT)
//
// Copyright (c) 2015 Stefan Arentz - http://github.com/st3fan/ewm
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::cpu::{CPU, CPUError};

#[derive(Debug)]
pub struct Computer {
    pub cpu: CPU,
    pub ram: Vec<u8>,
}

// Public API

impl Computer {
    pub fn new() -> Self {
        Computer { cpu: CPU::new(), ram: vec![0; 64*1024] } // TODO Memory is fixed 64K RAM
    }

    pub fn load(&mut self, addr: u16, program: Vec<u8>) {
        for n in 0..program.len() {
            self.ram[(addr + n as u16) as usize] = program[n];
        }
    }

    pub fn run(&mut self) -> Result<(), CPUError> {
        loop {
            self.step()?;
        }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        if addr > 2047 {
            return 0
        }
        return self.ram[addr as usize];
    }

    pub fn set_byte(&mut self, addr: u16, b: u8) {
        self.ram[addr as usize] = b;
    }

    pub fn get_word(&self, addr: u16) -> u16 {
        eprintln!("Debug: get_word({:#04x})", addr);
        return (self.get_byte(addr) as u16) | (self.get_byte(addr+1) as u16) << 8;
    }

    pub fn set_word(&mut self, addr: u16, w: u16) {
        panic!("TODO");
    }
}

impl Computer {
    fn fetch_byte(&mut self) -> u8 {
        let v = self.get_byte(self.cpu.pc);
        self.cpu.pc += 1;
        v
    }

    fn fetch_word(&mut self) -> u16 {
        let v = self.get_word(self.cpu.pc);
        self.cpu.pc += 2;
        return v
    }

    fn push_byte(&mut self, b: u8) {
        self.ram[0x0100 + self.cpu.sp as usize] = b;
        self.cpu.sp -= 1;
    }

    fn pull_byte(&mut self) -> u8 {
        self.cpu.sp += 1;
        let b = self.ram[0x0100 + self.cpu.sp as usize];
        b
    }

    fn push_word(&mut self, w: u16) {
        self.ram[0x0100 + self.cpu.sp as usize] = (w >> 8) as u8;
        self.cpu.sp -= 1;
        self.ram[0x0100 + self.cpu.sp as usize] = (w & 0xff) as u8;
        self.cpu.sp -= 1;
    }

    fn pull_word(&mut self) -> u16 {
        let l = self.pull_byte();
        let h = self.pull_byte();
        (l as u16) | (h as u16) << 8
    }

    fn update_nz(&mut self, v: u8) {
        self.cpu.n = (v & 0x80) != 0;
        self.cpu.z = v == 0;
    }

    fn step(&mut self) -> Result<(), CPUError> {
        let opcode = self.fetch_byte();
        match opcode {
            // Transfer Instructions

            0xa9 => { /* LDA imm */ 
                self.cpu.a = self.fetch_byte();
            }
            0xad => { /* LDA abs */
                let addr = self.fetch_word();
                self.cpu.a = self.get_byte(addr);
            }
            0xa2 => { /* LDX imm (NZ) */ 
                self.cpu.x = self.fetch_byte();
                self.update_nz(self.cpu.x);
            }
            0xae => { /* LDX abs (NZ) */ 
                let addr = self.fetch_word();
                self.cpu.x = self.get_byte(addr);
                self.update_nz(self.cpu.x);
            }
            0xa0 => { /* LDY imm (NZ) */ 
                self.cpu.y = self.fetch_byte();
                self.update_nz(self.cpu.y);
            }
            0xac => { /* LDY abs (NZ) */ 
                let addr = self.fetch_word();
                self.cpu.y = self.get_byte(addr);
                self.update_nz(self.cpu.y);
            }

            // Decrements & Increments

            0xe8 => { // INX (NZ)
                self.cpu.x += 1;
                self.update_nz(self.cpu.x);
            }

            0xc8 => { // INY (NZ)
                self.cpu.y += 1;
                self.update_nz(self.cpu.y);
            }

            // Jumps & Subroutines

            0x4c => { // JMP abs
                let addr = self.fetch_word();
                self.cpu.pc = addr;
            }

            0x6c => { // JMP ind
                let addr = self.fetch_word();
                self.cpu.pc = self.get_word(addr);
            }

            0x20 => { // JSR abs
                let addr = self.fetch_word();
                self.push_word(self.cpu.pc);
                self.cpu.pc = addr;
            }

            0x60 => { // RTS
                let addr = self.pull_word();
                self.cpu.pc = addr;
            }

            // Logical Operations: AND

            0x29 => { // AND imm
                let oper = self.fetch_byte();
                self.and(oper);
            }

            0x25 => { // AND zp
                let oper = self.fetch_byte();
                self.and(self.mem_get_byte_zpg(oper));
            }

            0x35 => { // AND zpx
                let oper = self.fetch_byte();
                self.and(self.mem_get_byte_zpgx(oper));
            }

            0x2D => { // AND abs
                let oper = self.fetch_word();
                self.and(self.mem_get_byte_abs(oper));
            }

            0x3D => { // AND absx
                let oper = self.fetch_word();
                self.and(self.mem_get_byte_absx(oper));
            }

            0x39 => { // AND absy
                let oper = self.fetch_word();
                self.and(self.mem_get_byte_absy(oper));
            }

            0x31 => { // AND indx
                let oper = self.fetch_byte();
                self.and(self.mem_get_byte_indx(oper));
            }

            0x21 => { // AND indy
                let oper = self.fetch_byte();
                self.and(self.mem_get_byte_indy(oper));
            }

            // Logical operations: EOR

            0x49 => { // EOR imm
                let oper = self.fetch_byte();
                self.eor(oper);
            }

            0x45 => { // EOR zp
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_zpg(oper));
            }

            0x55 => { // EOR zpx
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_zpgx(oper));
            }

            0x4D => { // EOR abs
                let oper = self.fetch_word();
                self.eor(self.mem_get_byte_abs(oper));
            }

            0x5D => { // EOR absx
                let oper = self.fetch_word();
                self.eor(self.mem_get_byte_absx(oper));
            }

            0x59 => { // EOR absy
                let oper = self.fetch_word();
                self.eor(self.mem_get_byte_absy(oper));
            }

            0x41 => { // EOR indx
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_indx(oper));
            }

            0x51 => { // EOR indy
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_indy(oper));
            }

            // Logical operations: ORA

            0x09 => { // ORA imm
                let oper = self.fetch_byte();
                self.ora(oper);
            }

            0x05 => { // ORA zp
                let oper = self.fetch_byte();
                self.ora(self.mem_get_byte_zpg(oper));
            }

            0x15 => { // ORA zpx
                let oper = self.fetch_byte();
                self.ora(self.mem_get_byte_zpgx(oper));
            }

            0x0D => { // ORA abs
                let oper = self.fetch_word();
                self.ora(self.mem_get_byte_abs(oper));
            }

            0x1D => { // ORA absx
                let oper = self.fetch_word();
                self.ora(self.mem_get_byte_absx(oper));
            }

            0x19 => { // ORA absy
                let oper = self.fetch_word();
                self.ora(self.mem_get_byte_absy(oper));
            }

            0x01 => { // ORA indx
                let oper = self.fetch_byte();
                self.ora(self.mem_get_byte_indx(oper));
            }

            0x11 => { // EOR indy
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_indy(oper));
            }

            // Interrupts

            0x00 => { // BRK
                // TODO This is incorrect
                self.cpu.pc -= 1;
                return Err(CPUError::Break);
            }

            0x40 => { // RTI
                // TODO
            }

            // Other
            
            0x24 => { // BIT zp
                let oper = self.fetch_byte();
                self.bit(self.mem_get_byte_zpg(oper));
            }

            0x2C => { // BIT abs
                let oper = self.fetch_word();
                self.bit(self.mem_get_byte_abs(oper));
            }

            0xEA => { // NOP
            }

            // Anything else results in an error. This won't work well because some code depends on
            // either behaviour of undefined 6502 opcodes or tries to detect the 65C02. For later.

            _ => {
                self.cpu.pc -= 1;
                return Err(CPUError::IllegalOpcode);
            }
        }
        Ok(())
    }
}

// TODO How to split this up into cpu_micro_ops.rs

impl Computer {
    // Logic Operations
    
    pub fn and(&mut self, m: u8) {
        self.cpu.a &= m;
        self.update_nz(self.cpu.a);
    }

    pub fn eor(&mut self, m: u8) {
        self.cpu.a ^= m;
        self.update_nz(self.cpu.a);
    }

    pub fn ora(&mut self, m: u8) {
        self.cpu.a |= m;
        self.update_nz(self.cpu.a);
    }

    // Other

    pub fn bit(&mut self, m: u8) {
        let t = self.cpu.a & m;
        self.cpu.n = (m & 0x80) != 0; // TODO This could be a simple transfer if we stored the flags in a u8
        self.cpu.v = (m & 0x40) != 0;
        self.cpu.z = t == 0;
    }
}

// TODO How to split this up into cpu_addressing.rs

impl Computer {
    pub fn mem_get_byte_zpg(&self, addr: u8) -> u8 {
        self.get_byte(addr as u16)
    }

    pub fn mem_get_byte_zpgx(&self, addr: u8) -> u8 {
        self.get_byte(addr.wrapping_add(self.cpu.x) as u16)
    }

    pub fn mem_get_byte_zpgy(&self, addr: u8) -> u8 {
        self.get_byte(addr.wrapping_add(self.cpu.y) as u16)
    }

    pub fn mem_get_byte_abs(&self, addr: u16) -> u8 {
        self.get_byte(addr)
    }

    pub fn mem_get_byte_absx(&self, addr: u16) -> u8 {
        self.get_byte(addr.wrapping_add(self.cpu.x as u16))
    }

    pub fn mem_get_byte_absy(&self, addr: u16) -> u8 {
        self.get_byte(addr.wrapping_add(self.cpu.y as u16))
    }

    // TODO 65C02 and use for JMP?
    // pub fn mem_get_byte_ind(&self, addr: u8) -> u8 {
    // }

    pub fn mem_get_byte_indx(&self, addr: u8) -> u8 {
        self.get_byte(self.get_word(addr.wrapping_add(self.cpu.x) as u16))
    }

    pub fn mem_get_byte_indy(&self, addr: u8) -> u8 {
        self.get_byte(self.get_word(addr as u16).wrapping_add(self.cpu.y as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Memory tests

    #[test]
    fn test_mem_get_byte_zpg() {
        let mut computer = Computer::new();
        computer.set_byte(13, 42);
        assert_eq!(computer.mem_get_byte_zpg(13), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgx() {
        let mut computer = Computer::new();
        computer.set_byte(13, 42);
        computer.cpu.x = 8;
        assert_eq!(computer.mem_get_byte_zpgx(5), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgx_wrapping() {
        let mut computer = Computer::new();
        computer.set_byte(0x10, 0x42);
        computer.cpu.x = 0x20;
        assert_eq!(computer.mem_get_byte_zpgx(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_zpgy() {
        let mut computer = Computer::new();
        computer.set_byte(13, 42);
        computer.cpu.y = 8;
        assert_eq!(computer.mem_get_byte_zpgy(5), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgy_wrapping() {
        let mut computer = Computer::new();
        computer.set_byte(0x10, 0x42);
        computer.cpu.y = 0x20;
        assert_eq!(computer.mem_get_byte_zpgy(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indx() {
        let mut computer = Computer::new();
        computer.set_byte(0x0432, 0x42);
        computer.set_byte(5, 0x32);
        computer.set_byte(6, 0x04);
        computer.cpu.x = 2;
        assert_eq!(computer.mem_get_byte_indx(3), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indx_wrapping() {
        let mut computer = Computer::new();
        computer.set_byte(0x0432, 0x42);
        computer.set_byte(0x20, 0x32);
        computer.set_byte(0x21, 0x04);
        computer.cpu.x = 0x30;
        assert_eq!(computer.mem_get_byte_indx(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indy() {
        let mut computer = Computer::new();
        computer.set_byte(0x0444, 0x42);
        computer.set_byte(5, 0x00);
        computer.set_byte(6, 0x04);
        computer.cpu.y = 0x44;
        assert_eq!(computer.mem_get_byte_indy(5), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indy_wrapping() {
        let mut computer = Computer::new();
        computer.set_byte(0x010, 0x42);
        computer.set_byte(5, 0xf0);
        computer.set_byte(6, 0xff);
        computer.cpu.y = 0x20;
        assert_eq!(computer.mem_get_byte_indy(5), 0x42);
    }
}

#[cfg(test)]
mod computer_tests {
    use super::*;

    #[test]
    fn test_initialized() {
        let computer = Computer::new();
        assert_eq!(computer.cpu.a, 0x00);
        assert_eq!(computer.cpu.x, 0x00);
        assert_eq!(computer.cpu.y, 0x00);
        assert_eq!(computer.cpu.sp, 0xff);
        assert_eq!(computer.cpu.pc, 0x0400);
        assert_eq!(computer.ram.len(), 64*1024);
    }

    #[test]
    fn test_stack_bytes() {
        let mut computer = Computer::new();
        assert_eq!(computer.cpu.sp, 0xff);

        computer.push_byte(0x42);
        assert_eq!(computer.cpu.sp, 0xfe);
        assert_eq!(computer.get_byte(0x01ff), 0x42);

        computer.push_byte(0x21);
        assert_eq!(computer.cpu.sp, 0xfd);
        assert_eq!(computer.get_byte(0x01fe), 0x21);

        assert_eq!(computer.pull_byte(), 0x21);
        assert_eq!(computer.cpu.sp, 0xfe);

        assert_eq!(computer.pull_byte(), 0x42);
        assert_eq!(computer.cpu.sp, 0xff);
    }

    #[test]
    fn test_stack_words() {
        let mut computer = Computer::new();
        assert_eq!(computer.cpu.sp, 0xff);

        computer.push_word(0x1234);
        assert_eq!(computer.cpu.sp, 0xfd);
        assert_eq!(computer.get_byte(0x01ff), 0x12);
        assert_eq!(computer.get_byte(0x01fe), 0x34);

        computer.push_word(0x6502);
        assert_eq!(computer.cpu.sp, 0xfb);
        assert_eq!(computer.get_byte(0x01fd), 0x65);
        assert_eq!(computer.get_byte(0x01fc), 0x02);

        assert_eq!(computer.pull_word(), 0x6502);
        assert_eq!(computer.pull_word(), 0x1234);
    }

    #[test]
    fn test_breaks_without_program() {
        let mut computer = Computer::new();
        assert_eq!(computer.run().unwrap_err(), CPUError::Break);
        assert_eq!(computer.cpu.pc, 0x0400);
    }

    #[test]
    fn test_load_registers() {
        let program: Vec<u8> = vec![
            0xa9, 0x11, // $0400 LDA $11
            0xa2, 0x22, // $0402 LDX $22
            0xa0, 0x33, // $0404 LDY $33
            0x00        // $0406 BRK
        ];

        let mut computer = Computer::new();
        computer.load(0x0400, program);

        assert_eq!(computer.run().unwrap_err(), CPUError::Break);

        assert_eq!(computer.cpu.a, 0x11);
        assert_eq!(computer.cpu.x, 0x22);
        assert_eq!(computer.cpu.y, 0x33);
        assert_eq!(computer.cpu.sp, 0xff);
        assert_eq!(computer.cpu.pc, 0x0406);
    }

    #[test]
    fn test_jmp_abs() {
        let mut computer = Computer::new();
        computer.load(0x0400, vec![
            0x4C, 0x05, 0x04,   // $0400 JMP $0405
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x00                // $0407 BRK
        ]);

        assert_eq!(computer.run().unwrap_err(), CPUError::Break);
        assert_eq!(computer.cpu.pc, 0x0407);
        assert_eq!(computer.cpu.a, 0x42);
    }

    #[test]
    fn test_jmp_ind() {
        let mut computer = Computer::new();
        computer.load(0x0400, vec![
            0x6C, 0x08, 0x04,   // $0400 JMP ($0408)
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x00,               // $0407 BRK
            0x05, 0x04          // $0408
        ]);

        assert_eq!(computer.run().unwrap_err(), CPUError::Break);
        assert_eq!(computer.cpu.pc, 0x0407);
        assert_eq!(computer.cpu.a, 0x42);
    }

    #[test]
    fn test_jsr_rts() {
        let mut computer = Computer::new();
        computer.load(0x0400, vec![
            0x20, 0x05, 0x04,   // $0400 JSR $0405
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x60,               // $0407 RTS
        ]);

        assert_eq!(computer.run().unwrap_err(), CPUError::Break);
        assert_eq!(computer.cpu.pc, 0x0403);
        assert_eq!(computer.cpu.a, 0x42);
        assert_eq!(computer.cpu.sp, 0xff);
    }
}

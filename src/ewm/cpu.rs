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

#[derive(Debug)]
pub struct CPU {
    // TODO Move this to State ?
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,

    pub n: bool,
    pub v: bool,
    pub b: bool,
    pub d: bool,
    pub i: bool,
    pub z: bool,
    pub c: bool,

    pub ram: Vec<u8>,
}

// Public API

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0x0400, sp: 0xff,
            a: 0, x: 0, y: 0,
            n: false, v: false, b: false, d: false, i: false, z: false, c: false,
            ram: vec![0; 64*1024]
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

    // TODO These are also in Computer which makes no sense

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
}

//

#[derive(Debug, PartialEq)]
pub enum CPUError {
    Break,
    IllegalOpcode,
}

// Status

impl CPU {
    fn get_status(&self) -> u8 {
        let mut status = 0;
        if self.n { status |= 0b10000000; }
        if self.v { status |= 0b01000000; }
        if self.b { status |= 0b00010000; }
        if self.d { status |= 0b00001000; }
        if self.i { status |= 0b00000100; }
        if self.z { status |= 0b00000010; }
        if self.c { status |= 0b00000001; }
        status
    }

    fn set_status(&mut self, status: u8) {
        self.n = (status & 0b10000000) != 0;
        self.v = (status & 0b01000000) != 0;
        self.b = (status & 0b00010000) != 0;
        self.d = (status & 0b00001000) != 0;
        self.i = (status & 0b00000100) != 0;
        self.z = (status & 0b00000010) != 0;
        self.c = (status & 0b00000001) != 0;
    }
}

// Modifiers

fn asl(b: u8) -> u8 {
    b
}

fn lsr(b: u8) -> u8 {
    b
}

fn rol(b: u8) -> u8 {
    b
}

fn ror(b: u8) -> u8 {
    b
}

//

impl CPU {
    fn fetch_byte(&mut self) -> u8 {
        let v = self.get_byte(self.pc);
        self.pc += 1;
        v
    }

    fn fetch_word(&mut self) -> u16 {
        let v = self.get_word(self.pc);
        self.pc += 2;
        return v
    }

    fn push_byte(&mut self, b: u8) {
        self.ram[0x0100 + self.sp as usize] = b;
        self.sp -= 1;
    }

    fn pull_byte(&mut self) -> u8 {
        self.sp += 1;
        self.ram[0x0100 + self.sp as usize]
    }

    fn push_word(&mut self, w: u16) {
        self.ram[0x0100 + self.sp as usize] = (w >> 8) as u8;
        self.sp -= 1;
        self.ram[0x0100 + self.sp as usize] = (w & 0xff) as u8;
        self.sp -= 1;
    }

    fn pull_word(&mut self) -> u16 {
        let l = self.pull_byte();
        let h = self.pull_byte();
        (l as u16) | (h as u16) << 8
    }

    fn update_nz(&mut self, v: u8) {
        self.n = (v & 0x80) != 0;
        self.z = v == 0;
    }

    fn step(&mut self) -> Result<(), CPUError> {
        let opcode = self.fetch_byte();
        match opcode {
            // Transfer Instructions

            // LDA

            0xA9 => { // LDA imm
                self.a = self.fetch_byte();
                self.update_nz(self.a);
            }

            0xA5 => { // LDA zpg
                let oper = self.fetch_byte();
                self.a = self.mem_get_byte_zpg(oper);
                self.update_nz(self.a);
            }

            0xB5 => { // LDA zpgx
                let oper = self.fetch_byte();
                self.a = self.mem_get_byte_zpgx(oper);
                self.update_nz(self.a);
            }

            0xAD => { // LDA abs
                let oper = self.fetch_word();
                self.a = self.mem_get_byte_abs(oper);
                self.update_nz(self.a);
            }

            0xBD => { // LDA absx
                let oper = self.fetch_word();
                self.a = self.mem_get_byte_absx(oper);
                self.update_nz(self.a);
            }

            0xB9 => { // LDA absy
                let oper = self.fetch_word();
                self.a = self.mem_get_byte_absy(oper);
                self.update_nz(self.a);
            }

            0xA1 => { // LDA indx
                let oper = self.fetch_byte();
                self.a = self.mem_get_byte_indx(oper);
                self.update_nz(self.a);
            }

            0xB1 => { // LDA indy
                let oper = self.fetch_byte();
                self.a = self.mem_get_byte_indy(oper);
                self.update_nz(self.a);
            }

            // LDX

            0xA2 => { // LDX imm
                self.x = self.fetch_byte();
                self.update_nz(self.x);
            }

            0xA6 => { // LDX zpg
                let oper = self.fetch_byte();
                self.x = self.mem_get_byte_zpg(oper);
                self.update_nz(self.x);
            }

            0xB6 => { // LDX zpgy
                let oper = self.fetch_byte();
                self.x = self.mem_get_byte_zpgy(oper);
                self.update_nz(self.x);
            }

            0xAE => { // LDX abs
                let oper = self.fetch_word();
                self.x = self.mem_get_byte_abs(oper);
                self.update_nz(self.x);
            }

            0xBE => { // LDX absy
                let oper = self.fetch_word();
                self.x = self.mem_get_byte_absy(oper);
                self.update_nz(self.x);
            }

            // LDY

            0xA0 => { // LDY imm
                self.y = self.fetch_byte();
                self.update_nz(self.y);
            }

            0xA4 => { // LDY zpg
                let oper = self.fetch_byte();
                self.y = self.mem_get_byte_zpg(oper);
                self.update_nz(self.y);
            }

            0xB4 => { // LDY zpgx
                let oper = self.fetch_byte();
                self.y = self.mem_get_byte_zpgx(oper);
                self.update_nz(self.y);
            }

            0xAC => { // LDY abs
                let oper = self.fetch_word();
                self.y = self.mem_get_byte_abs(oper);
                self.update_nz(self.y);
            }

            0xBC => { // LDY absx
                let oper = self.fetch_word();
                self.y = self.mem_get_byte_absx(oper);
                self.update_nz(self.y);
            }

            // STA

            0x85 => { // STA zpg
                let oper = self.fetch_byte();
                self.mem_set_byte_zpg(oper, self.a);
            }

            0x95 => { // STA zpgx
                let oper = self.fetch_byte();
                self.mem_set_byte_zpgx(oper, self.a);
            }

            0x8D => { // STA abs
                let oper = self.fetch_word();
                self.mem_set_byte_abs(oper, self.a);
            }

            0x9D => { // STA absx
                let oper = self.fetch_word();
                self.mem_set_byte_absx(oper, self.a);
            }

            0x99 => { // STA absy
                let oper = self.fetch_word();
                self.mem_set_byte_absy(oper, self.a);
            }

            0x81 => { // STA indx
                let oper = self.fetch_byte();
                self.mem_set_byte_indx(oper, self.a);
            }

            0x91 => { // STA indy
                let oper = self.fetch_byte();
                self.mem_set_byte_indy(oper, self.a);
            }

            // STX

            0x86 => { // STX zpg
                let oper = self.fetch_byte();
                self.mem_set_byte_zpg(oper, self.x);
            }

            0x96 => { // STX zpgy
                let oper = self.fetch_byte();
                self.mem_set_byte_zpgy(oper, self.x);
            }

            0x8E => { // STX abs
                let oper = self.fetch_word();
                self.mem_set_byte_abs(oper, self.x);
            }

            // STY

            0x84 => { // STY zpg
                let oper = self.fetch_byte();
                self.mem_set_byte_zpg(oper, self.y);
            }

            0x94 => { // STY zpgx
                let addr = self.fetch_byte();
                self.mem_set_byte_zpgx(addr, self.y);
            }

            0x8C => { // STY abs
                let oper = self.fetch_word();
                self.mem_set_byte_abs(oper, self.y);
            }

            // Transfer between registers

            0xAA => { // TAX
                self.x = self.a;
                self.update_nz(self.x);
            }

            0xA8 => { // TAY
                self.y = self.a;
                self.update_nz(self.y);
            }

            0xBA => { // TSX
                self.x = self.sp;
                self.update_nz(self.x);
            }

            0x8A => { // TXA
                self.a = self.x;
                self.update_nz(self.a);
            }

            0x9A => { // TXS
                self.sp = self.x;
            }

            0x98 => { // TYA
                self.a = self.y;
                self.update_nz(self.a);
            }

            // Stack Instructions

            0x48 => { // PHA
                self.push_byte(self.a);
            }

            0x08 => { // PHP
                let oper = self.get_status() | 0b00110000;
                self.push_byte(oper);
            }

            0x68 => { // PLA
                self.a = self.pull_byte();
                self.update_nz(self.a);
            }

            0x28 => { // PLP
                let oper = self.pull_byte();
                self.set_status(oper);
            }

            // Decrements & Increments

            0xE6 => { // INC zpg
                let oper = self.fetch_byte();
                let v = self.get_byte(oper as u16).wrapping_add(1);
                self.set_byte(oper as u16, v);
                self.update_nz(v);
            }
    
            0xF6 => { // INC zpx
                let oper = self.fetch_byte();
                let v = self.get_byte((oper + self.x) as u16).wrapping_add(1);
                self.set_byte((oper + self.x) as u16, v);
                self.update_nz(v);
            }

            0xEE => { // INC abs
                let oper = self.fetch_word();
                let v = self.get_byte(oper).wrapping_add(1);
                self.set_byte(oper, v);
                self.update_nz(v);
            }

            0xFE => { // INC absx
                let oper = self.fetch_word();
                let v = self.get_byte(oper + self.x as u16).wrapping_add(1);
                self.set_byte(oper + self.x as u16, v);
                self.update_nz(v);
            }

            0xe8 => { // INX (NZ)
                self.x = self.x.wrapping_add(1);
                self.update_nz(self.x);
            }

            0xc8 => { // INY (NZ)
                self.y = self.y.wrapping_add(1);
                self.update_nz(self.y);
            }

            0xC6 => { // DEC zpg
                let oper = self.fetch_byte();
                let v = self.get_byte(oper as u16).wrapping_sub(1);
                self.set_byte(oper as u16, v);
                self.update_nz(v);
            }
    
            0xD6 => { // DEC zpx
                let oper = self.fetch_byte();
                let v = self.get_byte((oper + self.x) as u16).wrapping_sub(1);
                self.set_byte((oper + self.x) as u16, v);
                self.update_nz(v);
            }

            0xCE => { // DEC abs
                let oper = self.fetch_word();
                let v = self.get_byte(oper).wrapping_sub(1);
                self.set_byte(oper, v);
                self.update_nz(v);
            }

            0xDE => { // DEC absx
                let oper = self.fetch_word();
                let v = self.get_byte(oper + self.x as u16).wrapping_sub(1);
                self.set_byte(oper + self.x as u16, v);
                self.update_nz(v);
            }

            0xCA => { // DEX (NZ)
                self.x = self.x.wrapping_sub(1);
                self.update_nz(self.x);
            }

            0x88 => { // DEY (NZ)
                self.y = self.y.wrapping_sub(1);
                self.update_nz(self.y);
            }

            // Arithmetic Operations

            0x69 => { // ADC immediate
                let m = self.fetch_byte();
                self.adc(m)
            }

            0x65 => { // ADC zpg
                let addr = self.fetch_byte();
                self.adc(self.mem_get_byte_zpg(addr));
            }

            0x75 => { // ADC zpgx
                let addr = self.fetch_byte();
                self.adc(self.mem_get_byte_zpgx(addr));
            }

            0x6D => { // ADC abs
                let addr = self.fetch_word();
                self.adc(self.mem_get_byte_abs(addr));
            }

            0x7D => { // ADC absx
                let addr = self.fetch_word();
                self.adc(self.mem_get_byte_absx(addr));
            }

            0x79 => { // ADC absy
                let addr = self.fetch_word();
                self.adc(self.mem_get_byte_absy(addr));
            }

            0x61 => { // ADC indx
                let addr = self.fetch_byte();
                self.adc(self.mem_get_byte_indx(addr));
            }

            0x71 => { // ADC indy
                let addr = self.fetch_byte();
                self.adc(self.mem_get_byte_indy(addr));
            }

            0xE9 => { // SBC immediate
                let m = self.fetch_byte();
                self.sbc(m)
            }

            0xE5 => { // SBC zpg
                let addr = self.fetch_byte();
                self.sbc(self.mem_get_byte_zpg(addr));
            }

            0xF5 => { // SBC zpgx
                let addr = self.fetch_byte();
                self.sbc(self.mem_get_byte_zpgx(addr));
            }

            0xED => { // SBC abs
                let addr = self.fetch_word();
                self.sbc(self.mem_get_byte_abs(addr));
            }

            0xFD => { // SBC absx
                let addr = self.fetch_word();
                self.sbc(self.mem_get_byte_absx(addr));
            }

            0xF9 => { // SBC absy
                let addr = self.fetch_word();
                self.sbc(self.mem_get_byte_absy(addr));
            }

            0xE1 => { // SBC indx
                let addr = self.fetch_byte();
                self.sbc(self.mem_get_byte_indx(addr));
            }

            0xF1 => { // SBC indy
                let addr = self.fetch_byte();
                self.sbc(self.mem_get_byte_indy(addr));
            }

            // Jumps & Subroutines

            0x4c => { // JMP abs
                let addr = self.fetch_word();
                self.pc = addr;
            }

            0x6c => { // JMP ind
                let addr = self.fetch_word();
                self.pc = self.get_word(addr);
            }

            0x20 => { // JSR abs
                let addr = self.fetch_word();
                self.push_word(self.pc);
                self.pc = addr;
            }

            0x60 => { // RTS
                let addr = self.pull_word();
                self.pc = addr;
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

            // Shift & Rotate Instructions

            0x0A => { // ASL
                self.a = asl(self.a);
            }

            0x06 => { // ASL zpg
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpg(addr, asl);
            }

            0x16 => { // ASL zpgx
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpgx(addr, asl);
            }

            0x0E => { // ASL abs
                let addr = self.fetch_word();
                self.mem_mod_byte_abs(addr, asl);
            }

            0x1E => { // ASL absx
                let addr = self.fetch_word();
                self.mem_mod_byte_absx(addr, asl);
            }

            0x4A => { // LSR
                self.a = lsr(self.a);
            }

            0x46 => { // LSR zpg
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpg(addr, lsr);
            }

            0x56 => { // LSR zpgx
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpgx(addr, lsr);
            }

            0x4E => { // LSR abs
                let addr = self.fetch_word();
                self.mem_mod_byte_abs(addr, lsr);
            }

            0x5E => { // LSR absx
                let addr = self.fetch_word();
                self.mem_mod_byte_absx(addr, lsr);
            }

            0x2A => { // ROL
                self.a = rol(self.a);
            }

            0x26 => { // ROL zpg
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpg(addr, rol);
            }

            0x36 => { // ROL zpgx
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpgx(addr, rol);
            }

            0x2E => { // ROL abs
                let addr = self.fetch_word();
                self.mem_mod_byte_abs(addr, rol);
            }

            0x3E => { // ROL absx
                let addr = self.fetch_word();
                self.mem_mod_byte_absx(addr, rol);
            }

            0x6A => { // ROR
                self.a = ror(self.a);
            }

            0x66 => { // ROR zpg
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpg(addr, ror);
            }

            0x76 => { // ROR zpgx
                let addr = self.fetch_byte();
                self.mem_mod_byte_zpgx(addr, ror);
            }

            0x6E => { // ROR abs
                let addr = self.fetch_word();
                self.mem_mod_byte_abs(addr, ror);
            }

            0x7E => { // ROR absx
                let addr = self.fetch_word();
                self.mem_mod_byte_absx(addr, ror);
            }

            // Flag Instructions

            0x18 => { // CLC
                self.c = false;
            }

            0xD8 => { // CLD
                self.d = false;
            }

            0x58 => { // CLI
                self.i = false;
            }

            0xB8 => { // CLV
                self.v = false;
            }

            0x38 => { // SEC
                self.c = true;
            }

            0xF8 => { // SED
                self.d = true;
            }

            0x78 => { // SEI
                self.i = true;
            }

            // Comparisons
            
            // CMP

            0xc9 => { // CMP imm
                let oper = self.fetch_byte();
                self.cmp(oper);
            }

            0xc5 => { // CMP zpg
                let oper = self.fetch_byte();
                self.cmp(self.mem_get_byte_zpg(oper));
            }

            0xd5 => { // CMP zpx
                let oper = self.fetch_byte();
                self.cmp(self.mem_get_byte_zpgx(oper));
            }

            0xcd => { // CMP abs
                let oper = self.fetch_word();
                self.cmp(self.mem_get_byte_abs(oper));
            }

            0xDD => { // CMP absx
                let oper = self.fetch_word();
                self.cmp(self.mem_get_byte_absx(oper));
            }

            0xD9 => { // CMP absy
                let oper = self.fetch_word();
                self.cmp(self.mem_get_byte_absy(oper));
            }

            0xC1 => { // CMP indx
                let oper = self.fetch_byte();
                self.cmp(self.mem_get_byte_indx(oper));
            }

            0xD1 => { // CMP indy
                let oper = self.fetch_byte();
                self.eor(self.mem_get_byte_indy(oper));
            }

            // CPX

            0xE0 => { // CPX imm
                let oper = self.fetch_byte();
                self.cpx(oper);
            }

            0xE4 => { // CPX zpg
                let oper = self.fetch_byte();
                self.cpx(self.mem_get_byte_zpg(oper));
            }

            0xEC => { // CPX abs
                let oper = self.fetch_word();
                self.cpx(self.mem_get_byte_abs(oper));
            }

            // CPY

            0xC0 => { // CPY imm
                let oper = self.fetch_byte();
                self.cpy(oper);
            }

            0xC4 => { // CPY zpg
                let oper = self.fetch_byte();
                self.cpy(self.mem_get_byte_zpg(oper));
            }

            0xCC => { // CPY abs
                let oper = self.fetch_word();
                self.cpy(self.mem_get_byte_abs(oper));
            }

            // Branches

            0x90 => { // BCC
                let oper = self.fetch_byte();
                if !self.c {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0xB0 => { // BCS
                let oper = self.fetch_byte();
                if self.c {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0xF0 => { // BEQ
                let oper = self.fetch_byte();
                if self.z {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0x30 => { // BMI
                let oper = self.fetch_byte();
                if self.n {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0xD0 => { // BNE
                let oper = self.fetch_byte();
                if !self.z {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0x10 => { // BPL
                let oper = self.fetch_byte();
                if !self.n {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0x50 => { // BVC
                let oper = self.fetch_byte();
                if !self.v {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            0x70 => { // BVS
                let oper = self.fetch_byte();
                if self.v {
                    self.pc = self.pc.wrapping_add(oper as u16);
                }
            }

            // Interrupts

            0x00 => { // BRK
                // TODO This is incorrect
                self.pc -= 1;
                return Err(CPUError::Break);
            }

            0x40 => { // RTI
                // TODO
            }

            // Other
            
            0x24 => { // BIT zpg
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
                self.pc -= 1;
                return Err(CPUError::IllegalOpcode);
            }
        }
        Ok(())
    }
}

// TODO How to split this up into cpu_micro_ops.rs

impl CPU {
    // Logic Operations
    
    fn and(&mut self, m: u8) {
        self.a &= m;
        self.update_nz(self.a);
    }

    fn eor(&mut self, m: u8) {
        self.a ^= m;
        self.update_nz(self.a);
    }

    fn ora(&mut self, m: u8) {
        self.a |= m;
        self.update_nz(self.a);
    }

    // Comparisons

    fn cmp(&mut self, m: u8) {
        let t = self.a.wrapping_sub(m);
        self.c = self.a >= m;
        self.n = (t & 0x80) != 0;
        self.z = t == 0;
    }

    fn cpx(&mut self, m: u8) {
        let t = self.x.wrapping_sub(m);
        self.c = self.x >= m;
        self.n = (t & 0x80) != 0;
        self.z = t == 0;
    }

    fn cpy(&mut self, m: u8) {
        let t = self.y.wrapping_sub(m);
        self.c = self.y >= m;
        self.n = (t & 0x80) != 0;
        self.z = t == 0;
    }

    // Other

    fn bit(&mut self, m: u8) {
        let t = self.a & m;
        self.n = (m & 0x80) != 0; // TODO This could be a simple transfer if we stored the flags in a u8
        self.v = (m & 0x40) != 0;
        self.z = t == 0;
    }

    fn adc(&mut self, m: u8) {
        // TODO
    }

    fn sbc(&mut self, m: u8) {
        // TODO
    }
}

#[cfg(test)]
mod operations_tests {
    use super::*;

    #[test]
    fn test_cmp_eq() {
        let mut cpu = CPU::new();
        cpu.a = 0x42;
        cpu.cmp(0x42);
        assert_eq!(cpu.z, true);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.n, false);
    }

    #[test]
    fn test_cmp_gt() {
        let mut cpu = CPU::new();
        cpu.a = 0x42;
        cpu.cmp(0x21);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.n, false);
    }

    #[test]
    fn test_cmp_gt_n() {
        let mut cpu = CPU::new();
        cpu.a = 0x84;
        cpu.cmp(0x01);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.c, true);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_cmp_lt() {
        let mut cpu = CPU::new();
        cpu.a = 0x42;
        cpu.cmp(0x84);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.c, false);
        assert_eq!(cpu.n, true);
    }

    #[test]
    fn test_cmp_lt_n() {
        let mut cpu = CPU::new();
        cpu.a = 0x01;
        cpu.cmp(0x84);
        assert_eq!(cpu.z, false);
        assert_eq!(cpu.c, false);
        assert_eq!(cpu.n, false);
    }
}

// TODO How to split this up into cpu_addressing.rs

impl CPU {
    // Getters

    pub fn mem_get_byte_zpg(&self, addr: u8) -> u8 {
        self.get_byte(addr as u16)
    }

    pub fn mem_get_byte_zpgx(&self, addr: u8) -> u8 {
        self.get_byte(addr.wrapping_add(self.x) as u16)
    }

    pub fn mem_get_byte_zpgy(&self, addr: u8) -> u8 {
        self.get_byte(addr.wrapping_add(self.y) as u16)
    }

    pub fn mem_get_byte_abs(&self, addr: u16) -> u8 {
        self.get_byte(addr)
    }

    pub fn mem_get_byte_absx(&self, addr: u16) -> u8 {
        self.get_byte(addr.wrapping_add(self.x as u16))
    }

    pub fn mem_get_byte_absy(&self, addr: u16) -> u8 {
        self.get_byte(addr.wrapping_add(self.y as u16))
    }

    pub fn mem_get_byte_indx(&self, addr: u8) -> u8 {
        self.get_byte(self.get_word(addr.wrapping_add(self.x) as u16))
    }

    pub fn mem_get_byte_indy(&self, addr: u8) -> u8 {
        self.get_byte(self.get_word(addr as u16).wrapping_add(self.y as u16))
    }

    // Setters

    pub fn mem_set_byte_zpg(&mut self, addr: u8, b: u8) {
        self.set_byte(addr as u16, b);
    }

    pub fn mem_set_byte_zpgx(&mut self, addr: u8, b: u8) {
        self.set_byte(addr.wrapping_add(self.x) as u16, b);
    }

    pub fn mem_set_byte_zpgy(&mut self, addr: u8, b: u8) {
        self.set_byte(addr.wrapping_add(self.y) as u16, b);
    }

    pub fn mem_set_byte_abs(&mut self, addr: u16, b: u8) {
        self.set_byte(addr, b);
    }

    pub fn mem_set_byte_absx(&mut self, addr: u16, b: u8) {
        self.set_byte(addr.wrapping_add(self.x as u16), b);
    }

    pub fn mem_set_byte_absy(&mut self, addr: u16, b: u8) {
        self.set_byte(addr.wrapping_add(self.y as u16), b);
    }

    pub fn mem_set_byte_indx(&mut self, addr: u8, b: u8) {
        self.set_byte(self.get_word(addr.wrapping_add(self.x) as u16), b);
    }

    pub fn mem_set_byte_indy(&mut self, addr: u8, b: u8) {
        self.set_byte(self.get_word(addr as u16).wrapping_add(self.y as u16), b);
    }

    // Modifiers

    pub fn mod_byte(&mut self, addr: u16, modifier: fn(u8) -> u8) {
    }

    pub fn mem_mod_byte_zpg(&mut self, addr: u8, modifier: fn(u8) -> u8) {
        self.mod_byte(addr as u16, modifier);
    }

    pub fn mem_mod_byte_zpgx(&mut self, addr: u8, modifier: fn(u8) -> u8) {
        self.mod_byte(addr as u16, modifier);
    }

    pub fn mem_mod_byte_abs(&mut self, addr: u16, modifier: fn(u8) -> u8) {
        self.mod_byte(addr as u16, modifier);
    }

    pub fn mem_mod_byte_absx(&mut self, addr: u16, modifier: fn(u8) -> u8) {
        self.mod_byte(addr as u16, modifier);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Memory tests

    #[test]
    fn test_mem_get_byte_zpg() {
        let mut cpu = CPU::new();
        cpu.set_byte(13, 42);
        assert_eq!(cpu.mem_get_byte_zpg(13), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgx() {
        let mut cpu = CPU::new();
        cpu.set_byte(13, 42);
        cpu.x = 8;
        assert_eq!(cpu.mem_get_byte_zpgx(5), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgx_wrapping() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x10, 0x42);
        cpu.x = 0x20;
        assert_eq!(cpu.mem_get_byte_zpgx(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_zpgy() {
        let mut cpu = CPU::new();
        cpu.set_byte(13, 42);
        cpu.y = 8;
        assert_eq!(cpu.mem_get_byte_zpgy(5), 42);
    }

    #[test]
    fn test_mem_get_byte_zpgy_wrapping() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x10, 0x42);
        cpu.y = 0x20;
        assert_eq!(cpu.mem_get_byte_zpgy(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indx() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x0432, 0x42);
        cpu.set_byte(5, 0x32);
        cpu.set_byte(6, 0x04);
        cpu.x = 2;
        assert_eq!(cpu.mem_get_byte_indx(3), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indx_wrapping() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x0432, 0x42);
        cpu.set_byte(0x20, 0x32);
        cpu.set_byte(0x21, 0x04);
        cpu.x = 0x30;
        assert_eq!(cpu.mem_get_byte_indx(0xf0), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indy() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x0444, 0x42);
        cpu.set_byte(5, 0x00);
        cpu.set_byte(6, 0x04);
        cpu.y = 0x44;
        assert_eq!(cpu.mem_get_byte_indy(5), 0x42);
    }

    #[test]
    fn test_mem_get_byte_indy_wrapping() {
        let mut cpu = CPU::new();
        cpu.set_byte(0x010, 0x42);
        cpu.set_byte(5, 0xf0);
        cpu.set_byte(6, 0xff);
        cpu.y = 0x20;
        assert_eq!(cpu.mem_get_byte_indy(5), 0x42);
    }
}

#[cfg(test)]
mod computer_tests {
    use super::*;

    #[test]
    fn test_initialized() {
        let cpu = CPU::new();
        assert_eq!(cpu.a, 0x00);
        assert_eq!(cpu.x, 0x00);
        assert_eq!(cpu.y, 0x00);
        assert_eq!(cpu.sp, 0xff);
        assert_eq!(cpu.pc, 0x0400);
        assert_eq!(cpu.ram.len(), 64*1024);
    }

    #[test]
    fn test_stack_bytes() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.sp, 0xff);

        cpu.push_byte(0x42);
        assert_eq!(cpu.sp, 0xfe);
        assert_eq!(cpu.get_byte(0x01ff), 0x42);

        cpu.push_byte(0x21);
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(cpu.get_byte(0x01fe), 0x21);

        assert_eq!(cpu.pull_byte(), 0x21);
        assert_eq!(cpu.sp, 0xfe);

        assert_eq!(cpu.pull_byte(), 0x42);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn test_stack_words() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.sp, 0xff);

        cpu.push_word(0x1234);
        assert_eq!(cpu.sp, 0xfd);
        assert_eq!(cpu.get_byte(0x01ff), 0x12);
        assert_eq!(cpu.get_byte(0x01fe), 0x34);

        cpu.push_word(0x6502);
        assert_eq!(cpu.sp, 0xfb);
        assert_eq!(cpu.get_byte(0x01fd), 0x65);
        assert_eq!(cpu.get_byte(0x01fc), 0x02);

        assert_eq!(cpu.pull_word(), 0x6502);
        assert_eq!(cpu.pull_word(), 0x1234);
    }

    #[test]
    fn test_breaks_without_program() {
        let mut cpu = CPU::new();
        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0400);
    }

    #[test]
    fn test_load_registers() {
        let program: Vec<u8> = vec![
            0xa9, 0x11, // $0400 LDA $11
            0xa2, 0x22, // $0402 LDX $22
            0xa0, 0x33, // $0404 LDY $33
            0x00        // $0406 BRK
        ];

        let mut cpu = CPU::new();
        cpu.load(0x0400, program);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);

        assert_eq!(cpu.a, 0x11);
        assert_eq!(cpu.x, 0x22);
        assert_eq!(cpu.y, 0x33);
        assert_eq!(cpu.sp, 0xff);
        assert_eq!(cpu.pc, 0x0406);
    }

    #[test]
    fn test_jmp_abs() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0x4C, 0x05, 0x04,   // $0400 JMP $0405
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x00                // $0407 BRK
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0407);
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn test_jmp_ind() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0x6C, 0x08, 0x04,   // $0400 JMP ($0408)
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x00,               // $0407 BRK
            0x05, 0x04          // $0408
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0407);
        assert_eq!(cpu.a, 0x42);
    }

    #[test]
    fn test_jsr_rts() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0x20, 0x05, 0x04,   // $0400 JSR $0405
            0x00,               // $0403 BRK
            0xEA,               // $0404 NOP
            0xA9, 0x42,         // $0405 LDA $42
            0x60,               // $0407 RTS
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0403);
        assert_eq!(cpu.a, 0x42);
        assert_eq!(cpu.sp, 0xff);
    }

    #[test]
    fn test_inx_wrapping() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0xA2, 0xFF,         // $0400 LDX #$FF
            0xE8,               // $0402 INX
            0x00,               // $0403 BRK
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0403);
        assert_eq!(cpu.x, 0x00);
    }

    #[test]
    fn test_dex_wrapping() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0xA2, 0x00,         // $0400 LDX #$00
            0xCA,               // $0402 DEX
            0x00,               // $0403 BRK
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0403);
        assert_eq!(cpu.x, 0xFF);
    }

    #[test]
    fn test_beq() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0xA2, 0x00,         // $0400 LDX #$00
            0xF0, 0x01,         // $0403 BEQ +1 ($0402)
            0x00,               // $0404 BRK
            0x00,               // $0405 BRK
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0405);
    }

    #[test]
    fn test_bne() {
        let mut cpu = CPU::new();
        cpu.load(0x0400, vec![
            0xA2, 0x00,         // $0400 LDX #$00
            0xD0, 0x01,         // $0403 BNE +1 ($0402)
            0x00,               // $0404 BRK
            0x00,               // $0405 BRK
        ]);

        assert_eq!(cpu.run().unwrap_err(), CPUError::Break);
        assert_eq!(cpu.pc, 0x0404);
    }
}

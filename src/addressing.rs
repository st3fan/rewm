// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use super::Computer;

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

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/

use super::Computer;

// "Micro" Operations

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


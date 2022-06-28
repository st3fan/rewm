// The MIT License (MIT)
//
// Copyright (c) 2022 Stefan Arentz - http://github.com/st3fan/rewm
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

mod ewm;
use ewm::{Computer, CPUError};

fn main() {
    let mut computer = Computer::new();
    match computer.run() {
        Ok(_) => { },
        Err(CPUError::IllegalOpcode) => {
            eprintln!("CPU Error: illegal opcode {} at {}", computer.get_byte(computer.cpu.pc), computer.cpu.pc);
            std::process::exit(1);
        }
        Err(CPUError::Break) => {
            eprintln!("CPU Error: break at {}", computer.cpu.pc);
            std::process::exit(1);
        }
    };
}


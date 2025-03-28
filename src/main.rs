mod assembler;
use assembler::assemble;
use std::{env, fs, process};

struct VM {
    ip: usize,        // 命令ポインタ
    regs: [i32; 4],   // レジスタ R0 ~ R3
    program: Vec<u8>, // 命令バイナリ
}

impl VM {
    fn new(program: Vec<u8>) -> Self {
        Self {
            ip: 0,
            regs: [0; 4],
            program,
        }
    }

    fn run(&mut self) {
        loop {
            let opcode = self.fetch_byte();

            match opcode {
                // MOV Rn, imm
                0x01 => {
                    let reg = self.fetch_byte() as usize;
                    let val = self.fetch_i8();
                    self.regs[reg] = val;
                }
                // ADD Rn, imm
                0x02 => {
                    let reg = self.fetch_byte() as usize;
                    let val = self.fetch_i8();
                    self.regs[reg] += val;
                }
                // PRINT Rn
                0x03 => {
                    let reg = self.fetch_byte() as usize;
                    println!("{}", self.regs[reg]);
                }
                // JMP addr
                0x10 => {
                    let addr = self.fetch_byte() as usize;
                    self.ip = addr;
                }
                // JZ reg, addr
                0x11 => {
                    let reg = self.fetch_byte() as usize;
                    let addr = self.fetch_byte() as usize;
                    if self.regs[reg] == 0 {
                        self.ip = addr;
                    }
                }
                // JNZ reg, addr
                0x12 => {
                    let reg = self.fetch_byte() as usize;
                    let addr = self.fetch_byte() as usize;
                    if self.regs[reg] != 0 {
                        self.ip = addr;
                    }
                }
                // HALT
                0xFF => {
                    break;
                }
                _ => {
                    panic!("Unknown opcode: 0x{:02X}", opcode);
                }
            }
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.program[self.ip];
        self.ip += 1;
        byte
    }

    fn fetch_i8(&mut self) -> i32 {
        let byte = self.fetch_byte();
        byte as i8 as i32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename.asm>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read program.asm");
    let program = assemble(&source);

    let mut vm = VM::new(program);
    vm.run();
}

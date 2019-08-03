use crate::display;
use crate::opcodes::OpCode;
use rand;

const PROGRAM_OFFSET: usize = 0x200;

pub struct CPU {
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pc: u16,
    pub gfx: display::Display,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [bool; 16],
    wait_for_key: Option<u8>,
}
impl CPU {
    pub fn new(game_data: &[u8]) -> Self {
        let mut memory = [0; 4096];
        for (i, byte) in display::FONTSET.iter().enumerate() {
            memory[i] = *byte;
        }
        for (i, byte) in game_data.iter().enumerate() {
            memory[i + PROGRAM_OFFSET] = *byte;
        }

        CPU {
            memory,
            v: [0; 16],
            i: 0,
            pc: PROGRAM_OFFSET as u16,
            gfx: display::Display::new(),
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [false; 16],
            wait_for_key: None,
        }
    }

    pub fn run_cycle(&mut self, dt: f64) {
        let num_to_run = (dt * 600.0).round() as u64;

        for _ in 1..num_to_run {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }

            if self.wait_for_key.is_none() {
                let raw_code = (((self.memory[self.pc as usize]) as u16) << 8) | (self.memory[(self.pc + 1) as usize] as u16);
                let opcode = OpCode::from_u16(raw_code).expect("Bad instruction");

                self.pc = self.run_instruction(opcode);
            }
        }
    }

    fn run_instruction(&mut self, instruction: OpCode) -> u16 {
        use OpCode::*;
        
        match instruction {
            ClearScreen => {
                self.gfx.clear();
                self.pc + 2
            },
            Return => {
                let prev = self.stack[(self.sp - 1) as usize];
                self.sp -= 1;
                prev + 2
            },
            JumpTo(addr) => addr,
            Call(addr) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                addr
            },
            SkipIfEqualsByte(reg, val) => {
                if self.read_reg(reg) == val {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            SkipIfNotEqualsByte(reg, val) => {
                if self.read_reg(reg) != val {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            SkipIfEquals(reg1, reg2) => {
                if self.read_reg(reg1) == self.read_reg(reg2) {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            LoadByte(reg, val) => {
                self.set_reg(reg, val);
                self.pc + 2
            },
            AddByte(reg, val) => {
                let reg_val = self.read_reg(reg);
                self.v[reg as usize] = reg_val.wrapping_add(val);
                self.pc + 2
            },
            Move(reg1, reg2) => {
                self.set_reg(reg1, self.read_reg(reg2));
                self.pc + 2
            },
            Or(reg1, reg2) => {
                let val1 = self.read_reg(reg1);
                let val2 = self.read_reg(reg2);
                self.set_reg(reg1, val1 | val2);
                self.pc + 2
            },
            And(reg1, reg2) => {
                let val1 = self.read_reg(reg1);
                let val2 = self.read_reg(reg2);
                self.set_reg(reg1, val1 & val2);
                self.pc + 2
            },
            XOr(reg1, reg2) => {
                let val1 = self.read_reg(reg1);
                let val2 = self.read_reg(reg2);
                self.set_reg(reg1, val1 ^ val2);
                self.pc + 2
            },
            Add(reg1, reg2) => {
                let val1 = self.read_reg(reg1) as u16;
                let val2 = self.read_reg(reg2) as u16;
                let sum = val1 + val2;
                self.set_reg(0xF, (sum > 255) as u8);
                self.set_reg(reg1, sum as u8);
                self.pc + 2
            },
            Sub(reg1, reg2) => {
                let val1 = self.read_reg(reg1);
                let val2 = self.read_reg(reg2);
                self.set_reg(0xF, (val1 > val2) as u8);
                self.set_reg(reg1, val1.wrapping_sub(val2));
                self.pc + 2
            },
            ShiftRight(reg) => {
                let val = self.read_reg(reg);
                self.set_reg(0xF, val & 0b1);
                self.set_reg(reg, val >> 1);
                self.pc + 2
            },
            ReverseSub(reg1, reg2) => {
                let val1 = self.read_reg(reg1);
                let val2 = self.read_reg(reg2);
                self.set_reg(0xF, (val2 > val1) as u8);
                self.set_reg(reg1, val2.wrapping_sub(val1));
                self.pc + 2
            },
            ShiftLeft(reg) => {
                let val = self.read_reg(reg);
                self.set_reg(0xF, val & 0b10000000);
                self.set_reg(reg, val << 1);
                self.pc + 2
            },
            SkipIfNotEquals(reg1, reg2) => {
                if self.read_reg(reg1) != self.read_reg(reg2) {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            LoadIReg(addr) => {
                self.i = addr;
                self.pc + 2
            },
            JumpPlus(addr) => addr + (self.read_reg(0x0) as u16),
            LoadRand(reg, val) => {
                self.set_reg(reg, rand::random::<u8>() & val);
                self.pc + 2
            },
            Draw(reg1, reg2, val) => {
                let x = self.read_reg(reg1);
                let y = self.read_reg(reg2);
                let start = self.i as usize;
                let end = start + (val as usize);

                self.v[0xF] = self.gfx.draw(x, y, &self.memory[start..end]) as u8;
                self.pc + 2
            },
            SkipPressed(reg) => {
                let val = self.read_reg(reg);
                let pressed = self.key[val as usize];

                if pressed {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            SkipNotPressed(reg) => {
                let val = self.read_reg(reg);
                let pressed = self.key[val as usize];

                if !pressed {
                    self.pc + 4
                } else {
                    self.pc + 2
                }
            },
            LoadDelay(reg) => {
                self.set_reg(reg, self.delay_timer);
                self.pc + 2
            },
            WaitForKey(reg) => {
                self.wait_for_key = Some(reg);
                self.pc + 2
            },
            SetDelay(reg) => {
                self.delay_timer = self.read_reg(reg);
                self.pc + 2
            },
            SetSoundDelay(reg) => {
                self.sound_timer = self.read_reg(reg);
                self.pc + 2
            },
            AddToIReg(reg) => {
                self.i += self.read_reg(reg) as u16;
                self.pc + 2
            },
            LoadSprite(reg) => {
                let val = self.read_reg(reg);
                self.i = (val * 5) as u16;
                self.pc + 2
            },
            StoreBCD(reg) => {
                let val = self.read_reg(reg);
                self.memory[self.i as usize] = val / 100;
                self.memory[(self.i as usize) + 1] = (val / 10) % 10;
                self.memory[(self.i as usize) + 2] = (val % 100) % 10;
                self.pc + 2
            },
            RegDump(reg) => {
                for i in 0x0..=reg {
                    self.memory[(self.i + i as u16) as usize] = self.read_reg(i);
                }
                self.pc + 2
            },
            RegLoad(reg) => {
                for i in 0x0..=reg {
                    self.set_reg(i, self.memory[(self.i + i as u16) as usize]);
                }
                self.pc + 2
            }
        }
    }

    pub fn key_press(&mut self, key: u8) {
        self.key[key as usize] = true;
        if let Some(reg) = self.wait_for_key {
            self.set_reg(reg, key);
            self.wait_for_key = None;
        }
    }

    pub fn key_release(&mut self, key: u8) {
        self.key[key as usize] = false;
    }

    fn read_reg(&self, reg_i: u8) -> u8 {
        self.v[reg_i as usize]
    }

    fn set_reg(&mut self, reg_i: u8, val: u8) {
        self.v[reg_i as usize] = val;
    }
}
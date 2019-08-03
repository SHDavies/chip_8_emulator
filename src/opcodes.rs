type Address = u16;
type Register = u8;

pub enum OpCode {
    ClearScreen,
    Return,
    JumpTo(Address),
    Call(Address),
    SkipIfEqualsByte(Register, u8),
    SkipIfNotEqualsByte(Register, u8),
    SkipIfEquals(Register, Register),
    LoadByte(Register, u8),
    AddByte(Register, u8),
    Move(Register, Register),
    Or(Register, Register),
    And(Register, Register),
    XOr(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    ShiftRight(Register),
    ReverseSub(Register, Register),
    ShiftLeft(Register),
    SkipIfNotEquals(Register, Register),
    LoadIReg(Address),
    JumpPlus(Address),
    LoadRand(Register, u8),
    Draw(Register, Register, u8),
    SkipPressed(Register),
    SkipNotPressed(Register),
    LoadDelay(Register),
    WaitForKey(Register),
    SetDelay(Register),
    SetSoundDelay(Register),
    AddToIReg(Register),
    LoadSprite(Register),
    StoreBCD(Register),
    RegDump(Register),
    RegLoad(Register),
}
impl OpCode {
    pub fn from_u16(raw_code: u16) -> Option<Self> {
        use OpCode::*;

        match xooo(raw_code) {
            0x0 => {
                match ooxx(raw_code) {
                    0xE0 => Some(ClearScreen),
                    0xEE => Some(Return),
                    _ => None,
                }
            },
            0x1 => Some(JumpTo(oxxx(raw_code))),
            0x2 => Some(Call(oxxx(raw_code))),
            0x3 => Some(SkipIfEqualsByte(oxoo(raw_code), ooxx(raw_code))),
            0x4 => Some(SkipIfNotEqualsByte(oxoo(raw_code), ooxx(raw_code))),
            0x5 => Some(SkipIfEquals(oxoo(raw_code), ooxo(raw_code))),
            0x6 => Some(LoadByte(oxoo(raw_code), ooxx(raw_code))),
            0x7 => Some(AddByte(oxoo(raw_code), ooxx(raw_code))),
            0x8 => {
                match ooox(raw_code) {
                    0x0 => Some(Move(oxoo(raw_code), ooxo(raw_code))),
                    0x1 => Some(Or(oxoo(raw_code), ooxo(raw_code))),
                    0x2 => Some(And(oxoo(raw_code), ooxo(raw_code))),
                    0x3 => Some(XOr(oxoo(raw_code), ooxo(raw_code))),
                    0x4 => Some(Add(oxoo(raw_code), ooxo(raw_code))),
                    0x5 => Some(Sub(oxoo(raw_code), ooxo(raw_code))),
                    0x6 => Some(ShiftRight(oxoo(raw_code))),
                    0x7 => Some(ReverseSub(oxoo(raw_code), ooxo(raw_code))),
                    0xE => Some(ShiftLeft(oxoo(raw_code))),
                    _ => None,
                }
            },
            0x9 => Some(SkipIfNotEquals(oxoo(raw_code), ooxo(raw_code))),
            0xA => Some(LoadIReg(oxxx(raw_code))),
            0xB => Some(JumpPlus(oxxx(raw_code))),
            0xC => Some(LoadRand(oxoo(raw_code), ooxx(raw_code))),
            0xD => Some(Draw(oxoo(raw_code), ooxo(raw_code), ooox(raw_code))),
            0xE => {
                match ooxx(raw_code) {
                    0x9E => Some(SkipPressed(oxoo(raw_code))),
                    0xA1 => Some(SkipNotPressed(oxoo(raw_code))),
                    _ => None,
                }
            },
            0xF => {
                match ooxx(raw_code) {
                    0x07 => Some(LoadDelay(oxoo(raw_code))),
                    0x0a => Some(WaitForKey(oxoo(raw_code))),
                    0x15 => Some(SetDelay(oxoo(raw_code))),
                    0x18 => Some(SetSoundDelay(oxoo(raw_code))),
                    0x1E => Some(AddToIReg(oxoo(raw_code))),
                    0x29 => Some(LoadSprite(oxoo(raw_code))),
                    0x33 => Some(StoreBCD(oxoo(raw_code))),
                    0x55 => Some(RegDump(oxoo(raw_code))),
                    0x65 => Some(RegLoad(oxoo(raw_code))),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[inline(always)]
fn xooo(code: u16) -> u8 {
    ((code >> 12) & 0xF) as u8
}
#[inline(always)]
fn oxoo(code: u16) -> u8 {
    ((code >> 8) & 0xF) as u8
}
#[inline(always)]
fn ooxo(code: u16) -> u8 {
    ((code >> 4) & 0xF) as u8
}
#[inline(always)]
fn ooox(code: u16) -> u8 {
    (code & 0xF) as u8
}
#[inline(always)]
fn ooxx(code: u16) -> u8 {
    (code & 0xFF) as u8
}
#[inline(always)]
fn oxxx(code: u16) -> u16 {
    code & 0xFFF
}
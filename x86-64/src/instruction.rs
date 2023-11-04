#[derive(Debug, Clone, Copy)]
pub enum Dest {
    Reg(u8),
    Mem(Addressing),
}

#[derive(Debug, Clone, Copy)]
pub enum Src {
    Reg(u8),
    Mem(Addressing),
    Imm(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Addressing {
    Displacement(u64),
    Base(u8),
    /// Base + Index
    BaseIndex(u8, u8),
    /// Base + Displacement
    BaseDisplacement(u8, u64),
    /// Base + Index + Displacement
    BaseIndexDisplacement(u8, u8, u64),
    /// Base + Index * Scale
    BaseIndexScale(u8, u8, u8),
    /// Index * Scale + Displacement
    IndexScaleDisplacement(u8, u8, u64),
    /// Base + Index * Scale + Displacement
    BaseIndexScaleDisplacement(u8, u8, u8, u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Mov(Dest, Src),
    Push(Src),
    Pop(Dest),

    Add(Dest, Src),
    Sub(Dest, Src),
    Inc(Dest),
    Dec(Dest),
    IMul(Dest, Src),
    IDiv(Src),

    And(Dest, Src),
    Or(Dest, Src),
    Xor(Dest, Src),
    Not(Dest),
    Neg(Dest),

    Cmp(Dest, Src),
    Test(Dest, Src),
    Jmp(Src),
    Je(Src),
    Jz(Src),
    Jnz(Src),
    Jg(Src),
    Jge(Src),
    Jl(Src),
    Jle(Src),
    Call(Src),
    Ret,
}

#[derive(Debug, Clone, Copy)]
pub enum Dest {
    Reg(u8),
    Mem(u64),
}

#[derive(Debug, Clone, Copy)]
pub enum Src {
    Reg(u8),
    Mem(u64),
    Imm(u64),
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

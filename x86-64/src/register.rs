use crate::simd::*;
use bitflags::bitflags;
use paste::paste;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Registers {
    gr: [u64; 16],
    rip: u64,
    rflags: Flags,
    simd: [AVX512Register; 16],
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags : u64 {
        const CARRY = 1 << 0;
        const PARITY = 1 << 2;
        const ADJUST = 1 << 4;
        const ZERO = 1 << 6;
        const SIGN = 1 << 7;
        const TRAP = 1 << 8;
        const INTERRUPT = 1 << 9;
        const DIRECTION = 1 << 10;
        const OVERFLOW = 1 << 11;
        const IOPL0 = 1 << 12;
        const IOPL1 = 1 << 13;
        const NESTED = 1 << 14;
        const RESUME = 1 << 16;
        const VIRTUAL8086 = 1 << 17;
        const ALIGNMENT = 1 << 18;
        const VIRTUALINTERRUPT = 1 << 19;
        const VIRTUALINTERRUPTPENDING = 1 << 20;
        const ID = 1 << 21;
        const AES = 1 << 30;
        const ALTERNATEINSTRUCTIONSET = 1 << 31;
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            gr: [0; 16],
            rip: 0,
            rflags: Flags::empty(),
            simd: [AVX512Register::new(); 16],
        }
    }

    pub fn rip(&self) -> u64 {
        self.rip
    }

    pub fn write_rip(&mut self, value: u64) {
        self.rip = value;
    }

    pub fn rflags(&self) -> &Flags {
        &self.rflags
    }

    pub fn rflags_mut(&mut self) -> &mut Flags {
        &mut self.rflags
    }

    pub fn xmm(&self, index: usize) -> XmmView {
        self.simd[index].xmm()
    }

    pub fn ymm(&self, index: usize) -> YmmView {
        self.simd[index].ymm()
    }

    pub fn zmm(&self, index: usize) -> ZmmView {
        self.simd[index].zmm()
    }

    pub fn xmm_mut(&mut self, index: usize) -> XmmViewMut {
        self.simd[index].xmm_mut()
    }

    pub fn ymm_mut(&mut self, index: usize) -> YmmViewMut {
        self.simd[index].ymm_mut()
    }

    pub fn zmm_mut(&mut self, index: usize) -> ZmmViewMut {
        self.simd[index].zmm_mut()
    }
}

macro_rules! define_rw {
    (
        $($type:ty = {
            $($name:ident $index:expr)*
        })*
    ) => {
        $(
            $(
                pub fn $name(&self) -> $type {
                    self.gr[$index] as $type
                }

                paste! {
                    pub fn [<write _ $name>](&mut self, value: $type) {
                        self.gr[$index] = value as u64;
                    }
                }
            )*
        )*
    };
}

impl Registers {
    define_rw! {
        u64 = {
            rax 0
            rcx 1
            rdx 2
            rbx 3
            rsp 4
            rbp 5
            rsi 6
            rdi 7
            r8 8
            r9 9
            r10 10
            r11 11
            r12 12
            r13 13
            r14 14
            r15 15
        }

        u32 = {
            eax 0
            ecx 1
            edx 2
            ebx 3
            esp 4
            ebp 5
            esi 6
            edi 7
            r8d 8
            r9d 9
            r10d 10
            r11d 11
            r12d 12
            r13d 13
            r14d 14
            r15d 15
        }

        u16 = {
            ax 0
            cx 1
            dx 2
            bx 3
            sp 4
            bp 5
            si 6
            di 7
            r8w 8
            r9w 9
            r10w 10
            r11w 11
            r12w 12
            r13w 13
            r14w 14
            r15w 15
        }

        u8 = {
            al 0
            cl 1
            dl 2
            bl 3
            spl 4
            bpl 5
            sil 6
            dil 7
            r8b 8
            r9b 9
            r10b 10
            r11b 11
            r12b 12
            r13b 13
            r14b 14
            r15b 15
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rw() {
        let mut reg = Registers::new();
        reg.write_rax(0x12_34_56_78_90_ab_cd_ef);
        assert_eq!(reg.rax(), 0x1234567890abcdef);
        assert_eq!(reg.eax(), 0x90abcdef);
        assert_eq!(reg.ax(), 0xcdef);
        assert_eq!(reg.al(), 0xef);
    }
}

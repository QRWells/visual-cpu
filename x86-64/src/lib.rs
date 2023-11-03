use cpu::CpuFeatures;
use paging::{MMU, PagingMode};

pub mod instruction;
pub mod paging;
pub mod register;
pub mod simd;

pub struct Cpu {
    mmu: MMU,
    device: Vec<Box<dyn cpu::Device>>,
}

impl cpu::Cpu for Cpu {
    fn run(&mut self) {
        todo!()
    }

    fn general_register_size(&self) -> usize {
        u64::BITS as usize / 8
    }

    fn endianness(&self) -> cpu::Endianness {
        cpu::Endianness::LittleEndian
    }

    fn add_device(&mut self, device: Box<dyn cpu::Device>) {
        self.device.push(device);
    }

    fn features(&self) -> cpu::CpuFeatures {
        CpuFeatures {
            simd: true,
            paging: self.mmu.paging_mode() != PagingMode::Real,
        }
    }
}

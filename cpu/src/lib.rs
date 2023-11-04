use thiserror::Error;

pub mod device;
pub mod simd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    LittleEndian,
    BigEndian,
}

pub struct CpuFeatures {
    pub simd: bool,
    pub paging: bool,
    pub unaligned_memory_access: bool,
}

pub trait Cpu {
    /// Runs the CPU
    fn run(&mut self);

    /// Returns the size of the general register in bytes
    ///
    /// (e.g. 32-bit register has size 4)
    fn general_register_size(&self) -> usize;

    /// The endianness of the CPU
    fn endianness(&self) -> Endianness;

    /// Add a device to the CPU
    fn add_device(&mut self, device: Box<dyn Device>);

    /// Returns if the CPU has virtual memory enabled
    fn features(&self) -> CpuFeatures;
}

pub trait Device: Addressable {
    /// Returns the name of the device
    fn name(&self) -> &str;
    /// Returns the start address of the device
    fn start_address(&self) -> usize;
    /// Returns the end address of the device
    fn end_address(&self) -> usize;
}

pub trait Addressable {
    /// Reads a byte from the addressable memory
    fn read_byte(&self, address: usize) -> Result<u8, MemoryAccessError>;
    /// Reads a slice of bytes from the addressable memory
    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], MemoryAccessError>;

    /// Writes a byte to the addressable memory
    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), MemoryAccessError>;
    /// Writes a slice of bytes to the addressable memory
    fn write_bytes(&mut self, address: usize, value: &[u8]) -> Result<(), MemoryAccessError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
    Execute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    Machine,
    Supervisor,
    User,
}

#[derive(Debug, Error)]
pub enum MemoryAccessError {
    /// The address is out of bounds
    #[error("Illegal memory access at address {address:#x} with size {size}")]
    OutOfBounds { address: usize, size: usize },

    /// The address is not aligned
    #[error("The address {address:#x} is not aligned")]
    Unaligned { address: usize },

    /// The address is not mapped
    #[error("The address is not mapped")]
    AddressNotMapped,

    /// The address is already mapped
    #[error("The address {address:#x} is already mapped")]
    AddressAlreadyMapped { address: usize },
}

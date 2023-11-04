use std::collections::BTreeMap;

use crate::{Addressable, Device, MemoryAccessError};

/// A generic DRAM device
#[derive(Debug, Clone)]
pub struct DRAM {
    /// The data of the DRAM
    segments: BTreeMap<usize, Segment>,
    /// The base address of the DRAM
    base_address: usize,
    /// The size of the DRAM
    size: usize,
    /// The alignment of the DRAM, defaults to 1 (byte aligned)
    alignment: usize,
}

const BLOCK_SIZE: usize = 1 << 12; // 4 KiB
const DEFAULT_ALLOC_SIZE: usize = 1 << 12; // 4 KiB

#[derive(Debug, Clone)]
struct Segment {
    start: usize,
    data: Vec<u8>,
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Eq for Segment {}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.start.partial_cmp(&other.start)
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.data == other.data
    }
}

impl Addressable for DRAM {
    fn read_byte(&self, address: usize) -> Result<u8, crate::MemoryAccessError> {
        self.check_access(address, 1)?;
        for segment in self.segments.values() {
            if segment.contains(address) {
                return segment.read_byte(address);
            }
        }

        Err(crate::MemoryAccessError::AddressNotMapped)
    }

    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], crate::MemoryAccessError> {
        self.check_access(address, 1)?;
        for segment in self.segments.values() {
            if segment.contains(address) {
                return segment.read_bytes(address, size);
            }
        }
        Err(crate::MemoryAccessError::AddressNotMapped)
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), crate::MemoryAccessError> {
        self.check_access(address, 1)?;
        for segment in self.segments.values_mut() {
            if segment.contains(address) {
                return segment.write_byte(address, value);
            }
        }
        Err(crate::MemoryAccessError::AddressNotMapped)
    }

    fn write_bytes(
        &mut self,
        address: usize,
        value: &[u8],
    ) -> Result<(), crate::MemoryAccessError> {
        self.check_access(address, 1)?;
        for segment in self.segments.values_mut() {
            if segment.contains(address) {
                return segment.write_bytes(address, value);
            }
        }
        Err(crate::MemoryAccessError::AddressNotMapped)
    }
}

impl Device for DRAM {
    fn name(&self) -> &str {
        "DRAM"
    }

    fn start_address(&self) -> usize {
        self.base_address
    }

    fn end_address(&self) -> usize {
        self.base_address + self.size
    }
}

impl DRAM {
    pub fn new(base_address: usize, size: usize) -> Self {
        Self {
            segments: Default::default(),
            base_address,
            size: size.next_power_of_two(),
            alignment: 1,
        }
    }

    /// Allocate a new segment of memory
    pub fn alloc(
        &mut self,
        address_hint: usize,
        size_hint: usize,
    ) -> Result<usize, MemoryAccessError> {
        let address = address_hint & !(BLOCK_SIZE - 1);

        if size_hint > self.size {
            return Err(MemoryAccessError::OutOfBounds {
                address: address_hint,
                size: size_hint,
            });
        }

        let size = size_hint + (address_hint & (DEFAULT_ALLOC_SIZE - 1));
        let size = if size < DEFAULT_ALLOC_SIZE {
            DEFAULT_ALLOC_SIZE
        } else {
            size.next_power_of_two()
        };

        if self.segments.is_empty() {
            let segment = Segment {
                start: address,
                data: vec![0; size],
            };
            self.segments.insert(address, segment);
            return Ok(address);
        }

        // check if the address is already mapped
        for (addr, seg) in self.segments.iter_mut() {
            if seg.contains(address) {
                return Err(MemoryAccessError::AddressAlreadyMapped { address: *addr });
            }

            // extend the segment if the address is at the end
            if seg.end() == address {
                seg.data.resize(seg.data.len() + size, 0);
                return Ok(*addr);
            }

            if address < *addr && address + size >= *addr {
                let size = size.max(seg.end() - address);
                let mut data = vec![0; size];
                // copy the data from the old segment
                data[(addr - address)..].copy_from_slice(&seg.data);
                seg.data = data;
                seg.start = address;
                return Ok(*addr);
            }
        }

        let segment = Segment {
            start: address,
            data: vec![0; size],
        };

        self.segments.insert(address, segment);

        Ok(address)
    }

    fn check_access(&self, address: usize, size: usize) -> Result<(), crate::MemoryAccessError> {
        if address % self.alignment != 0 {
            return Err(crate::MemoryAccessError::Unaligned { address });
        }

        if address < self.base_address || address >= self.base_address + self.size {
            return Err(crate::MemoryAccessError::OutOfBounds { address, size });
        }

        Ok(())
    }
}

impl Segment {
    pub(crate) fn end(&self) -> usize {
        self.start + self.data.len()
    }

    pub(crate) fn contains(&self, address: usize) -> bool {
        address >= self.start && address < self.end()
    }

    pub(crate) fn offset(&self, address: usize) -> usize {
        address - self.start
    }
}

impl Addressable for Segment {
    fn read_byte(&self, address: usize) -> Result<u8, crate::MemoryAccessError> {
        assert!(self.contains(address));
        let offset = self.offset(address);
        Ok(self.data[offset])
    }

    fn read_bytes(&self, address: usize, size: usize) -> Result<&[u8], crate::MemoryAccessError> {
        assert!(self.contains(address));
        let offset = self.offset(address);
        Ok(&self.data[offset..offset + size])
    }

    fn write_byte(&mut self, address: usize, value: u8) -> Result<(), crate::MemoryAccessError> {
        assert!(self.contains(address));
        let offset = self.offset(address);
        self.data[offset] = value;
        Ok(())
    }

    fn write_bytes(
        &mut self,
        address: usize,
        value: &[u8],
    ) -> Result<(), crate::MemoryAccessError> {
        assert!(self.contains(address));
        let offset = self.offset(address);
        self.data[offset..offset + value.len()].copy_from_slice(value);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_dram_alloc() {
        let mut dram = DRAM::new(0, 4 * 1024 * 1024);
        let addr = dram.alloc(0, 1024).unwrap();
        assert_eq!(addr, 0);
        let addr = dram.alloc(0, 1024);
        assert!(addr.is_err());
        let addr = dram.read_byte(4097);
        assert!(addr.is_err());
        let addr = dram.alloc(16 * 1024, 1024).unwrap();
        assert_eq!(addr, 16 * 1024);
        dram.write_byte(16 * 1024 + 1, 0x42).unwrap();
        let res = dram.read_byte(16 * 1024 + 1).unwrap();
        assert_eq!(res, 0x42);
    }
}
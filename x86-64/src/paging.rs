#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PagingMode {
    Real,
    Protected,
    Long,
    LongLA57,
}

pub struct MMU {
    paging_mode: PagingMode,
}

impl MMU {
    pub fn paging_mode(&self) -> PagingMode {
        self.paging_mode
    }
}

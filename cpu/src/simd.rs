use std::ops::Index;

use num_traits::Num;
use paste::paste;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simd<T: Num + Copy, const N: usize>([T; N]);

impl <T: Num + Copy, const N: usize> Simd<T, N> {
    pub fn new() -> Self {
        Self([T::zero(); N])
    }

    pub fn from_array(array: [T; N]) -> Self {
        Self(array)
    }
}

impl <T: Num + Copy, const N: usize> Index<usize> for Simd<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

macro_rules! alias {
    {
        $(
            $element_ty:ty = {
                $(
                    $num_elements:literal
                ),*}
        )*
    } => {
        paste! {
            $(
                $(
                    #[allow(non_camel_case_types)]
                    pub type [<$element_ty x $num_elements>] = $crate::simd::Simd<$element_ty, $num_elements>;
                )*
            )*
        }
    }
}

alias! {
    i8 = { 1, 2, 4, 8, 16, 32, 64 }
    i16 = { 1, 2, 4, 8, 16, 32 }
    i32 = { 1, 2, 4, 8, 16 }
    i64 = { 1, 2, 4, 8 }
    isize = { 1, 2, 4, 8, 16, 32, 64 }
    u8 = { 1, 2, 4, 8, 16, 32, 64 }
    u16 = { 1, 2, 4, 8, 16, 32 }
    u32 = { 1, 2, 4, 8, 16 }
    u64 = { 1, 2, 4, 8 }
    usize = { 1, 2, 4, 8, 16, 32, 64 }
    f32 = { 1, 2, 4, 8, 16 }
    f64 = { 1, 2, 4, 8 }
}

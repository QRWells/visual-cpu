use cpu::simd::*;
use num_traits::Zero;
use paste::paste;

/// Single AVX512 register
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AVX512Register {
    data: [u8; 64],
}

impl AVX512Register {
    pub fn new() -> Self {
        Self { data: [0; 64] }
    }

    pub fn zmm_mut(&mut self) -> ZmmViewMut {
        ZmmViewMut {
            data: &mut self.data[..],
        }
    }

    pub fn ymm_mut(&mut self) -> YmmViewMut {
        YmmViewMut {
            data: &mut self.data[0..32],
        }
    }

    pub fn xmm_mut(&mut self) -> XmmViewMut {
        XmmViewMut {
            data: &mut self.data[0..16],
        }
    }

    pub fn zmm(&self) -> ZmmView {
        ZmmView {
            data: &self.data[..],
        }
    }

    pub fn ymm(&self) -> YmmView {
        YmmView {
            data: &self.data[0..32],
        }
    }

    pub fn xmm(&self) -> XmmView {
        XmmView {
            data: &self.data[0..16],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZmmView<'a> {
    data: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YmmView<'a> {
    data: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XmmView<'a> {
    data: &'a [u8],
}

#[derive(Debug)]
pub struct ZmmViewMut<'a> {
    data: &'a mut [u8],
}

#[derive(Debug)]
pub struct YmmViewMut<'a> {
    data: &'a mut [u8],
}

#[derive(Debug)]
pub struct XmmViewMut<'a> {
    data: &'a mut [u8],
}

impl ZmmView<'_> {
    pub fn as_ymm(&self) -> YmmView {
        YmmView {
            data: &self.data[0..32],
        }
    }

    pub fn as_xmm(&self) -> XmmView {
        XmmView {
            data: &self.data[0..16],
        }
    }
}

impl YmmView<'_> {
    pub fn as_xmm(&self) -> XmmView {
        XmmView {
            data: &self.data[0..16],
        }
    }
}

macro_rules! read {
    (
        $($struct:ident = {
            $(
                $element_ty:ty = {
                    $(
                        $num_elements:literal
                    ),*
                }
            )*
        })*
    ) => {
        paste! {
            $(
                impl $struct<'_> {
                    $(
                        $(
                            pub fn [<read_ $element_ty x $num_elements>](&self) -> [<$element_ty x $num_elements>] {
                                let mut result = [$element_ty::zero(); $num_elements];
                                for i in 0..$num_elements {
                                    result[i] = self.data[i * std::mem::size_of::<$element_ty>()] as $element_ty;
                                }
                                [<$element_ty x $num_elements>]::from_array(result)
                            }
                        )*
                    )*
                }
            )*
        }
    }
}

macro_rules! write {
    (
        $($struct:ident = {
            $(
                $element_ty:ty = {
                    $(
                        $num_elements:literal
                    ),*
                }
            )*
        })*
    ) => {
        paste! {
            $(
                impl $struct<'_> {
                    $(
                        $(
                            pub fn [<write_ $element_ty x $num_elements>](&mut self, value: [<$element_ty x $num_elements>]) {
                                for i in 0..$num_elements {
                                    self.data[i * std::mem::size_of::<$element_ty>()] = value[i] as u8;
                                }
                            }

                            pub fn [<write_ $element_ty x $num_elements _array>](&mut self, value: [$element_ty; $num_elements]) {
                                for i in 0..$num_elements {
                                    self.data[i * std::mem::size_of::<$element_ty>()] = value[i] as u8;
                                }
                            }
                        )*
                    )*
                }
            )*
        }
    }
}

macro_rules! read_write {
    (
        $($struct:ident = {
            $(
                $element_ty:ty = {
                    $(
                        $num_elements:literal
                    ),*
                }
            )*
        })*
    ) => {
        read! {
            $($struct = {
                $(
                    $element_ty = {
                        $(
                            $num_elements
                        ),*
                    }
                )*
            })*
        }

        write! {
            $($struct = {
                $(
                    $element_ty = {
                        $(
                            $num_elements
                        ),*
                    }
                )*
            })*
        }
    }
}

read! {
    XmmView = {
        i8 = { 1, 2, 4, 8, 16 }
        i16 = { 1, 2, 4, 8 }
        i32 = { 1, 2, 4 }
        i64 = { 1, 2 }
        u8 = { 1, 2, 4, 8, 16 }
        u16 = { 1, 2, 4, 8 }
        u32 = { 1, 2, 4 }
        u64 = { 1, 2 }
        f32 = { 1, 2, 4 }
        f64 = { 1, 2 }
    }

    YmmView = {
        i8 = { 1, 2, 4, 8, 16, 32 }
        i16 = { 1, 2, 4, 8, 16 }
        i32 = { 1, 2, 4, 8 }
        i64 = { 1, 2, 4 }
        u8 = { 1, 2, 4, 8, 16, 32 }
        u16 = { 1, 2, 4, 8, 16 }
        u32 = { 1, 2, 4, 8 }
        u64 = { 1, 2, 4 }
        f32 = { 1, 2, 4, 8 }
        f64 = { 1, 2, 4 }
    }

    ZmmView = {
        i8 = { 1, 2, 4, 8, 16, 32, 64 }
        i16 = { 1, 2, 4, 8, 16, 32 }
        i32 = { 1, 2, 4, 8, 16 }
        i64 = { 1, 2, 4, 8 }
        u8 = { 1, 2, 4, 8, 16, 32, 64 }
        u16 = { 1, 2, 4, 8, 16, 32 }
        u32 = { 1, 2, 4, 8, 16 }
        u64 = { 1, 2, 4, 8 }
        f32 = { 1, 2, 4, 8, 16 }
        f64 = { 1, 2, 4, 8 }
    }
}

read_write! {
    XmmViewMut = {
        i8 = { 1, 2, 4, 8, 16 }
        i16 = { 1, 2, 4, 8 }
        i32 = { 1, 2, 4 }
        i64 = { 1, 2 }
        u8 = { 1, 2, 4, 8, 16 }
        u16 = { 1, 2, 4, 8 }
        u32 = { 1, 2, 4 }
        u64 = { 1, 2 }
        f32 = { 1, 2, 4 }
        f64 = { 1, 2 }
    }

    YmmViewMut = {
        i8 = { 1, 2, 4, 8, 16, 32 }
        i16 = { 1, 2, 4, 8, 16 }
        i32 = { 1, 2, 4, 8 }
        i64 = { 1, 2, 4 }
        u8 = { 1, 2, 4, 8, 16, 32 }
        u16 = { 1, 2, 4, 8, 16 }
        u32 = { 1, 2, 4, 8 }
        u64 = { 1, 2, 4 }
        f32 = { 1, 2, 4, 8 }
        f64 = { 1, 2, 4 }
    }

    ZmmViewMut = {
        i8 = { 1, 2, 4, 8, 16, 32, 64 }
        i16 = { 1, 2, 4, 8, 16, 32 }
        i32 = { 1, 2, 4, 8, 16 }
        i64 = { 1, 2, 4, 8 }
        u8 = { 1, 2, 4, 8, 16, 32, 64 }
        u16 = { 1, 2, 4, 8, 16, 32 }
        u32 = { 1, 2, 4, 8, 16 }
        u64 = { 1, 2, 4, 8 }
        f32 = { 1, 2, 4, 8, 16 }
        f64 = { 1, 2, 4, 8 }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: Add tests
    #[test]
    fn test() {
        let mut reg = AVX512Register::new();
        reg.xmm_mut()
            .write_i8x16_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6]);
        assert_eq!(
            reg.xmm().read_i8x16(),
            i8x16::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -1, -2, -3, -4, -5, -6])
        );
    }
}

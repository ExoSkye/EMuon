pub type Register = u8;
pub type Address = u16;
pub type Byte = u8;
pub type DualWord = u16;
pub type Nibble = u8;

pub trait BinOps {
    fn get_bit(&self, bit: u32) -> u8;
    fn get_nibble(&self, nibble: u8) -> u8;
    fn get_byte(&self, byte: u8) -> u8;
}

pub trait BinOpsMut {
    fn set_bit(&mut self, bit: u32, value: u8);
    fn set_nibble(&mut self, nibble: u8, value: u8);
    fn set_byte(&mut self, byte: u8, value: u8);
}

pub trait BinOpsBCD {
    fn get_bcd(&self) -> i8;
}

pub trait BinOpsBCDMut {
    fn set_bcd(&mut self, value: i8);
}

macro_rules! impl_binops {
    ($t:ty) => {
        impl BinOps for $t {
            fn get_bit(&self, bit: u32) -> u8 {
                assert!(bit < <$t>::BITS);
                ((*self >> bit) & 1) as u8
            }
            
            fn get_nibble(&self, nibble: u8) -> u8 {
                assert!(nibble < ((<$t>::BITS / 4) as u8));
                ((*self >> (nibble * 4)) & 0xF) as u8
            }

            fn get_byte(&self, byte: u8) -> u8 {
                assert!(byte < ((<$t>::BITS / 8) as u8));
                ((*self >> (byte * 8)) & 0xFF) as u8
            }
        }

        impl BinOpsMut for $t {
            fn set_bit(&mut self, bit: u32, value: u8) {
                assert!(bit < <$t>::BITS);
                assert!(value == 0 || value == 1);
                *self = (*self & !(1 << bit)) | ((value as $t) << bit);
            }

            fn set_nibble(&mut self, nibble: u8, value: u8) {
                assert!(nibble < ((<$t>::BITS / 4) as u8));
                assert!(value < 16);
                *self = (*self & !(0xF << (nibble * 4))) | ((value as $t) << (nibble * 4));
            }

            fn set_byte(&mut self, byte: u8, value: u8) {
                assert!(byte < ((<$t>::BITS / 8) as u8));
                *self = (*self & !(0xFF << (byte * 8))) | ((value as $t) << (byte * 8));
            }
        }
    }
}

macro_rules! impl_bcdops {
    ($t:ty) => {
        impl BinOpsBCD for $t {
            fn get_bcd(&self) -> i8 {
                assert!(<$t>::BITS == 8);
                let mut value = *self as i8;
                if value > 99 {
                    value -= 100;
                }
                value
            }
        }

        impl BinOpsBCDMut for $t {
            fn set_bcd(&mut self, value: i8) {
                assert!(<$t>::BITS == 8);
                let mut value = value;
                if value < 0 {
                    value += 100;
                }
                *self = value as $t;
            }
        }
    };
}

impl_binops!(u8);
impl_bcdops!(u8);
impl_binops!(u16);
impl_binops!(u32);
impl_binops!(u64);
impl_binops!(usize);
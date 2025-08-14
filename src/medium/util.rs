use bit_register::{NumBytes, TryFromBits, TryIntoBits};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Zero;

impl TryFromBits<u32> for Zero {
    fn try_from_bits(bits: u32) -> Result<Self, &'static str> {
        if bits != 0 {
            Err("Bits must be 0")
        } else {
            Ok(Zero)
        }
    }
}
impl TryIntoBits<u32> for Zero {
    fn try_into_bits(self) -> Result<u32, &'static str> {
        Ok(0)
    }
}
impl NumBytes for Zero {
    const NUM_BYTES: usize = 4;
}

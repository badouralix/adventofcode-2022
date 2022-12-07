use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

pub mod shell;

#[derive(Clone, Copy)]
pub struct BitSet<const N: usize> {
    bits: [u64; N],
}

impl<const N: usize> BitSet<N> {
    pub fn test(&self, n: impl Into<usize>) -> bool {
        let p = n.into();
        self.bits[p / 64] & (1 << (p % 64)) > 0
    }

    pub fn set(&mut self, n: impl Into<usize>) {
        let p = n.into();
        self.bits[p / 64] |= 1 << (p % 64)
    }
    pub fn reset(&mut self, n: impl Into<usize>) {
        let p = n.into();
        self.bits[p / 64] &= !(1 << (p % 64))
    }

    pub fn count_ones(&self) -> u32 {
        let mut res = 0;
        for x in self.bits {
            res += x.count_ones();
        }
        res
    }

    pub fn first_set(&self) -> u32 {
        let mut res = 0;
        for x in self.bits {
            if x > 0 {
                res += x.trailing_zeros();
                return res;
            }
            res += 64;
        }
        res
    }

    pub fn ones() -> Self {
        Self { bits: [!0; N] }
    }
}

impl<const N: usize> Default for BitSet<N> {
    fn default() -> Self {
        Self { bits: [0; N] }
    }
}

impl<const N: usize> BitAnd for BitSet<N> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..N {
            res.bits[i] = self.bits[i] & rhs.bits[i];
        }
        res
    }
}

impl<const N: usize> BitAndAssign for BitSet<N> {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.bits[i] &= rhs.bits[i];
        }
    }
}

impl<const N: usize> BitOr for BitSet<N> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..N {
            res.bits[i] = self.bits[i] | rhs.bits[i];
        }
        res
    }
}

impl<const N: usize> BitOrAssign for BitSet<N> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self.bits[i] |= rhs.bits[i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let mut set = BitSet::<3>::default();
        set.set(75u8);
        assert_eq!(set.first_set(), 75);
        set.set(36u8);
        assert_eq!(set.first_set(), 36);
        set.set(141u8);
        assert_eq!(set.first_set(), 36);
        set.reset(36u8);
        assert_eq!(set.first_set(), 75);
    }
}

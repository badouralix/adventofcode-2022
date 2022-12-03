use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Clone, Copy)]
pub struct BitSet<const N: usize> {
    bits: [u64; N],
}

impl<const N: usize> BitSet<N> {
    pub fn test(&self, n: usize) -> bool {
        self.bits[n / 64] & (1 << (63 - (n % 64))) > 0
    }

    pub fn set(&mut self, n: usize) {
        self.bits[n / 64] |= 1 << (63 - (n % 64))
    }

    pub fn count_ones(&self) -> u32 {
        let mut res = 0;
        for x in self.bits {
            res += x.count_ones();
        }
        res
    }

    pub fn leading_zeros(&self) -> u32 {
        let mut res = 0;
        for x in self.bits {
            if x > 0 {
                res += x.leading_zeros();
                return res;
            }
            res += 64;
        }
        res
    }

    pub fn first_set(&self) -> u32 {
        let mut res = 0;
        for x in self.bits {
            if x > 0 {
                res += x.leading_zeros();
                return res;
            }
            res += 64;
        }
        res
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

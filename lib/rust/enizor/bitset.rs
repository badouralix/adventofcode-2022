use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

pub const fn bitset_size(n: usize) -> usize {
    1 + (n / 64)
}

pub type ArrayBitSet<const N: usize> = BitSet<[u64; N]>;

impl<const N: usize> ArrayBitSet<N> {
    pub fn new() -> Self {
        Self { bits: [0; N] }
    }

    pub fn ones() -> Self {
        Self { bits: [!0; N] }
    }
}

impl<const N: usize> Default for ArrayBitSet<N> {
    fn default() -> Self {
        Self::new()
    }
}

pub type VecBitSet = BitSet<Vec<u64>>;

impl VecBitSet {
    pub fn new(n: impl Into<usize>) -> Self {
        Self {
            bits: vec![0; n.into()],
        }
    }

    pub fn ones(n: impl Into<usize>) -> Self {
        Self {
            bits: vec![!0; n.into()],
        }
    }
}

pub struct BitSet<T: AsMut<[u64]> + AsRef<[u64]>> {
    bits: T,
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> BitSet<T> {
    pub fn test(&self, n: impl Into<usize>) -> bool {
        let p = n.into();
        self.bits.as_ref()[p / 64] & (1 << (p % 64)) > 0
    }

    pub fn set(&mut self, n: impl Into<usize>) {
        let p = n.into();
        self.bits.as_mut()[p / 64] |= 1 << (p % 64)
    }
    pub fn reset(&mut self, n: impl Into<usize>) {
        let p = n.into();
        self.bits.as_mut()[p / 64] &= !(1 << (p % 64))
    }

    pub fn count_ones(&self) -> u32 {
        let mut res = 0;
        for &x in self.bits.as_ref() {
            res += x.count_ones();
        }
        res
    }

    pub fn first_set(&self) -> u32 {
        let mut res = 0;
        for &x in self.bits.as_ref() {
            if x > 0 {
                res += x.trailing_zeros();
                return res;
            }
            res += 64;
        }
        res
    }
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> BitAnd for BitSet<T> {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> BitAndAssign for BitSet<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        for i in 0..self.bits.as_ref().len() {
            self.bits.as_mut()[i] &= rhs.bits.as_ref()[i];
        }
    }
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> BitOr for BitSet<T> {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl<T: AsMut<[u64]> + AsRef<[u64]>> BitOrAssign for BitSet<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        for i in 0..self.bits.as_ref().len() {
            self.bits.as_mut()[i] |= rhs.bits.as_ref()[i];
        }
    }
}

/// A 2D-addressable bitset of width W length L
/// with x spanning [W_0 , W + W_O)
/// with y spanning [L_0 , L + W_O)
/// N must be >= bitset_size(L*W)
#[derive(Default)]
pub struct GridBitSet<
    const N: usize,
    const W: usize,
    const W_O: isize,
    const L: usize,
    const L_O: isize,
> {
    pub bitset: ArrayBitSet<N>,
}

impl<const N: usize, const W: usize, const W_O: isize, const L: usize, const L_O: isize>
    GridBitSet<N, W, W_O, L, L_O>
{
    pub fn new() -> Self {
        debug_assert!(N >= bitset_size(W * L));
        Self {
            bitset: ArrayBitSet::new(),
        }
    }

    fn pos((x, y): (isize, isize)) -> usize {
        debug_assert!(x - W_O >= 0);
        debug_assert!(W > (x - W_O) as usize);
        debug_assert!(y - L_O >= 0);
        debug_assert!(L > (y - L_O) as usize);
        (y - L_O) as usize * W + (x - W_O) as usize
    }

    pub fn test(&self, p: (isize, isize)) -> bool {
        self.bitset.test(Self::pos(p))
    }

    pub fn set(&mut self, p: (isize, isize)) {
        self.bitset.set(Self::pos(p))
    }
    pub fn reset(&mut self, p: (isize, isize)) {
        self.bitset.reset(Self::pos(p))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let mut set = BitSet::<Vec<u64>>::new(250u8);
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

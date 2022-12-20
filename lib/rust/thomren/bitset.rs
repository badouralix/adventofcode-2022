#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitSet64 {
    x: u64,
}

impl BitSet64 {
    pub fn new() -> Self {
        Self { x: 0 }
    }

    pub fn add(&mut self, n: usize) {
        if n >= 64 {
            panic!("BitSet64 can only contain values up to 63")
        }
        self.x |= 1 << n;
    }

    pub fn contains(&self, n: usize) -> bool {
        if n >= 64 {
            return false;
        }
        (self.x >> n) & 1 == 1
    }

    pub fn difference(&self, other: BitSet64) -> Self {
        Self {
            x: self.x & !other.x,
        }
    }

    pub fn iter(&self) -> BitSet64Iter {
        BitSet64Iter { bs: *self, cur: 0 }
    }
}

impl Default for BitSet64 {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<usize> for BitSet64 {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut res = Self { x: 0 };
        for n in iter {
            res.add(n);
        }
        res
    }
}
pub struct BitSet64Iter {
    bs: BitSet64,
    cur: usize,
}

impl Iterator for BitSet64Iter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.bs.contains(self.cur) && self.cur < 64 {
            self.cur += 1;
        }
        if self.cur == 64 {
            None
        } else {
            self.cur += 1;
            Some(self.cur - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_add_and_retrieve() {
        let mut bs = BitSet64::new();
        assert!(!bs.contains(0));
        assert!(!bs.contains(5));
        assert!(!bs.contains(63));
        bs.add(0);
        assert!(bs.contains(0));
        bs.add(1);
        bs.add(5);
        bs.add(63);
        assert!(bs.contains(1));
        assert!(bs.contains(5));
        assert!(bs.contains(63));
    }

    #[test]
    fn it_can_create_from_iter() {
        let bs = BitSet64::from_iter([0, 2, 5, 7, 14].into_iter());
        assert!(bs.contains(0));
        assert!(bs.contains(2));
        assert!(bs.contains(5));
        assert!(bs.contains(14));
        assert!(!bs.contains(13));
        assert!(!bs.contains(15));
    }

    #[test]
    fn it_can_iter() {
        let items = vec![0, 2, 5, 7, 14];
        let bs = BitSet64::from_iter(items.iter().cloned());
        let actual = bs.iter().collect::<Vec<usize>>();
        assert_eq!(items, actual);
    }
}

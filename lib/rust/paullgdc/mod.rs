pub struct LastElem<I: Iterator> {
    it: std::iter::Peekable<I>,
}

impl<I: Iterator> LastElem<I> {
    pub fn new(it: I) -> LastElem<I> {
        LastElem { it: it.peekable() }
    }
}

impl<F, I> Iterator for LastElem<I>
where
    I: Iterator<Item = F>,
{
    type Item = (F, bool);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.it.next()?;
        if self.it.peek().is_none() {
            Some((next, true))
        } else {
            Some((next, false))
        }
    }
}

#[inline(always)]
pub fn get_mut_2<T>(slice: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i == j || i >= slice.len() || j >= slice.len() {
        return None;
    }
    unsafe {
        let i = &mut *slice.as_mut_ptr().add(i);
        let j = &mut *slice.as_mut_ptr().add(j);
        Some((i, j))
    }
}

pub mod tokenize {

    pub fn parse_decimal_u32(input: &[u8]) -> Option<u32> {
        let mut res = 0;
        if !input.first()?.is_ascii_digit() {
            return None;
        }
        for b in input.iter() {
            match b {
                b'0'..=b'9' => {
                    res *= 10;
                    res += (b - b'0') as u32;
                }
                _ => {
                    break;
                }
            }
        }
        Some(res)
    }

    pub struct Tokenizer<'a> {
        input: &'a [u8],
        pos: usize,
    }

    impl<'a> Tokenizer<'a> {
        pub fn new(input: &'a [u8]) -> Self {
            Self { input, pos: 0 }
        }

        pub fn left(&self) -> &[u8] {
            &self.input[self.pos..]
        }

        pub fn next_ascii_char(&mut self) -> Option<u8> {
            let b = self.next_nth_byte(0)?;
            self.advance(1);
            if b.is_ascii() {
                Some(b)
            } else {
                None
            }
        }

        pub fn parse_next_decimal_u8(&mut self) -> Option<u8> {
            let mut res = 0;
            let mut skip = 0;
            for (i, b) in self.input[self.pos..].iter().enumerate() {
                match b {
                    b'0'..=b'9' => {
                        res *= 10;
                        res += b - b'0';
                        skip = i + 1;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if skip == 0 {
                return None;
            }
            self.pos += skip;
            Some(res)
        }

        pub fn parse_next_decimal_i32(&mut self) -> Option<i32> {
            let mul = if *self.input.get(self.pos)? == b'-' {
                self.advance(1);
                -1
            } else {
                1
            };
            let mut skip = 0;
            let mut res = 0;
            for (i, b) in self.input[self.pos..].iter().enumerate() {
                match b {
                    b'0'..=b'9' => {
                        res *= 10;
                        res += (b - b'0') as i32;
                        skip = i + 1;
                    }
                    _ => {
                        break;
                    }
                }
            }
            if skip == 0 {
                return None;
            }
            self.pos += skip;
            Some(res * mul)
        }

        pub fn next_nth_byte(&self, n: usize) -> Option<u8> {
            self.input.get(self.pos + n).copied()
        }

        pub fn eat_chars(&mut self, chars: &[u8]) -> Option<()> {
            if &self.input[self.pos..][..chars.len()] == chars {
                self.pos += chars.len();
                Some(())
            } else {
                None
            }
        }

        pub fn eat_byte(&mut self, b: u8) -> Option<()> {
            if self.next_nth_byte(0)? == b {
                self.pos += 1;
                Some(())
            } else {
                None
            }
        }

        pub fn eat_byte_or_end(&mut self, b: u8) -> Option<()> {
            match self.next_nth_byte(0) {
                Some(c) if c == b => {
                    self.pos += 1;
                }
                Some(_) => {
                    return None;
                }
                None => {}
            }
            Some(())
        }

        pub fn advance(&mut self, n: usize) {
            self.pos += n;
        }

        pub fn next_token(&mut self) -> Option<&'a [u8]> {
            while self.next_nth_byte(0)? == b' ' {
                self.advance(1);
            }
            let start = self.pos;
            let mut i = 0;
            while self.next_nth_byte(i).is_some()
                && self.next_nth_byte(i)? != b' '
                && self.next_nth_byte(i)? != b'\n'
            {
                i += 1;
            }
            let end = start + i;
            Some(&self.input[start..end])
        }

        pub fn consume_next_token(&mut self) -> Option<&[u8]> {
            let next = self.next_token()?;
            self.advance(next.len());
            Some(next)
        }
        pub fn end(&self) -> bool {
            self.pos == self.input.len()
        }
    }
}

pub mod bitset {
    pub struct Bitset<const N: usize> {
        elements: [u64; N],
    }

    impl<const N: usize> Bitset<N> {
        pub fn empty() -> Self {
            Self { elements: [0; N] }
        }

        pub fn add(&mut self, e: u32) {
            self.elements[(e / u64::BITS) as usize] |= 1 << (e % u64::BITS);
        }

        pub fn contains(&mut self, e: u32) -> bool {
            (self.elements[(e / u64::BITS) as usize] & (1 << (e % u64::BITS))) != 0
        }
    }
}

pub mod arena {

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Handle {
        idx: u32,
        generation: u8,
    }

    struct Slot<T> {
        content: Option<T>,
        generation: u8,
    }
    pub struct Arena<T> {
        storage: Vec<Slot<T>>,
    }

    impl<T> Default for Arena<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Arena<T> {
        pub fn new() -> Self {
            Arena {
                storage: Vec::new(),
            }
        }

        pub fn reserve(&mut self, additional: usize) {
            self.storage.reserve(additional)
        }

        pub fn insert(&mut self, e: T) -> Handle {
            let handle = self.handle_to_next();
            self.storage.push(Slot {
                content: Some(e),
                generation: handle.generation,
            });
            handle
        }

        pub fn handle_to_next(&self) -> Handle {
            Handle {
                idx: self.storage.len() as u32,
                generation: 0,
            }
        }

        pub fn get(&self, handle: Handle) -> Option<&T> {
            let slot = self.storage.get(handle.idx as usize)?;
            if slot.generation != handle.generation {
                return None;
            }
            slot.content.as_ref()
        }

        pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
            let slot = self.storage.get_mut(handle.idx as usize)?;
            if slot.generation != handle.generation {
                return None;
            }
            slot.content.as_mut()
        }
    }
}

pub mod tree {
    use super::arena::{Arena, Handle};

    #[macro_export]
    macro_rules! for_children {
        ($child:ident of node $n:ident in graph $g:ident $b:block) => {
            let first = $g.get($n).unwrap().child;
            if  $n != first {
                let mut current = first;
                loop {
                    let $child = current;
                    $b
                    if let Some(current_node) = $g.get(current) {
                        if current_node.next == first {
                            break
                        } else {
                            current = current_node.next;
                        }
                    } else {
                        break
                    }
                }
            }

        };
    }

    pub struct Node<T> {
        pub content: T,
        pub child: Handle,
        pub parent: Handle,
        pub next: Handle,
        pub previous: Handle,
    }

    pub struct Tree<T> {
        storage: Arena<Node<T>>,
        pub root: Option<Handle>,
    }

    impl<T> Default for Tree<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T> Tree<T> {
        pub fn new() -> Self {
            Self {
                storage: Arena::new(),
                root: None,
            }
        }

        pub fn insert_node(&mut self, node: T) -> Handle {
            let handle = self.storage.handle_to_next();
            self.storage.insert(Node {
                content: node,
                parent: handle,
                child: handle,
                next: handle,
                previous: handle,
            })
        }

        pub fn make_root(&mut self, root: Handle) {
            self.root = Some(root);
        }

        pub fn get(&self, handle: Handle) -> Option<&Node<T>> {
            self.storage.get(handle)
        }

        pub fn get_mut(&mut self, handle: Handle) -> Option<&mut Node<T>> {
            self.storage.get_mut(handle)
        }

        pub fn add_child(&mut self, parent: Handle, child: Handle) -> Option<()> {
            let parent_node = self.get_mut(parent)?;
            if parent_node.child == parent {
                parent_node.child = child;
                let child_node = self.get_mut(child)?;
                child_node.parent = parent;
                return Some(());
            }
            let first_child = parent_node.child;
            let first_child_node = self.get_mut(first_child)?;
            let previous = first_child_node.previous;
            first_child_node.previous = child;
            let previous_node = self.get_mut(previous)?;
            previous_node.next = child;

            let child_node = self.get_mut(child)?;
            child_node.next = first_child;
            child_node.previous = previous;
            child_node.parent = parent;
            Some(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Tree;

        #[test]
        fn test_graph_insert() {
            let mut g = Tree::new();
            let h1 = g.insert_node(1);

            let n1 = g.get(h1).unwrap();
            assert!(n1.content == 1);
            assert!(n1.child == h1);
            assert!(n1.parent == h1);
        }

        #[test]
        fn test_graph_insert_child() {
            let mut g = Tree::new();
            let h1 = g.insert_node(1);
            let h2 = g.insert_node(2);
            let h3 = g.insert_node(2);

            g.add_child(h1, h2).unwrap();
            g.add_child(h1, h3).unwrap();
            let n1 = g.get(h1).unwrap();
            assert!(n1.child == h2);
        }

        #[test]
        fn test_graph_insert_multiple_child() {
            let mut g = Tree::new();
            let h1 = g.insert_node(1);
            let h2 = g.insert_node(2);
            let h3 = g.insert_node(2);

            g.add_child(h1, h2).unwrap();
            g.add_child(h1, h3).unwrap();

            let n2 = g.get(h2).unwrap();
            let n3 = g.get(h3).unwrap();

            assert!(n3.next == h2);
            assert!(n3.previous == h2);
            assert!(n3.parent == h1);

            assert!(n2.next == h3);
            assert!(n2.previous == h3);
            assert!(n2.parent == h1);
        }

        #[test]
        fn test_for_children_loop() {
            let mut g = Tree::new();
            let h1 = g.insert_node(1);
            let h2 = g.insert_node(2);
            let h3 = g.insert_node(2);

            g.add_child(h1, h2).unwrap();
            g.add_child(h1, h3).unwrap();

            let mut i = 0;
            for_children!(c of node h1 in graph g {
                if i == 0 {
                    assert!(c == h2);
                    let c = g.get_mut(c).unwrap();
                    c.content += 1;
                } else if i == 1 {
                    assert!(c == h3);
                }
                i += 1
            });
            assert!(i == 2);
            assert!(g.get(h2).unwrap().content == 3);
        }
    }
}

pub mod array {
    use std::{fmt::Debug, mem::MaybeUninit, ops::Deref};

    pub struct Array<T, const N: usize> {
        inner: [MaybeUninit<T>; N],
        lenght: usize,
    }

    impl<T: Debug, const N: usize> Debug for Array<T, N> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.iter()).finish()
        }
    }

    impl<T, const N: usize> Array<T, N> {
        pub fn new() -> Self {
            Self {
                inner: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
                lenght: 0,
            }
        }
    }

    impl<T, const N: usize> Default for Array<T, N> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: Copy, const N: usize> Array<T, N> {
        pub fn from_slice(src: &[T]) -> Option<Self> {
            if src.len() >= N {
                return None;
            }
            let mut dst = Self::new();
            for (src, dst) in src.iter().zip(&mut dst.inner) {
                dst.write(*src);
            }
            dst.lenght = src.len();
            Some(dst)
        }
    }

    impl<T: Clone, const N: usize> Clone for Array<T, N> {
        fn clone(&self) -> Self {
            let mut other = Self::new();
            for (src, dst) in self.iter().zip(&mut other.inner) {
                dst.write(src.clone());
            }
            other
        }
    }
    impl<T: Copy, const N: usize> Copy for Array<T, N> {}

    impl<T, const N: usize> Deref for Array<T, N> {
        type Target = [T];
        fn deref(&self) -> &Self::Target {
            unsafe { std::mem::transmute::<&[MaybeUninit<T>], &[T]>(&self.inner[..self.lenght]) }
        }
    }
}

pub mod matrix {
    use std::{
        fmt::Debug,
        ops::{Index, IndexMut},
    };

    pub struct Matrix<T> {
        inner: Vec<T>,
        pub dims: (usize, usize),
    }

    impl<T> Matrix<T> {
        pub fn from_vec(v: Vec<T>, row_len: usize) -> Option<Self> {
            if v.len() % row_len != 0 {
                return None;
            }
            Some(Self {
                dims: (row_len, v.len() / row_len),
                inner: v,
            })
        }

        pub fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> Matrix<U> {
            let mut m2 = self.inner.iter().map(f).collect::<Matrix<_>>();
            m2.dims = self.dims;
            m2
        }

        pub fn get(&self, i: usize, j: usize) -> Option<&T> {
            if i >= self.dims.0 || j >= self.dims.1 {
                None
            } else {
                self.inner.get(i + self.dims.0 * j)
            }
        }

        pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
            if i >= self.dims.0 || j >= self.dims.1 {
                None
            } else {
                self.inner.get_mut(i + self.dims.0 * j)
            }
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.inner.iter()
        }

        pub fn neighbors(
            &self,
            i: usize,
            j: usize,
        ) -> impl Iterator<Item = (usize, usize)> + 'static {
            let dims = self.dims;
            [
                if i == 0 { None } else { Some((i - 1, j)) },
                if i + 1 >= dims.0 {
                    None
                } else {
                    Some((i + 1, j))
                },
                if j == 0 { None } else { Some((i, j - 1)) },
                if j + 1 >= dims.1 {
                    None
                } else {
                    Some((i, j + 1))
                },
            ]
            .into_iter()
            .flatten()
        }
    }

    impl<T: Debug> Debug for Matrix<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("[")?;
            for i in 0..self.dims.1 {
                f.debug_list()
                    .entries(self.inner[(i * self.dims.0)..(i + 1) * self.dims.0].iter())
                    .finish()?;
                f.write_str("\n")?;
            }
            f.write_str("]")?;
            Ok(())
        }
    }

    impl<T> FromIterator<T> for Matrix<T> {
        fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
            let inner = iter.into_iter().collect::<Vec<T>>();
            let len = inner.len();
            Self::from_vec(inner, len).unwrap()
        }
    }

    impl<T> Index<(usize, usize)> for Matrix<T> {
        type Output = T;
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            self.get(index.0, index.1).unwrap()
        }
    }

    impl<T> IndexMut<(usize, usize)> for Matrix<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            self.get_mut(index.0, index.1).unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Matrix;

        #[test]
        fn test_neighbors() {
            let m = Matrix::from_vec(vec![0, 1, 2, 3, 5, 6, 7, 8, 9], 3).unwrap();

            let mut n0 = m.neighbors(0, 0);
            assert_eq!(n0.next(), Some((1, 0)));
            assert_eq!(n0.next(), Some((0, 1)));
            assert_eq!(n0.next(), None);

            let mut n1 = m.neighbors(1, 0);
            assert_eq!(n1.next(), Some((0, 0)));
            assert_eq!(n1.next(), Some((2, 0)));
            assert_eq!(n1.next(), Some((1, 1)));
            assert_eq!(n1.next(), None);

            let mut n2 = m.neighbors(2, 0);
            assert_eq!(n2.next(), Some((1, 0)));
            assert_eq!(n2.next(), Some((2, 1)));
            assert_eq!(n2.next(), None);

            let mut n3 = m.neighbors(1, 1);
            assert_eq!(n3.next(), Some((0, 1)));
            assert_eq!(n3.next(), Some((2, 1)));
            assert_eq!(n3.next(), Some((1, 0)));
            assert_eq!(n3.next(), Some((1, 2)));
            assert_eq!(n3.next(), None);

            let mut n4 = m.neighbors(1, 2);
            assert_eq!(n4.next(), Some((0, 2)));
            assert_eq!(n4.next(), Some((2, 2)));
            assert_eq!(n4.next(), Some((1, 1)));
            assert_eq!(n4.next(), None);
        }
    }
}

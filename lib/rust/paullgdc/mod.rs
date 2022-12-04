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

pub mod tokenize {
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

        pub fn next_u8(&mut self) -> Option<u8> {
            let mut res = 0;
            let mut skip = 0;
            for (i, b) in self.input[self.pos..].into_iter().enumerate() {
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
                return  None;
            }
            self.pos += skip;
            Some(res)
        }

        pub fn eat_chars(&mut self, chars: &[u8]) -> Option<()> {
            if &self.input[self.pos..chars.len()] == chars {
                self.pos += chars.len();
                Some(())
            } else {
                None
            }
        }

        pub fn eat_byte(&mut self, b: u8) -> Option<()> {
            if *self.input.get(self.pos)? == b {
                self.pos += 1;
                Some(())
            } else {
                None
            }
        }
    }
}

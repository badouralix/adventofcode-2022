pub struct LastElem<I: Iterator> {
    it: std::iter::Peekable<I>,
}

impl <I: Iterator> LastElem<I> {
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

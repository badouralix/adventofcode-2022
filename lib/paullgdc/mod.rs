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
    fn next(&mut self) -> Option<Self::Item> {
        if self.it.peek().is_none() {
            Some((self.it.next()?, true))
        } else {
            Some((self.it.next()?, false))
        }
    }
}

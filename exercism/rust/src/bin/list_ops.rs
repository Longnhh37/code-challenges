pub fn append<I, J>(a: I, b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    a.chain(b)
}

pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    nested_iter.flatten()
}

struct Filter<I, F> {
    iter: I,
    predicate: F,
}

impl<I, F> Iterator for Filter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    #[allow(clippy::while_let_on_iterator)]
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) {
                return Some(item);
            }
        }
        None
    }
}

pub fn filter<I, F>(iter: I, predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: FnMut(&I::Item) -> bool,
{
    Filter { iter, predicate }
}

#[allow(clippy::while_let_on_iterator)]
pub fn length<I: Iterator>(mut iter: I) -> usize {
    let mut count = 0usize;
    while let Some(_) = iter.next() {
        count += 1;
    }
    count
}

struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(item) => Some((self.f)(item)),
            None => None,
        }
    }
}

pub fn map<I, F, U>(iter: I, function: F) -> impl Iterator<Item = U>
where 
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    Map { iter, f: function }
}

#[allow(clippy::while_let_on_iterator)]
pub fn foldl<I, F, U>(mut iter: I,mut acc: U, f: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next() {
        acc = f(acc, item);
    }
    acc
}

pub fn foldr<I, F, U>(mut iter: I, mut acc: U, f: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    while let Some(item) = iter.next_back() {
        acc = f(acc, item);
    }
    acc
}

pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    iter.rev()
}


fn main() {}

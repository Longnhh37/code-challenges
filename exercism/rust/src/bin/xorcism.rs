use std::borrow::Borrow;
use std::iter::Cycle;
use std::slice::Iter;

#[derive(Clone)]
pub struct Xorcism<'a> {
    key_iter: Cycle<Iter<'a, u8>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Self
    where
        Key: AsRef<[u8]> + ?Sized,
    {
        Self {
            key_iter: key.as_ref().iter().cycle(),
        }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            let key_byte = self.key_iter.next().unwrap();
            *b ^= key_byte;
        }
    }

    pub fn munge<'b, Data>(&'b mut self, data: Data) -> impl Iterator<Item = u8> + 'b
    where
        Data: IntoIterator,
        Data::Item: std::borrow::Borrow<u8>,
        Data::IntoIter: 'b,
    {
        data.into_iter()
            .map(move |b| b.borrow() ^ self.key_iter.next().unwrap())
    }
}

fn main() {}

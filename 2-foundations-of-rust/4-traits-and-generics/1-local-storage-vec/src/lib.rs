/// A growable, generic list that resides on the stack if it's small,
/// but is moved to the heap to grow larger if needed.
/// This list is generic over the items it contains as well as the
/// size of its buffer if it's on the stack.
pub enum LocalStorageVec<T, const N: usize> {
    Stack { buf: [T; N], len: usize },
    Heap(Vec<T>),
}

impl<T, const N: usize> From<Vec<T>> for LocalStorageVec<T, N> {
    fn from(v: Vec<T>) -> Self {
        Self::Heap(v)
    }
}

impl<T: Default, const N: usize, const M: usize> From<[T; N]> for LocalStorageVec<T, M> {
    fn from(array: [T; N]) -> Self {
        if N <= M {
            let mut it = array.into_iter();
            Self::Stack {
                buf: [(); M].map(|_| it.next().unwrap_or_default()),
                len: N,
            }
        } else {
            Self::Heap(Vec::from(array))
        }
    }
}

impl<T: Copy + Default, const N: usize> LocalStorageVec<T, N> {
    pub fn new() -> Self {
        Self::Stack {
            buf: [T::default(); N],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Stack { len, .. } => *len,
            Self::Heap(v) => v.len(),
        }
    }

    pub fn push(&mut self, item: T) {
        match self {
            Self::Stack { buf, len } if *len < N => {
                buf[*len] = item;
                *len += 1;
            }
            _ => {
                let mut v = match std::mem::replace(self, Self::Heap(Vec::new())) {
                    Self::Stack { buf, len } => buf[..len].to_vec(),
                    Self::Heap(v) => v,
                };
                v.push(item);
                *self = Self::Heap(v);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self {
            Self::Stack { buf, len } if *len > 0 => {
                *len -= 1;
                Some(buf[*len])
            }
            Self::Heap(v) => v.pop(),
            _ => None,
        }
    }

    pub fn insert(&mut self, index: usize, item: T) {
        match self {
            Self::Stack { buf, len } if *len < N => {
                buf.copy_within(index..*len, index + 1);
                buf[index] = item;
                *len += 1;
            }
            _ => {
                let mut v = match std::mem::replace(self, Self::Heap(Vec::new())) {
                    Self::Stack { buf, len } => buf[..len].to_vec(),
                    Self::Heap(v) => v,
                };
                v.insert(index, item);
                *self = Self::Heap(v);
            }
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self {
            Self::Stack { buf, len } if *len > 0 => {
                let item = buf[index];
                buf.copy_within(index + 1..*len, index);
                *len -= 1;
                item
            }
            Self::Heap(v) => v.remove(index),
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn clear(&mut self) {
        match self {
            Self::Stack { len, .. } => *len = 0,
            Self::Heap(v) => v.clear(),
        }
    }
}

pub struct LocalStorageVecIter<T, const N: usize> {
    vec: LocalStorageVec<T, N>,
    counter: usize,
}

impl<T: Default, const N: usize> Iterator for LocalStorageVecIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.vec.len() {
            let item = std::mem::take(&mut self.vec[self.counter]);
            self.counter += 1;
            Some(item)
        } else {
            None
        }
    }
}
use std::ops::IndexMut;

impl<T, const N: usize> IndexMut<usize> for LocalStorageVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            LocalStorageVec::Stack { buf, len } if index < *len => &mut buf[index],
            LocalStorageVec::Heap(v) => &mut v[index],
            _ => panic!("Index out of bounds"),
        }
    }
}
impl<T: Default, const N: usize> IntoIterator for LocalStorageVec<T, N> {
    type Item = T;
    type IntoIter = LocalStorageVecIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        LocalStorageVecIter {
            vec: self,
            counter: 0,
        }
    }
}

use std::ops::{Index, Range, RangeFrom, RangeTo};

impl<T, const N: usize> Index<usize> for LocalStorageVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Self::Stack { buf, len } if index < *len => &buf[index],
            Self::Heap(v) => &v[index],
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T, const N: usize> Index<RangeTo<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        match self {
            Self::Stack { buf, len } if index.end <= *len => &buf[..index.end],
            Self::Heap(v) => &v[..index.end],
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T, const N: usize> Index<RangeFrom<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        match self {
            Self::Stack { buf, len } if index.start < *len => &buf[index.start..*len],
            Self::Heap(v) => &v[index.start..],
            _ => panic!("Index out of bounds"),
        }
    }
}

impl<T, const N: usize> Index<Range<usize>> for LocalStorageVec<T, N> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        match self {
            Self::Stack { buf, len } if index.end <= *len => &buf[index],
            Self::Heap(v) => &v[index],
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::{Deref, DerefMut};

impl<T, const N: usize> Deref for LocalStorageVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        <Self as AsRef<[T]>>::as_ref(self)
    }
}
use std::convert::AsRef;

impl<T, const N: usize> AsRef<[T]> for LocalStorageVec<T, N> {
    fn as_ref(&self) -> &[T] {
        match self {
            LocalStorageVec::Stack { buf, len } => &buf[..*len],
            LocalStorageVec::Heap(v) => v.as_ref(),
        }
    }
}
use std::convert::AsMut;

impl<T, const N: usize> AsMut<[T]> for LocalStorageVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        match self {
            LocalStorageVec::Stack { buf, len } => &mut buf[..*len],
            LocalStorageVec::Heap(v) => v.as_mut(),
        }
    }
}
impl<T, const N: usize> DerefMut for LocalStorageVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        <Self as AsMut<[T]>>::as_mut(self)
    }
}

#[cfg(test)]
mod test {
    use crate::LocalStorageVec;



    #[test]
    fn it_from_vecs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::from(vec![1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));

        let vec: LocalStorageVec<usize, 2> = LocalStorageVec::from(vec![1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
    }


    #[test]
    fn it_as_refs() {
        let vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);
        let vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice: &[i32] = vec.as_ref();
        assert!(slice.len() == 128);

        let mut vec: LocalStorageVec<i32, 256> = LocalStorageVec::from([0; 128]);
        let slice_mut: &[i32] = vec.as_mut();
        assert!(slice_mut.len() == 128);
        let mut vec: LocalStorageVec<i32, 32> = LocalStorageVec::from([0; 128]);
        let slice_mut: &[i32] = vec.as_mut();
        assert!(slice_mut.len() == 128);
    }


    #[test]
    fn it_constructs() {
        let vec: LocalStorageVec<usize, 10> = LocalStorageVec::new();
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 0 }));
    }


    #[test]
    fn it_lens() {
        let vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
        let vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        assert_eq!(vec.len(), 3);
    }


    #[test]
    fn it_pushes() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::new();
        for value in 0..128 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Stack { len: 128, .. }));
        for value in 128..256 {
            vec.push(value);
        }
        assert!(matches!(vec, LocalStorageVec::Heap(v) if v.len() == 256))
    }


    #[test]
    fn it_pops() {
        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        for _ in 0..128 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 256]);
        for _ in 0..256 {
            assert_eq!(vec.pop(), Some(0))
        }
        assert_eq!(vec.pop(), None);
    }


    #[test]
    fn it_inserts() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        vec.insert(1, 3);
        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 3, 1, 2],
                len: 4
            }
        ));

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3]);

        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2, 3, 4]);
        vec.insert(1, 3);
        assert!(matches!(vec, LocalStorageVec::Heap { .. }));
        assert_eq!(vec.as_ref(), &[0, 3, 1, 2, 3, 4])
    }


    #[test]
    fn it_removes() {
        let mut vec: LocalStorageVec<_, 4> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);

        assert!(matches!(
            vec,
            LocalStorageVec::Stack {
                buf: [0, 2, _, _],
                len: 2
            }
        ));
        assert_eq!(elem, 1);

        let mut vec: LocalStorageVec<_, 2> = LocalStorageVec::from([0, 1, 2]);
        let elem = vec.remove(1);
        assert!(matches!(vec, LocalStorageVec::Heap(..)));
        assert_eq!(vec.as_ref(), &[0, 2]);
        assert_eq!(elem, 1);
    }


    #[test]
    fn it_clears() {
        let mut vec: LocalStorageVec<_, 10> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Stack { buf: _, len: 4 }));
        vec.clear();
        assert_eq!(vec.len(), 0);

        let mut vec: LocalStorageVec<_, 3> = LocalStorageVec::from([0, 1, 2, 3]);
        assert!(matches!(vec, LocalStorageVec::Heap(_)));
        vec.clear();
        assert_eq!(vec.len(), 0);
    }


    #[test]
    fn it_iters() {
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 32]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);

        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from(vec![0; 128]);
        let mut iter = vec.into_iter();
        for item in &mut iter {
            assert_eq!(item, 0);
        }
        assert_eq!(iter.next(), None);
    }


    #[test]
    fn it_indexes() {
        let vec: LocalStorageVec<i32, 10> = LocalStorageVec::from([0, 1, 2, 3, 4, 5]);
        assert_eq!(vec[1], 1);
        assert_eq!(vec[..2], [0, 1]);
        assert_eq!(vec[4..], [4, 5]);
        assert_eq!(vec[1..3], [1, 2]);
    }


    #[test]
    fn it_borrowing_iters() {
        let vec: LocalStorageVec<String, 10> = LocalStorageVec::from([
            "0".to_owned(),
            "1".to_owned(),
            "2".to_owned(),
            "3".to_owned(),
            "4".to_owned(),
            "5".to_owned(),
        ]);
        let iter = vec.iter();
        for _ in iter {}
        drop(vec);
    }


    #[test]
    fn it_derefs() {
        use std::ops::{Deref, DerefMut};
        let vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        let chunks = vec.chunks(4);
        let slice: &[_] = vec.deref();

        let mut vec: LocalStorageVec<_, 128> = LocalStorageVec::from([0; 128]);
        let chunks = vec.chunks_mut(4);
        let slice: &mut [_] = vec.deref_mut();
    }
}
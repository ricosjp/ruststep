use super::*;
use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Array<T, E> {
    entity: E,
    minidx: isize,
    dummy: PhantomData<T>,
}

impl<T, E: AsRef<[T]>> AsRef<[T]> for Array<T, E> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.entity.as_ref()
    }
}

impl<T, E: AsMut<[T]>> AsMut<[T]> for Array<T, E> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        self.entity.as_mut()
    }
}

impl<T, E: Borrow<[T]>> Borrow<[T]> for Array<T, E> {
    #[inline(always)]
    fn borrow(&self) -> &[T] {
        self.entity.borrow()
    }
}

impl<T, E: BorrowMut<[T]>> BorrowMut<[T]> for Array<T, E> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.entity.borrow_mut()
    }
}

impl<T, E: AsRef<[T]>> Index<isize> for Array<T, E> {
    type Output = T;
    #[inline(always)]
    fn index(&self, idx: isize) -> &T {
        self.as_ref().index((idx - self.minidx) as usize)
    }
}

impl<T, E: AsRef<[T]> + AsMut<[T]>> IndexMut<isize> for Array<T, E> {
    #[inline(always)]
    fn index_mut(&mut self, idx: isize) -> &mut T {
        let minidx = self.minidx;
        self.as_mut().index_mut((idx - minidx) as usize)
    }
}

impl<'a, T, E: AsRef<[T]>> IntoIterator for &'a Array<T, E> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'a, T, E: AsMut<[T]>> IntoIterator for &'a mut Array<T, E> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_iter()
    }
}

impl<T, E> From<E> for Array<T, E> {
    fn from(entity: E) -> Self {
        Array {
            entity,
            minidx: 0,
            dummy: PhantomData,
        }
    }
}

impl<T, E> Array<T, E> {
    pub fn new(entity: E, minidx: isize) -> Self {
        Array {
            entity,
            minidx,
            dummy: PhantomData,
        }
    }
}

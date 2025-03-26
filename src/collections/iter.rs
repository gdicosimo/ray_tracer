use std::{alloc, marker::PhantomData};

use super::ArrayList;

pub struct IntoIter<T> {
    ptr: *mut T,
    len: usize,
    current: usize,
    layout: Option<alloc::Layout>,
}

pub struct Iter<'a, T> {
    ptr: *const T,
    len: usize,
    current: usize,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    ptr: *mut T,
    len: usize,
    current: usize,
    _marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(list: &'a ArrayList<T>) -> Self {
        Iter {
            ptr: list.ptr.as_ptr(),
            len: list.len(),
            current: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(list: &'a mut ArrayList<T>) -> Self {
        IterMut {
            ptr: list.ptr.as_ptr(),
            len: list.len(),
            current: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> IntoIter<T> {
    pub(crate) fn new(list: ArrayList<T>) -> Self {
        let iter = IntoIter {
            ptr: list.ptr.as_ptr(),
            len: list.len(),
            current: 0,
            layout: list.layout,
        };
        std::mem::forget(list);
        iter
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            unsafe {
                let item = self.ptr.add(self.current).read();
                self.current += 1;
                Some(item)
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.current;
        (remaining, Some(remaining))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            unsafe {
                let item = &*self.ptr.add(self.current);
                self.current += 1;
                Some(item)
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.current;
        (remaining, Some(remaining))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            unsafe {
                let item = &mut *self.ptr.add(self.current);
                self.current += 1;
                Some(item)
            }
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len - self.current;
        (remaining, Some(remaining))
    }
}

impl<T> IntoIterator for ArrayList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for i in self.current..self.len {
            unsafe {
                std::ptr::drop_in_place(self.ptr.add(i));
            }
        }

        if let Some(layout) = self.layout {
            unsafe { alloc::dealloc(self.ptr as *mut u8, layout) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: ArrayList<i32> = ArrayList::new();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_immutable_iter() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_mutable_iter() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        for item in list.iter_mut() {
            *item *= 2;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
    }

    #[test]
    fn test_into_iter_consumption() {
        let mut list = ArrayList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some("a".to_string()));
        assert_eq!(iter.next(), Some("b".to_string()));
        assert_eq!(iter.next(), Some("c".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_partial_iteration() {
        let mut list = ArrayList::new();
        list.push(10);
        list.push(20);
        list.push(30);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(10));
        // No se consumen todos los elementos
    } // El drop del iterador debería limpiar los elementos restantes

    #[test]
    fn test_zero_sized_types() {
        let mut list = ArrayList::new();
        list.push(());
        list.push(());

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(()));
        assert_eq!(iter.next(), Some(()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_large_dataset() {
        let mut list = ArrayList::new();
        for i in 0..10_000 {
            list.push(i);
        }

        let mut count = 0;
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, i);
            count += 1;
        }
        assert_eq!(count, 10_000);
    }

    #[test]
    fn test_mixed_iterators() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let sum: i32 = list.iter().sum();
        assert_eq!(sum, 6);

        for item in list.iter_mut() {
            *item += 1;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
    }

    #[test]
    fn test_iterator_size_hint() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        iter.next();
        assert_eq!(iter.size_hint(), (2, Some(2)));
    }
}

use std::{alloc, ptr};

use super::iter;

pub struct ArrayList<T> {
    pub(crate) ptr: ptr::NonNull<T>,
    capacity: usize,
    len: usize,
    pub(crate) layout: Option<alloc::Layout>,
}

impl<T> ArrayList<T> {
    pub fn new() -> Self {
        let ptr = ptr::NonNull::dangling();
        ArrayList {
            ptr,
            capacity: 0,
            len: 0,
            layout: None,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = alloc::Layout::array::<T>(capacity).unwrap();

        let raw_ptr = unsafe { alloc::alloc(layout) as *mut T };

        let ptr = ptr::NonNull::new(raw_ptr).unwrap_or_else(|| {
            alloc::handle_alloc_error(layout);
        });

        Self {
            ptr,
            capacity,
            len: 0,
            layout: Some(layout),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            let end = self.ptr.as_ptr().add(self.len);
            end.write(element);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            unsafe {
                let end = self.ptr.as_ptr().add(self.len - 1);
                self.len -= 1;
                Some(end.read())
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(&*self.ptr.as_ptr().add(index)) }
        }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            unsafe { Some(&mut *self.ptr.as_ptr().add(index)) }
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        unsafe {
            let removed_item = self.ptr.add(index).read();

            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index - 1,
            );

            self.len -= 1;
            Some(removed_item)
        }
    }

    pub fn clear(&mut self) {
        if self.len == 0 {
            return;
        }

        unsafe {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
        }

        self.len = 0;
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> iter::Iter<'_, T> {
        iter::Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
        iter::IterMut::new(self)
    }

    pub fn into_iter(self) -> iter::IntoIter<T> {
        iter::IntoIter::new(self)
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        let new_layout = alloc::Layout::array::<T>(new_capacity).unwrap();

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = unsafe {
            if let Some(old_layout) = self.layout {
                alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
            } else {
                alloc::alloc(new_layout)
            }
        };

        self.ptr = match ptr::NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_capacity;
        self.layout = Some(new_layout);
    }
}

unsafe impl<T: Send> Send for ArrayList<T> {}
unsafe impl<T: Sync> Sync for ArrayList<T> {}

impl<T> Default for ArrayList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for ArrayList<T> {
    #[inline]
    fn drop(&mut self) {
        self.clear();

        if let Some(layout) = self.layout {
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_and_push() {
        let mut al = ArrayList::<f32>::new();
        al.push(35.014633);
        al.push(-106.686264);

        assert!(al.capacity == 2);
        assert!(!al.is_empty());

        al.push(0.0);
        assert!(al.capacity == 4);
        al.pop();

        assert_eq!(al.pop(), Some(-106.686264));
        assert_eq!(al.pop(), Some(35.014633));

        assert!(al.is_empty());
    }

    #[test]
    fn empty() {
        let al = ArrayList::<()>::new();
        assert!(al.is_empty())
    }

    #[test]
    fn test_with_capacity() {
        let mut vec = ArrayList::<String>::with_capacity(10);
        assert_eq!(vec.len(), 0); // len debe ser 0 inicialmente

        vec.push("Hello".to_string());
        assert_eq!(vec.len(), 1);
    }
}

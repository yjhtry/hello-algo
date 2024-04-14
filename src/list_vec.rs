use std::ptr;

use crate::List;

#[derive(Debug)]
pub struct VecList<T> {
    data: Vec<T>,
    size: usize,
    capacity: usize,
    extend_ratio: u8,
}

impl<T> VecList<T> {
    pub fn new() -> Self {
        VecList {
            data: Vec::new(),
            size: 0,
            capacity: 10,
            extend_ratio: 2,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        VecList {
            data: Vec::with_capacity(capacity),
            size: 0,
            capacity,
            extend_ratio: 2,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<T> List for VecList<T> {
    type Item = T;

    fn get(&self, index: usize) -> Option<&Self::Item> {
        if index < self.size {
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn set(&mut self, index: usize, item: Self::Item) -> Option<Self::Item> {
        if index < self.size {
            let old_item = std::mem::replace(&mut self.data[index], item);
            Some(old_item)
        } else {
            None
        }
    }

    fn add(&mut self, item: Self::Item) {
        if self.size == self.capacity {
            self.extend_capacity();
        }

        self.data[self.size] = item;
        self.size += 1;
    }

    fn insert(&mut self, index: usize, mut item: Self::Item) {
        if index >= self.size {
            panic!("index out of bounds");
        }

        if self.size == self.capacity {
            self.extend_capacity();
        }

        // todo Test
        for i in index..=self.size {
            item = std::mem::replace(&mut self.data[i], item);
        }

        self.size += 1;
    }

    fn remove(&mut self, index: usize) -> Option<Self::Item> {
        if index >= self.size {
            return None;
        }

        unsafe {
            // infallible
            let ret;
            {
                let ptr = self.data.as_mut_ptr().add(index);

                ret = ptr::read(ptr);

                ptr::copy(ptr.add(1), ptr, self.size - index - 1);
            }

            self.size -= 1;

            Some(ret)
        }
    }

    fn extend_capacity(&mut self) {
        let new_capacity = self.capacity * self.extend_ratio as usize;
        let mut new_data = Vec::with_capacity(new_capacity);

        unsafe {
            new_data.set_len(new_capacity);
            ptr::copy_nonoverlapping(self.data.as_ptr(), new_data.as_mut_ptr(), self.size);
        }

        self.data = new_data;
        self.capacity = new_capacity;
    }

    fn to_vec(&self) -> Vec<Self::Item> {
        let mut res = Vec::with_capacity(self.size);

        unsafe {
            res.set_len(self.size);
            ptr::copy_nonoverlapping(self.data.as_ptr(), res.as_mut_ptr(), self.size);
        }

        res
    }
}

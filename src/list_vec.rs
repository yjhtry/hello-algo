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
        const DEFAULT_CAPACITY: usize = 10;
        VecList {
            data: Vec::with_capacity(DEFAULT_CAPACITY),
            size: 0,
            capacity: DEFAULT_CAPACITY,
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

        self.data.push(item);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_list() -> VecList<i32> {
        let mut list = VecList::new();
        list.add(1);
        list.add(2);
        list.add(3);
        list.add(4);
        list.add(5);

        list
    }

    #[test]
    fn test_get() {
        let list = setup_list();
        println!("{:?}", list);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), Some(&4));
        assert_eq!(list.get(4), Some(&5));
        assert_eq!(list.get(5), None);
    }

    #[test]
    fn test_set() {
        let mut list = setup_list();
        assert_eq!(list.set(0, 10), Some(1));
        assert_eq!(list.set(1, 20), Some(2));
        assert_eq!(list.set(2, 30), Some(3));
        assert_eq!(list.set(3, 40), Some(4));
        assert_eq!(list.set(4, 50), Some(5));
        assert_eq!(list.set(5, 60), None);
    }

    #[test]
    fn test_add() {
        let mut list = setup_list();
        list.add(6);
        list.add(7);
        list.add(8);
        list.add(9);
        list.add(10);

        assert_eq!(list.get(5), Some(&6));
        assert_eq!(list.get(6), Some(&7));
        assert_eq!(list.get(7), Some(&8));
        assert_eq!(list.get(8), Some(&9));
        assert_eq!(list.get(9), Some(&10));
    }

    #[test]
    fn test_insert() {
        let mut list = setup_list();
        list.insert(0, 10);
        list.insert(1, 20);
        list.insert(2, 30);
        list.insert(3, 40);
        list.insert(4, 50);

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&30));
        assert_eq!(list.get(3), Some(&40));
        assert_eq!(list.get(4), Some(&50));
    }
}

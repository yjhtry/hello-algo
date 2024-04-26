use std::fmt::Display;

use crate::KvPair;

#[derive(Debug)]
pub struct LinearHashMap<T: Copy> {
    data: Vec<Option<KvPair<T>>>,
    capacity: usize,
    extend_rate: usize,
    load_factor: f64,
    size: usize,
    tombstone: Option<KvPair<T>>,
}

impl<T: Copy + Display> Display for LinearHashMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.data.iter().enumerate() {
            if v.is_some() {
                writeln!(f, "{}: {}", i, v.unwrap())?;
            } else {
                writeln!(f, "{}: None", i)?;
            }
        }
        Ok(())
    }
}

impl<T: Copy + Default> LinearHashMap<T> {
    pub fn new() -> Self {
        LinearHashMap {
            data: vec![Default::default(); 10],
            capacity: 10,
            extend_rate: 2,
            load_factor: 2.0 / 3.0,
            size: 0,
            tombstone: Some(KvPair {
                key: -1,
                value: Default::default(),
            }),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn load_factor(&self) -> f64 {
        self.size as f64 / self.capacity as f64
    }

    fn hash_func(&self, key: i32) -> usize {
        (key as usize) % self.capacity
    }

    fn find_bucket(&mut self, key: i32) -> usize {
        let mut index = self.hash_func(key);
        let mut first_tombstone: i32 = -1;

        while let Some(kv) = self.data[index].as_ref() {
            if kv.key == key {
                if first_tombstone != -1 {
                    self.data[first_tombstone as usize] = self.data[index].take();
                    self.data[index] = self.tombstone.clone();
                }
                return index;
            }

            if first_tombstone == -1 && kv.key == -1 {
                first_tombstone = index as i32;
            }

            index = index + 1 % self.capacity;
        }

        if first_tombstone == -1 {
            index
        } else {
            first_tombstone as usize
        }
    }

    pub fn get(&mut self, key: i32) -> Option<T> {
        let index = self.find_bucket(key);
        let item = self.data[index];

        if item.is_some() && item.unwrap().key != -1 {
            Some(item.unwrap().value)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: i32, value: T) {
        if self.load_factor() > self.load_factor {
            self.extend();
        }

        let index = self.find_bucket(key);

        self.data[index] = Some(KvPair { key, value });

        self.size += 1;
    }

    pub fn remove(&mut self, key: i32) -> Option<T> {
        let index = self.find_bucket(key);

        if let Some(kv) = self.data[index] {
            self.data[index] = self.tombstone.clone();
            self.size -= 1;

            Some(kv.value)
        } else {
            None
        }
    }

    fn extend(&mut self) {
        let new_capacity = self.capacity * self.extend_rate;
        let old_data = std::mem::replace(&mut self.data, vec![Default::default(); new_capacity]);

        self.size = 0;

        for kv in old_data {
            if kv.is_some() && kv.unwrap().key != -1 {
                let kv = kv.unwrap();
                self.insert(kv.key, kv.value);
            }
        }

        self.capacity = new_capacity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_hashmap() {
        let mut map = LinearHashMap::new();

        for i in 1..=5 {
            map.insert(i, i * 10);
        }

        assert_eq!(map.size(), 5);

        for i in 1..=5 {
            assert_eq!(map.get(i), Some(i * 10));
        }

        for i in 1..=5 {
            map.remove(i);
        }

        assert_eq!(map.size(), 0);

        for i in 1..=5 {
            assert_eq!(map.get(i), None);
        }
    }
}

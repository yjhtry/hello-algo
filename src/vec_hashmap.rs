use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub struct KvPair<T: Copy> {
    pub key: i32,
    pub value: T,
}

/// A simple hash map implemented with a vector.
pub struct VecHashMap<T: Copy> {
    data: Vec<Vec<KvPair<T>>>,
    capacity: usize,
    expand_rate: usize,
    load_factor: f64,
    size: usize,
}

impl<T: Copy + Display> Display for KvPair<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.key, self.value)
    }
}

impl<T: Copy + Display> Display for VecHashMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, bucket) in self.data.iter().enumerate() {
            if !bucket.is_empty() {
                write!(f, "{}: ", index)?;

                for kv in bucket {
                    write!(f, "{} ", kv)?;
                }

                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<T: Copy> VecHashMap<T> {
    pub fn new() -> Self {
        VecHashMap {
            data: vec![vec![]; 10],
            capacity: 10,
            load_factor: 2.0 / 3.0,
            expand_rate: 2,
            size: 0,
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

    fn hash_index(&self, key: i32) -> usize {
        (key as usize) % self.capacity
    }

    pub fn get(&self, key: i32) -> Option<T> {
        let index = self.hash_index(key);
        let bucket = &self.data[index];

        bucket.iter().find(|kv| kv.key == key).map(|kv| kv.value)
    }

    pub fn insert(&mut self, key: i32, value: T) {
        if self.load_factor() > self.load_factor {
            self.extend();
        }

        let index = self.hash_index(key);

        if let Some(kv) = self.data[index].iter_mut().find(|kv| kv.key == key) {
            kv.value = value;
            return;
        } else {
            self.data[index].push(KvPair { key, value });
        }

        self.size += 1;
    }

    pub fn remove(&mut self, key: i32) -> Option<T> {
        let index = self.hash_index(key);
        let bucket = &mut self.data[index];

        if let Some(pos) = bucket.iter().position(|kv| kv.key == key) {
            let kv = bucket.remove(pos);
            self.size -= 1;

            Some(kv.value)
        } else {
            None
        }
    }

    fn extend(&mut self) {
        let new_capacity = self.capacity * self.expand_rate;
        let old_data = std::mem::replace(&mut self.data, vec![vec![]; new_capacity]);

        self.size = 0;

        for bucket in old_data {
            for kv in bucket {
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
    fn test_vec_hashmap() {
        let mut map = VecHashMap::new();

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

    #[test]
    fn test_vec_hashmap_conflict() {
        let mut map = VecHashMap::new();

        map.insert(11, 110);

        for i in 1..=5 {
            map.insert(i, i * 10);
        }

        assert_eq!(map.size(), 6);

        assert_eq!(map.data[1].len(), 2);
    }
}

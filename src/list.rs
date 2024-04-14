pub trait List {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn set(&mut self, index: usize, item: Self::Item) -> Option<Self::Item>; // return old item
    fn add(&mut self, item: Self::Item);
    fn insert(&mut self, index: usize, item: Self::Item);
    fn remove(&mut self, index: usize) -> Option<Self::Item>;
    fn extend_capacity(&mut self);

    fn to_vec(&self) -> Vec<Self::Item>;
}

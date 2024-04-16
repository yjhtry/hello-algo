pub trait Queue {
    type Item;
    fn push(&mut self, value: Self::Item);
    fn pop(&mut self) -> Option<Self::Item>;
    // todo return Option<&Self::Item>
    fn peek(&self) -> Option<Self::Item>;
    fn to_vec(&self) -> Vec<Self::Item>;
}

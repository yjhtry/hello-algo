pub trait DoubleQueue {
    type Item;

    fn push_first(&mut self, value: Self::Item);
    fn push_last(&mut self, value: Self::Item);
    fn pop_first(&mut self) -> Option<Self::Item>;
    fn pop_last(&mut self) -> Option<Self::Item>;
    fn peek_first(&self) -> Option<Self::Item>;
    fn peek_last(&self) -> Option<Self::Item>;
    fn to_vec(&self) -> Vec<Self::Item>;
}

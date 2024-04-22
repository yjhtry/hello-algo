pub struct VecDoubleQueue<T> {
    data: Vec<T>,
    front: usize,
    rear: usize,
    size: usize,
    capacity: usize,
    capacity_ratio: usize,
}

impl<T: Copy + Default> VecDoubleQueue<T> {
    pub fn new() -> Self {
        Self {
            data: vec![Default::default(); 10],
            size: 0,
            front: 0,
            rear: 0,
            capacity: 10,
            capacity_ratio: 2,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn push_first(&mut self, value: T) {
        self.extend_capacity();

        self.rear = (self.rear + 1) % self.capacity;

        self.data[self.rear] = value;

        self.size += 1;
    }

    pub fn extend_capacity(&mut self) {
        if self.size == self.capacity {
            let new_capacity = self.capacity * self.capacity_ratio;
            let mut data = vec![Default::default(); new_capacity];

            for (i, v) in self.data.to_vec().into_iter().enumerate() {
                data[i] = v;
            }

            self.data = data;
            self.front = self.capacity;
            self.rear = self.size;
            self.capacity = new_capacity;
        }
    }
}

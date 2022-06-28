#[cfg(test)]
mod tests;

mod heap {
    pub struct Heap<T> {
        pub elements: Vec<T>,
    }

    pub trait BinaryHeap<T> {
        fn new(size: usize) -> Self;
        fn insert(&mut self, value: T);
        fn pop(&mut self) -> T;
    }

    impl<T: std::cmp::PartialOrd + Copy> BinaryHeap<T> for Heap<T> {
        fn new(size: usize) -> Self {
            Heap {
                elements: Vec::with_capacity(size),
            }
        }

        fn insert(&mut self, value: T) {
            fn heapyfy_up<T: std::cmp::PartialOrd>(heap: &mut Heap<T>, i: usize) {
                let mut cur_elem = i;
                let mut parent: usize = cur_elem.wrapping_sub(1) / 2;
                while parent < heap.elements.len()
                    && heap.elements[cur_elem] > heap.elements[parent]
                {
                    heap.elements.swap(cur_elem, parent);
                    cur_elem = parent;
                    parent = cur_elem.wrapping_sub(1) / 2;
                }
            }
            self.elements.push(value);
            heapyfy_up(self, self.elements.len() - 1);
        }
        fn pop(&mut self) -> T {
            self.elements[0]
        }
    }
}

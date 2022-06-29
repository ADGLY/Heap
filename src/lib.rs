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
            fn heapify_up<T: std::cmp::PartialOrd>(heap: &mut Heap<T>, i: usize) {
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
            heapify_up(self, self.elements.len() - 1);
        }
        fn pop(&mut self) -> T {
            fn heapify_down<T: std::cmp::PartialOrd>(heap: &mut Heap<T>, i: usize) {
                let mut i = i;
                let mut left_child = 2 * i + 1;
                let mut right_child = 2 * i + 2;
                while left_child < heap.elements.len() {
                    let mut largest = left_child;
                    if right_child < heap.elements.len()
                        && heap.elements[right_child] > heap.elements[left_child]
                    {
                        largest = right_child;
                    }
                    if heap.elements[largest] > heap.elements[i] {
                        heap.elements.swap(largest, i);
                        i = largest;
                        left_child = 2 * i + 1;
                        right_child = 2 * i + 2;
                    } else {
                        break;
                    }
                }
            }
            let max = self.elements[0];
            let cur_len = self.elements.len();
            self.elements.swap(0, cur_len - 1);
            self.elements.pop();
            heapify_down(self, 0);
            max
        }
    }
}

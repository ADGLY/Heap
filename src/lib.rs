#[cfg(test)]
mod tests;

mod heap {
    use std::marker::PhantomData;
    pub struct Heap<T: HeapImpl, U> {
        pub elements: Vec<U>,
        phantom: PhantomData<T>,
    }

    impl<T: HeapImpl, U> Heap<T, U> {
        pub fn new(size: usize) -> Self {
            Heap {
                elements: Vec::with_capacity(size),
                phantom: PhantomData,
            }
        }
    }

    pub trait HeapTrait<U> {
        fn insert(&mut self, value: U);
        fn pop(&mut self) -> U;
    }

    impl<U: std::cmp::PartialOrd + Copy> HeapTrait<U> for Heap<BinaryHeap, U> {
        fn insert(&mut self, value: U) {
            fn heapify_up<U: std::cmp::PartialOrd>(heap: &mut Heap<BinaryHeap, U>, i: usize) {
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

        fn pop(&mut self) -> U {
            fn heapify_down<U: std::cmp::PartialOrd>(heap: &mut Heap<BinaryHeap, U>, i: usize) {
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

    impl<U: std::cmp::PartialOrd + Copy> HeapTrait<U> for Heap<FouraryHeap, U> {
        fn insert(&mut self, value: U) {
            fn heapify_up<U: std::cmp::PartialOrd>(heap: &mut Heap<FouraryHeap, U>, mut i: usize) {
                while i != 0 {
                    let parent: usize = (i - 1) >> 2;
                    if heap.elements[i] > heap.elements[parent] {
                        heap.elements.swap(i, parent);
                        i = parent;
                    } else {
                        break;
                    }
                }
            }
            self.elements.push(value);
            let cur_len = self.elements.len();
            heapify_up(self, cur_len - 1);
        }

        fn pop(&mut self) -> U {
            fn heapify_down<U: std::cmp::PartialOrd>(
                heap: &mut Heap<FouraryHeap, U>,
                mut i: usize,
            ) {
                let mut child_1: usize = 1;
                let mut child_2: usize = 2;
                let mut child_3: usize = 3;
                let mut child_4: usize = 4;
                while child_4 < heap.elements.len() {
                    let largest_1: usize =
                        child_1 + (heap.elements[child_2] > heap.elements[child_1]) as usize;
                    let largest_2 =
                        child_3 + (heap.elements[child_4] > heap.elements[child_3]) as usize;
                    let mut largest = largest_1;
                    if heap.elements[largest_2] > heap.elements[largest_1] {
                        largest = largest_2;
                    }
                    if heap.elements[largest] > heap.elements[i] {
                        heap.elements.swap(largest, i);
                        i = largest;
                        let temp_idx = i << 2;
                        child_1 = temp_idx + 1;
                        child_2 = temp_idx + 2;
                        child_3 = temp_idx + 3;
                        child_4 = temp_idx + 4;
                    } else {
                        return;
                    }
                }
                if child_1 < heap.elements.len() {
                    let mut largest: usize = child_1
                        + (child_2 < heap.elements.len()
                            && heap.elements[child_2] > heap.elements[child_1])
                            as usize;
                    if child_3 < heap.elements.len() {
                        let largest_2 = child_3
                            + (child_4 < heap.elements.len()
                                && heap.elements[child_4] > heap.elements[child_3])
                                as usize;
                        if heap.elements[largest_2] > heap.elements[largest] {
                            largest = largest_2;
                        }
                    }
                    if heap.elements[largest] > heap.elements[i] {
                        heap.elements.swap(largest, i);
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

    mod inner_heap {
        use crate::heap::BinaryHeap;
        use crate::heap::FouraryHeap;

        pub trait Sealed {}
        impl Sealed for BinaryHeap {}
        impl Sealed for FouraryHeap {}
    }

    pub trait HeapImpl: inner_heap::Sealed {}
    impl HeapImpl for BinaryHeap {}
    impl HeapImpl for FouraryHeap {}

    pub struct BinaryHeap;
    pub struct FouraryHeap;
}

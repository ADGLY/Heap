#[cfg(test)]
mod tests;

pub mod heap {
    use std::marker::PhantomData;
    pub struct Heap<T: HeapImpl, U> {
        elements: Vec<U>,
        phantom: PhantomData<T>,
    }

    impl<T: HeapImpl, U: std::cmp::PartialOrd + Copy> Heap<T, U> {
        pub fn new(size: usize) -> Self {
            Heap {
                elements: Vec::with_capacity(size),
                phantom: PhantomData,
            }
        }

        pub fn max(&mut self) -> U {
            self.elements[0]
        }

        pub fn clear(&mut self) {
            //Safe because U should implement copy so no drop needed
            unsafe { self.elements.set_len(0) }
        }
    }

    pub trait HeapTrait<U: std::cmp::PartialOrd + Copy> {
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

    #[inline(always)]
    fn get_grandparent(i: usize) -> usize {
        //Hacky we could add a condition to check if i is at least 3
        i.wrapping_sub(3) / 4
    }

    fn push_up_min<U: std::cmp::PartialOrd + Copy>(heap: &mut Heap<MaxMinHeap, U>, mut i: usize) {
        let mut grandparent = get_grandparent(i);
        while grandparent < heap.elements.len() && heap.elements[i] < heap.elements[grandparent] {
            heap.elements.swap(i, grandparent);
            i = grandparent;
            grandparent = get_grandparent(i);
        }
    }

    fn push_up_max<U: std::cmp::PartialOrd + Copy>(heap: &mut Heap<MaxMinHeap, U>, mut i: usize) {
        let mut grandparent = get_grandparent(i);
        while grandparent < heap.elements.len() && heap.elements[i] > heap.elements[grandparent] {
            heap.elements.swap(i, grandparent);
            i = grandparent;
            grandparent = get_grandparent(i);
        }
    }

    #[inline(always)]
    fn get_parent(i: usize) -> usize {
        (i - 1) / 2
    }

    #[inline(always)]
    fn get_level(i: usize) -> usize {
        63 - (i + 1).leading_zeros() as usize
    }

    fn push_up<U: std::cmp::PartialOrd + Copy>(heap: &mut Heap<MaxMinHeap, U>, i: usize) {
        if i != 0 {
            let parent = get_parent(i);
            if get_level(i) % 2 != 0 {
                if heap.elements[i] > heap.elements[parent] {
                    heap.elements.swap(i, parent);
                    push_up_max(heap, parent);
                } else {
                    push_up_min(heap, i);
                }
            } else if heap.elements[i] < heap.elements[parent] {
                heap.elements.swap(i, parent);
                push_up_min(heap, parent);
            } else {
                push_up_max(heap, i);
            }
        }
    }

    #[inline(always)]
    fn has_two_children_and_four_grandchildren<U>(heap: &Heap<MaxMinHeap, U>, i: usize) -> bool {
        4 * i + 6 < heap.elements.len()
    }

    #[inline(always)]
    fn index_of_larger_child_or_grand_child<U: std::cmp::PartialOrd + Copy>(
        heap: &Heap<MaxMinHeap, U>,
        m: usize,
    ) -> usize {
        let start_idx_grand_children = 4 * m + 3;
        let larger_left_grandchildren = start_idx_grand_children
            + (heap.elements[start_idx_grand_children + 1]
                > heap.elements[start_idx_grand_children]) as usize;
        let larger_right_grandchildren = start_idx_grand_children
            + 2
            + (heap.elements[start_idx_grand_children + 3]
                > heap.elements[start_idx_grand_children + 2]) as usize;

        if heap.elements[larger_left_grandchildren] > heap.elements[larger_right_grandchildren] {
            return larger_left_grandchildren;
        }
        larger_right_grandchildren
    }

    #[inline(always)]
    fn get_number_of_descendants<U>(heap: &Heap<MaxMinHeap, U>, i: usize) -> usize {
        let heap_len = heap.elements.len();
        if heap_len <= 2 * i + 1 {
            return 0;
        }
        if heap_len <= 2 * i + 3 {
            return heap_len - 2 * i - 1;
        }
        if heap_len < 4 * i + 4 {
            return 2;
        }
        heap_len - 4 * i - 1
    }

    #[inline(always)]
    fn get_left_child(i: usize) -> usize {
        2 * i + 1
    }

    #[inline(always)]
    fn get_right_child(i: usize) -> usize {
        2 * i + 2
    }

    fn push_down_max<U: std::cmp::PartialOrd + Copy>(heap: &mut Heap<MaxMinHeap, U>, mut m: usize) {
        while has_two_children_and_four_grandchildren(heap, m) {
            let i = m;
            m = index_of_larger_child_or_grand_child(heap, i);
            if heap.elements[m] > heap.elements[i] {
                heap.elements.swap(m, i);
                let parent = get_parent(m);
                if heap.elements[m] < heap.elements[parent] {
                    heap.elements.swap(m, parent);
                }
            } else {
                return;
            }
        }
        let number_of_descendants = get_number_of_descendants(heap, m);
        match number_of_descendants {
            0 => (),
            1 => {
                let left_child = get_left_child(m);
                if heap.elements[left_child] > heap.elements[m] {
                    heap.elements.swap(m, left_child);
                }
            }
            2 => {
                let left_child = get_left_child(m);
                let right_child = get_right_child(m);
                let max_idx =
                    left_child + (heap.elements[right_child] > heap.elements[left_child]) as usize;
                if heap.elements[max_idx] > heap.elements[m] {
                    heap.elements.swap(m, max_idx);
                }
            }
            3 => {
                let right_child = get_right_child(m);
                let left_left_child = 4 * m + 3;
                let mut max_idx = left_left_child;
                if heap.elements[right_child] > heap.elements[max_idx] {
                    max_idx = right_child;
                }

                if heap.elements[max_idx] > heap.elements[m] {
                    heap.elements.swap(m, max_idx);
                }
            }
            4 => {
                let right_child = get_right_child(m);
                let left_left_child = 4 * m + 3;
                let left_right_child = 4 * m + 4;
                let mut max_idx = left_left_child;
                if heap.elements[left_right_child] > heap.elements[max_idx] {
                    max_idx = left_right_child;
                }
                if heap.elements[right_child] > heap.elements[max_idx] {
                    max_idx = right_child;
                }
                if heap.elements[max_idx] > heap.elements[m] {
                    heap.elements.swap(m, max_idx);
                }
            }
            5 => {
                let left_left_child = 4 * m + 3;
                let left_right_child = 4 * m + 4;
                let right_left_child = 4 * m + 5;
                let mut max_idx = left_left_child;
                if heap.elements[left_right_child] > heap.elements[max_idx] {
                    max_idx = left_right_child;
                }
                if heap.elements[right_left_child] > heap.elements[max_idx] {
                    max_idx = right_left_child;
                }
                if heap.elements[max_idx] > heap.elements[m] {
                    heap.elements.swap(m, max_idx);
                }
            }
            x => panic!("This case shouldn't be reachable ! Value is {} !", x),
        }
    }

    impl<U: std::cmp::PartialOrd + Copy> HeapTrait<U> for Heap<MaxMinHeap, U> {
        fn insert(&mut self, value: U) {
            self.elements.push(value);
            let cur_elem = self.elements.len();
            push_up(self, cur_elem - 1);
        }

        fn pop(&mut self) -> U {
            let max = self.elements[0];
            let cur_size = self.elements.len();
            self.elements.swap(0, cur_size - 1);
            self.elements.pop();
            push_down_max(self, 0);
            max
        }
    }
    mod inner_heap {
        use super::BinaryHeap;
        use super::FouraryHeap;
        use super::MaxMinHeap;

        pub trait Sealed {}
        impl Sealed for BinaryHeap {}
        impl Sealed for FouraryHeap {}
        impl Sealed for MaxMinHeap {}
    }

    pub trait HeapImpl: inner_heap::Sealed {}
    impl HeapImpl for BinaryHeap {}
    impl HeapImpl for FouraryHeap {}
    impl HeapImpl for MaxMinHeap {}

    pub struct BinaryHeap;
    pub struct FouraryHeap;
    pub struct MaxMinHeap;
}

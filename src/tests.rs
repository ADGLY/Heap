use crate::heap::BinaryHeap;
use crate::heap::FourAryHeap;
use crate::heap::Heap;
use crate::heap::HeapTrait;

use ordered_float::OrderedFloat;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rstest::*;

static TEST_SIZE: usize = 100_000;

use rand::Rng;
#[rstest]
#[case(Heap::<OrderedFloat<f64>>::new(TEST_SIZE), BinaryHeap)]
#[case(Heap::<u64>::new(TEST_SIZE), FourAryHeap)]
fn binary_heap<T, U>(#[case] mut heap: Heap<U>, #[case] _implementation: T)
where
    U: std::cmp::PartialOrd + Copy + std::fmt::Debug + std::cmp::Ord,
    Standard: Distribution<U>,
    Heap<U>: HeapTrait<T, U>,
{
    let mut std_heap = std::collections::BinaryHeap::new();
    let mut rng = rand::thread_rng();

    for _x in 0..TEST_SIZE {
        let value = rng.gen::<U>();
        heap.insert(value);
        std_heap.push(value);
        assert_eq!(heap.elements[0], *std_heap.peek().unwrap());
    }

    for _x in 0..TEST_SIZE - 1 {
        assert_eq!(heap.elements[0], *std_heap.peek().unwrap());

        let value = heap.pop();
        let std_value = std_heap.pop().unwrap();

        assert_eq!(value, std_value);

        assert_eq!(heap.elements[0], *std_heap.peek().unwrap());
    }
    let value = heap.pop();
    let std_value = std_heap.pop().unwrap();
    assert_eq!(value, std_value);
}

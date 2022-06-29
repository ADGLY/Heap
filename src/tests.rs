use crate::heap::BinaryHeap;
use crate::heap::FouraryHeap;
use crate::heap::Heap;
use crate::heap::HeapImpl;
use crate::heap::HeapTrait;

use ordered_float::OrderedFloat;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rstest::*;

static TEST_SIZE: usize = 100_000;

use rand::Rng;
#[rstest]
#[case(Heap::<BinaryHeap, OrderedFloat<f64>>::new(TEST_SIZE))]
#[case(Heap::<FouraryHeap, u64>::new(TEST_SIZE))]
fn binary_heap<T: HeapImpl, U>(#[case] mut heap: Heap<T, U>)
where
    U: std::cmp::PartialOrd + Copy + std::fmt::Debug + std::cmp::Ord,
    Standard: Distribution<U>,
    Heap<T, U>: HeapTrait<U>,
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

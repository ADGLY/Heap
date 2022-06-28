use crate::heap::BinaryHeap;
use crate::heap::Heap;
#[test]
fn it_works() {
    let mut heap: Heap<u64> = BinaryHeap::new(10);
    heap.insert(10);
    heap.insert(20);
    heap.insert(15);
    heap.insert(100);
    assert_eq!(heap.elements[0], 100);
}

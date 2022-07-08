use criterion::{criterion_group, criterion_main, Criterion};
use heap::heap::*;
use rand::Rng;

const TEST_SIZE: usize = 1_000_000;

fn heap_benchmark(c: &mut Criterion) {
    let mut values = Vec::<f64>::with_capacity(TEST_SIZE);
    let mut rng = rand::thread_rng();
    for _x in 0..TEST_SIZE {
        values.push(rng.gen::<f64>());
    }
    let mut heap = Heap::<BinaryHeap, f64>::new(TEST_SIZE);

    c.bench_function("insert bench", |b| {
        b.iter(|| {
            for value in &values {
                heap.insert(*value);
            }
            heap.clear();
        })
    });
}

criterion_group!(benches, heap_benchmark);
criterion_main!(benches);

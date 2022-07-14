use std::time::{Duration, Instant};

use criterion::{criterion_group, criterion_main, Criterion};
use heap::heap::*;
use rand::Rng;

const TEST_SIZE: usize = 1_000_000;

fn heap_benchmark_impl<T: HeapImpl, U: Copy + std::cmp::PartialOrd>(
    heap: &mut Heap<T, U>,
    values: &[U],
    c: &mut Criterion,
    bench_name: &str,
) where
    Heap<T, U>: HeapTrait<U>,
{
    c.bench_function(&format!("Insert {}", bench_name), |b| {
        b.iter_custom(|iters| {
            let start = Instant::now();
            for _i in 0..iters {
                for value in values {
                    heap.insert(*value);
                }
            }
            let time = start.elapsed();
            heap.clear();
            time
        });
    });
    heap.clear();

    for value in values {
        heap.insert(*value);
    }
    c.bench_function(&format!("Pop {}", bench_name), |b| {
        b.iter_custom(|iters| {
            let mut duration: Duration = Duration::default();
            for _i in 0..iters {
                let start = Instant::now();
                for _k in 0..values.len() {
                    heap.pop();
                }
                duration += start.elapsed();
                for value in values {
                    heap.insert(*value);
                }
            }
            duration
        });
    });
}

fn heap_benchmark(c: &mut Criterion) {
    let mut values = Vec::<f64>::with_capacity(TEST_SIZE);
    let mut rng = rand::thread_rng();
    for _x in 0..TEST_SIZE {
        values.push(rng.gen::<f64>());
    }
    for heap_type in ["Binary", "Fourary"].iter() {
        let bench_name = format!("bench for {} heap", heap_type);
        match *heap_type {
            "Binary" => heap_benchmark_impl(
                &mut Heap::<BinaryHeap, f64>::new(TEST_SIZE),
                &values,
                c,
                &bench_name,
            ),
            "Fourary" => {
                heap_benchmark_impl(
                    &mut Heap::<FouraryHeap, f64>::new(TEST_SIZE),
                    &values,
                    c,
                    &bench_name,
                );
            }
            _ => panic!("Heap type not found !"),
        }
    }
}

criterion_group!(benches, heap_benchmark);
criterion_main!(benches);

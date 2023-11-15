use berry::hashtree::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("insert 100", |b| {
        let mut tree = HashTree::<isize>::new();
        b.iter(|| tree.new_node(black_box(100), None))
    });

    c.bench_function("insert with parent 100", |b| {
        let (mut tree, hash) = HashTree::<isize>::from(0);
        b.iter(|| tree.new_node(black_box(100), Some(hash)))
    });

    c.bench_function("find 100", |b| {
        let (mut tree, hash) = HashTree::<isize>::from(0);
        for i in 1..99 {
            if i % 2 != 0 {
                tree.new_node(i, Some(hash));
            } else {
                tree.new_node(i, Some(HashTree::<isize>::hasher_boi(&(i - 1))));
            }
        }
        b.iter(|| tree.get(HashTree::<isize>::hasher_boi(&black_box(100))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

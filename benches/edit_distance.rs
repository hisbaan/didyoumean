use criterion::{criterion_group, criterion_main, Criterion};
use didyoumean::edit_distance;

pub fn edit_distance_bench(c: &mut Criterion) {
    let arr = vec!["abarthrosis", "abarticular", "abarticulation"];
    let search_chars = "abartclat".chars().collect::<Vec<_>>();
    c.bench_function("edit_distance", |b| {
        b.iter(|| {
            for known_term in arr.iter() {
                edit_distance(&search_chars, known_term);
            }
        })
    });
}

criterion_group!(benches, edit_distance_bench);
criterion_main!(benches);

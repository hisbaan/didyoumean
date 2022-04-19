use criterion::{criterion_group, criterion_main, Criterion};
use didyoumean::edit_distance;

pub fn edit_distance_bench(c: &mut Criterion) {
    let arr = vec!["abarthrosis", "abarticular", "abarticulation"];
    c.bench_function("edit_distance", |b| {
        b.iter(|| {
            for search_term in arr.iter() {
                edit_distance(search_term, "abartclat");
            }
        })
    });
}

criterion_group!(benches, edit_distance_bench);
criterion_main!(benches);

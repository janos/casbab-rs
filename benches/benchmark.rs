use criterion::{criterion_group, criterion_main, Criterion};

use casbab;

const BENCHMARK_PHASE: &str = "xCAMELSnakeKebab_screaming pascal XXX";

fn benchmark_camel(c: &mut Criterion) {
    c.bench_function("camel", |b| {
        b.iter(|| casbab::camel(BENCHMARK_PHASE));
    });
}

fn benchmark_pascal(c: &mut Criterion) {
    c.bench_function("pascal", |b| {
        b.iter(|| casbab::pascal(BENCHMARK_PHASE));
    });
}

fn benchmark_snake(c: &mut Criterion) {
    c.bench_function("snake", |b| {
        b.iter(|| casbab::snake(BENCHMARK_PHASE));
    });
}

fn benchmark_camel_snake(c: &mut Criterion) {
    c.bench_function("camel_snake", |b| {
        b.iter(|| casbab::camel_snake(BENCHMARK_PHASE));
    });
}

fn benchmark_screaming_snake(c: &mut Criterion) {
    c.bench_function("screaming_snake", |b| {
        b.iter(|| casbab::screaming_snake(BENCHMARK_PHASE));
    });
}

fn benchmark_kebab(c: &mut Criterion) {
    c.bench_function("kebab", |b| {
        b.iter(|| casbab::kebab(BENCHMARK_PHASE));
    });
}

fn benchmark_camel_kebab(c: &mut Criterion) {
    c.bench_function("camel_kebab", |b| {
        b.iter(|| casbab::camel_kebab(BENCHMARK_PHASE));
    });
}

fn benchmark_screaming_kebab(c: &mut Criterion) {
    c.bench_function("screaming_kebab", |b| {
        b.iter(|| casbab::screaming_kebab(BENCHMARK_PHASE));
    });
}

fn benchmark_title(c: &mut Criterion) {
    c.bench_function("title", |b| {
        b.iter(|| casbab::title(BENCHMARK_PHASE));
    });
}

fn benchmark_lower(c: &mut Criterion) {
    c.bench_function("lower", |b| {
        b.iter(|| casbab::lower(BENCHMARK_PHASE));
    });
}

fn benchmark_screaming(c: &mut Criterion) {
    c.bench_function("screaming", |b| {
        b.iter(|| casbab::screaming(BENCHMARK_PHASE));
    });
}

criterion_group!(
    benches,
    benchmark_camel,
    benchmark_pascal,
    benchmark_snake,
    benchmark_camel_snake,
    benchmark_screaming_snake,
    benchmark_kebab,
    benchmark_camel_kebab,
    benchmark_screaming_kebab,
    benchmark_title,
    benchmark_lower,
    benchmark_screaming,
);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bytes_sum(expr: &str) -> u16 {
    let chars = expr.as_bytes();

    let mut sum = 0u16;
    for i in 0..chars.len() {
        sum += chars[i] as u16;
    }

    sum
}

fn bench_eval_binary(c: &mut Criterion) {
    let expressions = ["1+1", "1 + 2", "1-2", "3.5-4", "4 * 3", "12.837/8.3"];

    let mut group = c.benchmark_group("bench_eval_binary");

    for expr in expressions.iter() {
        group.bench_with_input(BenchmarkId::new("eval_binary", expr), expr, |b, &expr| {
            b.iter(|| mexe::eval_binary(expr));
        });
        group.bench_with_input(BenchmarkId::new("read_bytes", expr), expr, |b, &expr| {
            b.iter(|| bytes_sum(expr));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_eval_binary);
criterion_main!(benches);

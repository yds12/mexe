use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! float_eq {
    ($op1:expr, $op2:expr) => {
        assert!(float_cmp::approx_eq!(f64, $op1, $op2));
    };
}

fn bench_eval(c: &mut Criterion) {
    let expressions = [
        "1 + 2",
        "4 * 3",
        "12.837/8.3",
        "2 * (1.5 - 6)",
        "(1.5 / (2 + 3 * 0.1) + 6) * 3",
        "(2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) - 1.1",
        "(2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * (((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) * (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * (((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) - 1.1",
    ];

    let mut group = c.benchmark_group("bench_eval");

    for expr in expressions.iter() {
        dbg!(mexe::eval(expr));
        dbg!(meval::eval_str(expr));
        dbg!(fasteval::ez_eval(expr, &mut fasteval::EmptyNamespace));
        float_eq!(mexe::eval(expr).unwrap(), meval::eval_str(expr).unwrap());

        group.bench_with_input(BenchmarkId::new("mexe", expr), expr, |b, &expr| {
            b.iter(|| mexe::eval(expr));
        });
        group.bench_with_input(BenchmarkId::new("meval", expr), expr, |b, &expr| {
            b.iter(|| meval::eval_str(expr));
        });
        group.bench_with_input(BenchmarkId::new("fasteval", expr), expr, |b, &expr| {
            b.iter(|| fasteval::ez_eval(expr, &mut fasteval::EmptyNamespace));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_eval);
criterion_main!(benches);

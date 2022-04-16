use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

macro_rules! float_eq {
    ($op1:expr, $op2:expr) => {
        assert!(float_cmp::approx_eq!(f64, $op1, $op2));
    };
}

const EXPRESSIONS: [&str; 10] = [
        "1 + 2",
        "2 * 3",
        "3/8.3",
        "4 * (1.5 - 6)",
        "(5 / (2 + 3 * 0.1) + 6) * 3",
        "(6 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) - 1.1",
        "(7 * (((1.5 * (0.7 - 0.33 + (6.00 * 0.05))) / (2.5 + 3 * 0.1) + 6) * 3)) - 1.1",
        "((8 + 7.07) * ((((1.5 - 0.001 + 0.005) * (0.7 - 0.33 + (6.00 * 0.05))) / (2.5 + 3 * 0.1) + 6) * 3)) - 1.1",
        "((9 + 7.07) * ((((1.5 - 0.001 + 0.005) * (0.7 - 0.33 + (6.00 * 0.05))) / (2.5 + 3 * 0.1) + 6) * 3)) - 1.1 * (1.00 * 2838.88736 * 3 / (6 - 2))",
        "(10 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * (((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) * (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * (((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + (2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3))) * 3)) / ((2 * (((2 * ((1.5 / (2 + 3 * 0.1) + 6) * 3)) / (2 + 3 * 0.1) + 6) * 3)) * 2 + 3 * 0.1) + 6) * 3)) - 1.1",
    ];

fn bench_mexe(c: &mut Criterion) {
    for expr in EXPRESSIONS.iter() {
        dbg!(mexe::eval(expr));
        float_eq!(mexe::eval(expr).unwrap(), meval::eval_str(expr).unwrap());
        let bench_id = format!("bench_mexe {}", expr);
        c.bench_function(&bench_id, |b| b.iter(|| mexe::eval(black_box(expr))));
    }
}

fn bench_cmp(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_eval");

    for expr in EXPRESSIONS.iter() {
        dbg!(mexe::eval(expr));
        dbg!(meval::eval_str(expr));
        dbg!(fasteval::ez_eval(expr, &mut fasteval::EmptyNamespace));
        float_eq!(mexe::eval(expr).unwrap(), meval::eval_str(expr).unwrap());

        group.bench_with_input(
            BenchmarkId::new("bench_cmp mexe", expr),
            expr,
            |b, &expr| {
                b.iter(|| mexe::eval(expr));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("bench_cmp meval", expr),
            expr,
            |b, &expr| {
                b.iter(|| meval::eval_str(expr));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("bench_cmp fasteval", expr),
            expr,
            |b, &expr| {
                b.iter(|| fasteval::ez_eval(expr, &mut fasteval::EmptyNamespace));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_cmp, bench_mexe);
criterion_main!(benches);

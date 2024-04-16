use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use polkavm::BackendKind;

use compare_runtimes::*;

fn bench(
    c: &mut Criterion,
    group_name: &str,
    input_evm: (Vec<u8>, Vec<u8>),
    input_pvm: (Vec<u8>, Vec<u8>),
) {
    let mut group = c.benchmark_group(group_name);

    group.bench_with_input(BenchmarkId::new("PrepareEvm", 0), &input_evm, |b, (code, data)| {
        b.iter(|| runtimes::evm::prepare(code.clone(), data.clone()))
    });

    group.bench_with_input(
        BenchmarkId::new("PreparePolkaVMInterpreter", 0),
        &input_pvm,
        |b, (code, data)| {
            b.iter(|| {
                runtimes::polkavm::prepare_pvm(code, data, BackendKind::Interpreter);
            });
        },
    );

    group.bench_with_input(
        BenchmarkId::new("PreparePolkaVM", 0),
        &input_pvm,
        |b, (code, data)| {
            b.iter(|| {
                runtimes::polkavm::prepare_pvm(code, data, BackendKind::Compiler);
            });
        },
    );
}

fn bench_baseline(c: &mut Criterion) {
    bench(
        c,
        "Baseline",
        cases::evm::baseline(),
        cases::polkavm::baseline(),
    );
}

fn bench_odd_product(c: &mut Criterion) {
    bench(
        c,
        "OddProduct",
        cases::evm::odd_product(0),
        cases::polkavm::odd_product(0),
    );
}

fn bench_triangle_number(c: &mut Criterion) {
    bench(
        c,
        "TriangleNumber",
        cases::evm::triangle_number(0),
        cases::polkavm::triangle_number(0),
    );
}

fn bench_fibonacci_recursive(c: &mut Criterion) {
    bench(
        c,
        "FibonacciRecursive",
        cases::evm::fib_recursive(0),
        cases::polkavm::fib_recursive(0),
    );
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    bench(
        c,
        "FibonacciIterative",
        cases::evm::fib_iterative(0),
        cases::polkavm::fib_iterative(0),
    );
}

//fn bench_fibonacci_iterative_unchecked(c: &mut Criterion) {
//    bench(
//        c,
//        "FibonacciIterativeUnchecked",
//        cases::evm::fib_iterative_unchecked(0),
//        cases::polkavm::fib_iterative_unchecked(0),
//    );
//}

fn bench_fibonacci_binet(c: &mut Criterion) {
    bench(
        c,
        "FibonacciBinet",
        cases::evm::fib_binet(0),
        cases::polkavm::fib_binet(0),
    );
}

fn bench_erc20(c: &mut Criterion) {
    bench(c, "ERC20", cases::evm::erc20(), cases::polkavm::erc20());
}

criterion_group!(
    name = prepare;
    config = Criterion::default();
    targets = bench_baseline,
    bench_odd_product,
    bench_triangle_number,
    bench_fibonacci_recursive,
    bench_fibonacci_iterative,
    //bench_fibonacci_iterative_unchecked,
    bench_fibonacci_binet,
    bench_erc20
);
criterion_main!(prepare);

use std::time::Duration;

use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use polkavm::BackendKind;
use pprof::criterion::{Output, PProfProfiler};

use compare_runtimes::*;

fn bench<'a, P, I, J, M>(
    mut group: BenchmarkGroup<'a, M>,
    parameters: &[P],
    input_evm: I,
    input_pvm: J,
) where
    P: Copy + std::fmt::Display,
    I: Fn(P) -> (Vec<u8>, Vec<u8>),
    J: Fn(P) -> (Vec<u8>, Vec<u8>),
    M: Measurement,
{
    for p in parameters {
        let (evm_code, evm_data) = input_evm(*p);
        let vm = runtimes::evm::prepare(evm_code, evm_data);
        group.bench_with_input(BenchmarkId::new("EVM", p), p, move |b, _| {
            b.iter(|| {
                runtimes::evm::execute(vm.clone());
            });
        });

        let (pvm_code, pvm_data) = input_pvm(*p);
        let (state, pre, export) =
            runtimes::polkavm::prepare_pvm(&pvm_code, &pvm_data, BackendKind::Interpreter);
        group.bench_with_input(BenchmarkId::new("PolkaVMInterpreter", p), p, |b, _| {
            b.iter(|| {
                revive_integration::mock_runtime::call(state.clone(), &pre, export);
            });
        });

        let (pvm_code, pvm_data) = input_pvm(*p);
        let (state, pre, export) =
            runtimes::polkavm::prepare_pvm(&pvm_code, &pvm_data, BackendKind::Compiler);
        group.bench_with_input(BenchmarkId::new("PolkaVM", p), p, |b, _| {
            b.iter(|| {
                revive_integration::mock_runtime::call(state.clone(), &pre, export);
            });
        });
    }
}

fn bench_odd_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("OddProduct");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(60));
    bench(
        group,
        &[2_000_000u32, 4_000_000, 8_000_000, 120_000_000],
        |p| cases::evm::odd_product(p),
        |p| cases::polkavm::odd_product(p),
    );
}

fn bench_triangle_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("TriangleNumber");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(60));
    bench(
        group,
        &[3_000_000i64, 6_000_000, 12_000_000, 180_000_000],
        |p| cases::evm::triangle_number(p),
        |p| cases::polkavm::triangle_number(p),
    );
}

fn bench_fibonacci_recurisve(c: &mut Criterion) {
    bench(
        c.benchmark_group("FibonacciRecursive"),
        &[8, 12, 16, 18, 20],
        |p| cases::evm::fib_recursive(p),
        |p| cases::polkavm::fib_recursive(p),
    );
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    bench(
        c.benchmark_group("FibonacciIterative"),
        &[32, 64, 128, 256],
        |p| cases::evm::fib_iterative(p),
        |p| cases::polkavm::fib_iterative(p),
    );
}

fn bench_fibonacci_binet(c: &mut Criterion) {
    bench(
        c.benchmark_group("FibonacciBinet"),
        &[32, 64, 128, 256],
        |p| cases::evm::fib_binet(p),
        |p| cases::polkavm::fib_binet(p),
    );
}

fn bench_fibonacci_iterative_unchecked(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciIterativeUnchecked");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(60));
    bench(
        group,
        &[32, 64, 128, 256, 4096, 50_400_000],
        |p| cases::evm::fib_iterative_unchecked(p),
        |p| cases::polkavm::fib_iterative_unchecked(p),
    );
}

fn bench_baseline(c: &mut Criterion) {
    bench(
        c.benchmark_group("Baseline"),
        &[0u8],
        |_| cases::evm::baseline(),
        |_| cases::polkavm::baseline(),
    );
}

criterion_group!(
    name = runtime;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_baseline,
    bench_odd_product,
    bench_triangle_number,
    bench_fibonacci_recurisve,
    bench_fibonacci_iterative,
    bench_fibonacci_iterative_unchecked,
    bench_fibonacci_binet
);
criterion_main!(runtime);

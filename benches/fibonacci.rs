use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion};
use pprof::criterion::{Output, PProfProfiler};

use compare_runtimes::*;

fn bench_fibonacci_recurisve(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciRecursive");

    for n in [8, 12, 16, 18, 20].iter() {
        let (evm_code, evm_data) = cases::evm::fib_recursive(*n);
        group.bench_with_input(BenchmarkId::new("EVM", n), n, move |b, _| {
            b.iter(|| {
                let vm = runtimes::evm::prepare(evm_code.clone(), evm_data.clone());
                runtimes::evm::execute(vm);
            })
        });

        let (pvm_code, pvm_data) = cases::polkavm::fib_recursive(*n);
        let (state, pre) = runtimes::polkavm::prepare(&pvm_code, pvm_data.clone());
        group.bench_with_input(BenchmarkId::new("PolkaVM", n), n, |b, _| {
            b.iter(|| {
                runtimes::polkavm::call(state.clone(), pre.clone());
            });
        });
    }
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciIterative");

    for n in [32, 64, 128, 256].iter() {
        let (evm_code, evm_data) = cases::evm::fib_iterative(*n);
        group.bench_with_input(BenchmarkId::new("EVM", n), n, move |b, _| {
            b.iter(|| {
                let vm = runtimes::evm::prepare(evm_code.clone(), evm_data.clone());
                runtimes::evm::execute(vm);
            })
        });

        let (pvm_code, pvm_data) = cases::polkavm::fib_iterative(*n);
        let (state, pre) = runtimes::polkavm::prepare(&pvm_code, pvm_data.clone());
        group.bench_with_input(BenchmarkId::new("PolkaVM", n), n, |b, _| {
            b.iter(|| {
                runtimes::polkavm::call(state.clone(), pre.clone());
            });
        });
    }
}

fn bench_fibonacci_binet(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciBinet");

    for n in [32, 64, 128, 256].iter() {
        let (evm_code, evm_data) = cases::evm::fib_binet(*n);
        group.bench_with_input(BenchmarkId::new("EVM", n), n, move |b, _| {
            b.iter(|| {
                let vm = runtimes::evm::prepare(evm_code.clone(), evm_data.clone());
                runtimes::evm::execute(vm);
            })
        });

        let (pvm_code, pvm_data) = cases::polkavm::fib_binet(*n);
        let (state, pre) = runtimes::polkavm::prepare(&pvm_code, pvm_data.clone());
        group.bench_with_input(BenchmarkId::new("PolkaVM", n), n, |b, _| {
            b.iter(|| {
                runtimes::polkavm::call(state.clone(), pre.clone());
            });
        });
    }
}

fn bench_fibonacci_prepare(c: &mut Criterion) {
    let mut group = c.benchmark_group("FibonacciPrepare");

    let (evm_code, evm_data) = cases::evm::fib_binet(0);
    group.bench_with_input(
        BenchmarkId::new("EvmBinet", 0),
        &(&evm_code, &evm_data),
        |b, _| b.iter(|| runtimes::evm::prepare(evm_code.clone(), evm_data.clone())),
    );

    let (evm_code, evm_data) = cases::evm::fib_iterative(0);
    group.bench_with_input(
        BenchmarkId::new("EvmIterative", 0),
        &(&evm_code, &evm_data),
        |b, _| b.iter(|| runtimes::evm::prepare(evm_code.clone(), evm_data.clone())),
    );

    let (pvm_code, pvm_data) = cases::polkavm::fib_binet(0);
    group.bench_with_input(
        BenchmarkId::new("PolkaVMBinet", 0),
        &(&pvm_code, &pvm_data),
        |b, _| {
            b.iter(|| {
                runtimes::polkavm::prepare(&pvm_code, pvm_data.clone());
            });
        },
    );

    let (pvm_code, pvm_data) = cases::polkavm::fib_iterative(0);
    group.bench_with_input(
        BenchmarkId::new("PolkaVMIterative", 0),
        &(&pvm_code, &pvm_data),
        |b, _| {
            b.iter(|| {
                runtimes::polkavm::prepare(&pvm_code, pvm_data.clone());
            });
        },
    );
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench_fibonacci_recurisve,
    bench_fibonacci_iterative,
    bench_fibonacci_binet,
    bench_fibonacci_prepare
);
criterion_main!(benches);

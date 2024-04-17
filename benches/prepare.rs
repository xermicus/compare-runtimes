use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use polkavm::BackendKind;

use compare_runtimes::{runtimes::polkavm::instantiate_engine, *};

fn bench(
    c: &mut Criterion,
    group_name: &str,
    input_evm: (Vec<u8>, Vec<u8>),
    input_pvm: (Vec<u8>, Vec<u8>),
) {
    let mut group = c.benchmark_group(group_name);
    let code_size = input_pvm.0.len();

    group.bench_with_input(
        BenchmarkId::new("Evm", code_size),
        &input_evm,
        |b, (code, data)| b.iter(|| runtimes::evm::prepare(code.clone(), data.clone())),
    );

    {
        let engine = instantiate_engine(BackendKind::Interpreter);
        group.bench_with_input(
            BenchmarkId::new("PvmInterpreterCompile", code_size),
            &(&input_pvm.0, engine),
            |b, (code, engine)| {
                b.iter(|| {
                    revive_integration::mock_runtime::recompile_code(code, &engine);
                });
            },
        );
    }

    {
        let engine = instantiate_engine(BackendKind::Compiler);
        group.bench_with_input(
            BenchmarkId::new("PvmCompile", code_size),
            &(&input_pvm.0, engine),
            |b, (code, engine)| {
                b.iter(|| {
                    revive_integration::mock_runtime::recompile_code(code, engine);
                });
            },
        );
    }

    {
        let engine = instantiate_engine(BackendKind::Interpreter);
        let module = revive_integration::mock_runtime::recompile_code(&input_pvm.0, &engine);
        group.bench_with_input(
            BenchmarkId::new("PvmInterpreterInstantiate", code_size),
            &(module, engine),
            |b, (module, engine)| {
                b.iter(|| {
                    revive_integration::mock_runtime::instantiate_module(module, engine);
                });
            },
        );
    }

    {
        let engine = instantiate_engine(BackendKind::Compiler);
        let module = revive_integration::mock_runtime::recompile_code(&input_pvm.0, &engine);
        group.bench_with_input(
            BenchmarkId::new("PvmInstantiate", code_size),
            &(module, engine),
            |b, (module, engine)| {
                b.iter(|| {
                    revive_integration::mock_runtime::instantiate_module(module, engine);
                });
            },
        );
    }
}

fn bench_baseline(c: &mut Criterion) {
    bench(
        c,
        "PrepareBaseline",
        cases::evm::baseline(),
        cases::polkavm::baseline(),
    );
}

fn bench_odd_product(c: &mut Criterion) {
    bench(
        c,
        "PrepareOddProduct",
        cases::evm::odd_product(0),
        cases::polkavm::odd_product(0),
    );
}

fn bench_triangle_number(c: &mut Criterion) {
    bench(
        c,
        "PrepareTriangleNumber",
        cases::evm::triangle_number(0),
        cases::polkavm::triangle_number(0),
    );
}

fn bench_fibonacci_recursive(c: &mut Criterion) {
    bench(
        c,
        "PrepareFibonacciRecursive",
        cases::evm::fib_recursive(0),
        cases::polkavm::fib_recursive(0),
    );
}

fn bench_fibonacci_iterative(c: &mut Criterion) {
    bench(
        c,
        "PrepareFibonacciIterative",
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
        "PrepareFibonacciBinet",
        cases::evm::fib_binet(0),
        cases::polkavm::fib_binet(0),
    );
}

fn bench_erc20(c: &mut Criterion) {
    bench(
        c,
        "PrepareERC20",
        cases::evm::erc20(),
        cases::polkavm::erc20(),
    );
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

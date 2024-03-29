pub fn odd_product(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/Computation.pvm").to_vec(),
        super::evm::odd_product(n).1,
    )
}

pub fn triangle_number(n: i64) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/Computation.pvm").to_vec(),
        super::evm::triangle_number(n).1,
    )
}

pub fn fib_recursive(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciRecursive.pvm").to_vec(),
        super::evm::fib_recursive(n).1,
    )
}

pub fn fib_iterative(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciIterative.pvm").to_vec(),
        super::evm::fib_iterative(n).1,
    )
}

pub fn fib_binet(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciBinet.pvm").to_vec(),
        super::evm::fib_binet(n).1,
    )
}

pub fn baseline() -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/Baseline.pvm").to_vec(),
        super::evm::baseline().1,
    )
}

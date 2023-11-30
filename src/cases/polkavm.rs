use parity_scale_codec::Encode;
use primitive_types::U256;

use super::FIB3;

pub fn fib_recursive(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciRecursive.pvm").to_vec(),
        (FIB3, U256::from(n)).encode(),
    )
}

pub fn fib_iterative(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciIterative.pvm").to_vec(),
        (FIB3, U256::from(n)).encode(),
    )
}

pub fn fib_binet(n: u32) -> (Vec<u8>, Vec<u8>) {
    (
        include_bytes!("../../cases/FibonacciBinet.pvm").to_vec(),
        (FIB3, U256::from(n)).encode(),
    )
}

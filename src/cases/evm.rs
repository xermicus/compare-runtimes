use parity_scale_codec::Encode;
use primitive_types::U256;

use super::FIB3;

pub fn fib_recursive(n: u32) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!("../../cases/FibonacciRecursive.bin-runtime")).unwrap(),
        (FIB3, buf).encode(),
    )
}

pub fn fib_iterative(n: u32) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!("../../cases/FibonacciIterative.bin-runtime")).unwrap(),
        (FIB3, buf).encode(),
    )
}

pub fn fib_binet(n: u32) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!("../../cases/FibonacciBinet.bin-runtime")).unwrap(),
        (FIB3, buf).encode(),
    )
}

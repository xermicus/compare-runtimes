use parity_scale_codec::Encode;
use primitive_types::U256;

use super::{BASELINE, FIB3, ODD_PRODUCT, TRIANGLE_NUMBER};

pub fn odd_product(n: u32) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!("../../cases/Computation.bin-runtime")).unwrap(),
        (ODD_PRODUCT, buf).encode(),
    )
}

pub fn triangle_number(n: i64) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!("../../cases/Computation.bin-runtime")).unwrap(),
        (TRIANGLE_NUMBER, buf).encode(),
    )
}

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

pub fn fib_iterative_unchecked(n: u32) -> (Vec<u8>, Vec<u8>) {
    let mut buf = [0; 32];
    U256::from(n).to_big_endian(&mut buf);
    (
        hex::decode(include_str!(
            "../../cases/FibonacciIterativeUnchecked.bin-runtime"
        ))
        .unwrap(),
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

pub fn erc20() -> (Vec<u8>, Vec<u8>) {
    let buf = [0; 32];
    (
        hex::decode(include_str!("../../cases/ERC20.bin-runtime")).unwrap(),
        (FIB3, buf).encode(),
    )
}

pub fn baseline() -> (Vec<u8>, Vec<u8>) {
    (
        hex::decode(include_str!("../../cases/Baseline.bin-runtime")).unwrap(),
        BASELINE.encode(),
    )
}

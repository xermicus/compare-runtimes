pub mod cases;
pub mod runtimes;

#[cfg(test)]
mod tests {
    use parity_scale_codec::Decode;
    use polkavm::BackendKind;
    use primitive_types::U256;

    use crate::*;

    #[test]
    fn fibonacci_works() {
        let check = |expected: u32, evm_code, evm_input, polkavm_code: Vec<u8>, polkavm_input| {
            let evm = runtimes::evm::prepare(evm_code, evm_input);
            let evm_ret = U256::from_big_endian(&runtimes::evm::execute(evm));
            assert_eq!(evm_ret.as_u32(), expected);

            let (state, instance) =
                runtimes::polkavm::prepare(&polkavm_code, polkavm_input, BackendKind::Compiler);
            let state = runtimes::polkavm::call(state, instance);
            let polkavm_ret = U256::decode(&mut &state.output.1[..]).unwrap();

            assert_eq!(polkavm_ret, evm_ret);
        };

        let fibonacci_sequence = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        ];

        for (n, expected) in (0..20).zip(fibonacci_sequence.iter()) {
            let (evm_code, evm_input) = cases::evm::fib_recursive(n);
            let (polkavm_code, polkavm_data) = cases::polkavm::fib_recursive(n);
            check(*expected, evm_code, evm_input, polkavm_code, polkavm_data);

            let (evm_code, evm_input) = cases::evm::fib_iterative(n);
            let (polkavm_code, polkavm_input) = cases::polkavm::fib_iterative(n);
            check(*expected, evm_code, evm_input, polkavm_code, polkavm_input);

            let (evm_code, evm_input) = cases::evm::fib_binet(n);
            let (polkavm_code, polkavm_input) = cases::polkavm::fib_binet(n);
            check(*expected, evm_code, evm_input, polkavm_code, polkavm_input);
        }
    }
}

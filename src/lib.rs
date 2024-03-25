pub mod cases;
pub mod runtimes;

#[cfg(test)]
mod tests {
    use polkavm::BackendKind;
    use primitive_types::U256;

    use crate::*;

    #[test]
    fn baseline_works() {
        let (evm_code, evm_input) = cases::evm::baseline();
        let (polkavm_code, polkavm_input) = cases::polkavm::baseline();

        let evm = runtimes::evm::prepare(evm_code, evm_input.clone());
        assert_eq!(runtimes::evm::execute(evm), Vec::<u8>::new());

        let (state, pre, export) = runtimes::polkavm::prepare_pvm(
            &polkavm_code[..],
            &polkavm_input[..],
            BackendKind::Interpreter,
        );
        let state = revive_integration::mock_runtime::call(state, &pre, export);
        assert_eq!(state.output.flags, 0);
        assert_eq!(state.output.data, Vec::<u8>::new());
    }

    #[test]
    fn fibonacci_works() {
        let check =
            |expected: u32, evm_code, evm_input, polkavm_code: Vec<u8>, polkavm_input: Vec<u8>| {
                let evm = runtimes::evm::prepare(evm_code, evm_input);
                let evm_ret = U256::from_big_endian(&runtimes::evm::execute(evm));
                assert_eq!(evm_ret.as_u32(), expected);

                let (state, pre, export) = runtimes::polkavm::prepare_pvm(
                    &polkavm_code[..],
                    &polkavm_input[..],
                    BackendKind::Interpreter,
                );
                let state = revive_integration::mock_runtime::call(state, &pre, export);
                let polkavm_ret = U256::from_big_endian(&mut &state.output.data[..]);

                assert_eq!(polkavm_ret, evm_ret);
            };

        let fibonacci_sequence = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        ];

        for (n, expected) in fibonacci_sequence
            .iter()
            .enumerate()
            .map(|(n, expected)| (n as u32, *expected))
        {
            let (evm_code, evm_input) = cases::evm::fib_recursive(n);
            let (polkavm_code, polkavm_input) = cases::polkavm::fib_recursive(n);
            check(
                expected,
                evm_code,
                evm_input.clone(),
                polkavm_code,
                polkavm_input,
            );

            let (evm_code, evm_input) = cases::evm::fib_iterative(n);
            let (polkavm_code, polkavm_input) = cases::polkavm::fib_iterative(n);
            check(
                expected,
                evm_code,
                evm_input.clone(),
                polkavm_code,
                polkavm_input,
            );

            let (evm_code, evm_input) = cases::evm::fib_binet(n);
            let (polkavm_code, polkavm_input) = cases::polkavm::fib_binet(n);
            check(
                expected,
                evm_code,
                evm_input.clone(),
                polkavm_code,
                polkavm_input,
            );
        }
    }
}

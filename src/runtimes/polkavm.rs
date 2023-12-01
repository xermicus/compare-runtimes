use parity_scale_codec::Encode;
use polkavm::{
    BackendKind, Caller, Config, Engine, ExecutionConfig, Gas, GasMeteringKind, InstancePre,
    Linker, Module, ModuleConfig, ProgramBlob, Trap,
};

#[derive(Clone, Debug)]
pub struct State {
    pub input: Vec<u8>,
    pub output: (u32, Vec<u8>),
    pub value: u128,
}

fn link_host_functions(engine: &Engine) -> Linker<State> {
    let mut linker = Linker::new(engine);

    linker
        .func_wrap(
            "input",
            |caller: Caller<State>, out_ptr: u32, out_len_ptr: u32| -> Result<(), Trap> {
                let (mut caller, state) = caller.split();

                caller.write_memory(out_ptr, &state.input)?;
                caller.write_memory(out_len_ptr, &(state.input.len() as u32).encode())?;

                Ok(())
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "value_transferred",
            |caller: Caller<State>, out_ptr: u32, out_len_ptr: u32| -> Result<(), Trap> {
                let (mut caller, state) = caller.split();

                let value = state.value.encode();

                caller.write_memory(out_ptr, &value)?;
                caller.write_memory(out_len_ptr, &(value.len() as u32).encode())?;

                Ok(())
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "seal_return",
            |caller: Caller<State>, flags: u32, data_ptr: u32, data_len: u32| -> Result<(), Trap> {
                let (caller, state) = caller.split();

                state.output.0 = flags;
                state.output.1 = caller.read_memory_into_new_vec(data_ptr, data_len)?;

                Err(Default::default())
            },
        )
        .unwrap();

    linker
        .func_wrap(
            "debug_message",
            |caller: Caller<State>, str_ptr: u32, str_len: u32| -> Result<u32, Trap> {
                let (caller, _) = caller.split();

                let data = caller.read_memory_into_new_vec(str_ptr, str_len)?;
                print!("debug_message: {}", String::from_utf8(data).unwrap());

                Ok(0)
            },
        )
        .unwrap();

    linker
}

pub fn prepare(code: &[u8], input: Vec<u8>, backend: BackendKind) -> (State, InstancePre<State>) {
    let blob = ProgramBlob::parse(code).unwrap();

    let mut config = Config::new();
    config.set_allow_insecure(false);
    config.set_backend(Some(backend));

    let engine = Engine::new(&config).unwrap();

    let mut module_config = ModuleConfig::new();
    module_config.set_gas_metering(Some(GasMeteringKind::Async));

    let module = Module::from_blob(&engine, &module_config, &blob).unwrap();

    let func = link_host_functions(&engine)
        .instantiate_pre(&module)
        .unwrap();

    let state = State {
        input,
        output: Default::default(),
        value: Default::default(),
    };

    (state, func)
}

pub fn call(mut state: State, on: InstancePre<State>) -> State {
    let mut config = ExecutionConfig::default();
    config.set_gas(Gas::MAX);

    on.instantiate()
        .unwrap()
        .get_func("call")
        .unwrap()
        .call_ex(&mut state, &[], config)
        .unwrap_err();

    state
}

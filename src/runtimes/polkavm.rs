use polkavm::{BackendKind, Config, ExportIndex, InstancePre, SandboxKind};
use revive_integration::mock_runtime;
use revive_integration::mock_runtime::State;

pub fn prepare_pvm(
    code: &[u8],
    input: &[u8],
    backend: BackendKind,
) -> (State, InstancePre<State>, ExportIndex) {
    let mut config = Config::new();
    config.set_backend(Some(backend));
    config.set_sandbox(Some(SandboxKind::Linux));

    let (instance_pre, export_index) = mock_runtime::prepare(code, Some(config));

    (State::new(input.to_vec()), instance_pre, export_index)
}

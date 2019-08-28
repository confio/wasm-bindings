use wasmer_runtime::{compile, compile_with, error, imports};
use wasmer_runtime_core::types::Value;
use wasmer_runtime_core::Instance;

use wasmer_middleware_common::metering;
use wasmer_runtime_core::backend::Compiler;
//use wasmer_runtime::Ctx;

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] =
    include_bytes!("../contracts/hasher/target/wasm32-unknown-unknown/release/hasher.wasm");


pub fn get_metered_compiler(limit: u64, metering: bool) -> impl Compiler {
    use wasmer_runtime_core::codegen::{MiddlewareChain, StreamingCompiler};
    use wasmer_singlepass_backend::ModuleCodeGenerator as SinglePassMCG;
    let c: StreamingCompiler<SinglePassMCG, _, _, _, _> = StreamingCompiler::new(move || {
        let mut chain = MiddlewareChain::new();
        if metering {
            chain.push(metering::Metering::new(limit));
        }
        chain
    });
    c
}

pub fn setup() -> error::Result<Instance> {
    let import_object = imports! {};
//    let module = compile_with(&WASM, &compiler::get_metered_compiler(60000, true)).unwrap();
    let module = compile(&WASM)?;
    module.instantiate(&import_object)
}

// just panic here if we can't run
pub fn run(inst: &Instance, start: i32, step: i32) -> i32 {
    let result = inst.call("hash", &[start.into(), step.into()]).unwrap();
    if let Value::I32(val) = result[0] {
        val
    } else {
        panic!("result not i32");
    }
}

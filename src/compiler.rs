use wasmer_runtime::{compile_with, error, imports};
use wasmer_runtime_core::types::Value;
use wasmer_runtime_core::Instance;

use wasmer_middleware_common::metering;
use wasmer_runtime_core::backend::Compiler;

use wasmer_clif_backend::CraneliftCompiler;
use wasmer_singlepass_backend::SinglePassCompiler;

#[cfg(feature = "llvm")]
use wasmer_llvm_backend::LLVMCompiler;

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../target/wasm32-unknown-unknown/release/hasher.wasm");

fn get_metered_compiler(limit: u64, metering: bool) -> impl Compiler {
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

#[cfg(feature = "llvm")]
fn get_metered_llvm_compiler(limit: u64, metering: bool) -> impl Compiler {
    use wasmer_llvm_backend::ModuleCodeGenerator as LlvmMCG;
    use wasmer_runtime_core::codegen::{MiddlewareChain, StreamingCompiler};
    let c: StreamingCompiler<LlvmMCG, _, _, _, _> = StreamingCompiler::new(move || {
        let mut chain = MiddlewareChain::new();
        if metering {
            chain.push(metering::Metering::new(limit));
        }
        chain
    });
    c
}

pub fn setup_singlepass() -> error::Result<Instance> {
    let import_object = imports! {};
    let module = compile_with(&WASM, &SinglePassCompiler::new())?;
    module.instantiate(&import_object)
}

pub fn setup_clif() -> error::Result<Instance> {
    let import_object = imports! {};
    let module = compile_with(&WASM, &CraneliftCompiler::new())?;
    module.instantiate(&import_object)
}

#[cfg(feature = "llvm")]
pub fn setup_llvm() -> error::Result<Instance> {
    let import_object = imports! {};
    let module = compile_with(&WASM, &LLVMCompiler::new())?;
    module.instantiate(&import_object)
}

pub fn setup_metered(limit: u64) -> error::Result<Instance> {
    let import_object = imports! {};
    let module = compile_with(&WASM, &get_metered_compiler(limit, true))?;
    module.instantiate(&import_object)
}

#[cfg(feature = "llvm")]
pub fn setup_metered_llvm(limit: u64) -> error::Result<Instance> {
    let import_object = imports! {};
    let module = compile_with(&WASM, &get_metered_llvm_compiler(limit, true))?;
    module.instantiate(&import_object)
}

// just panic here if we can't run
pub fn run(inst: &Instance, count: i32, start: i32, step: i32) -> i32 {
    let result = inst
        .call("hash", &[count.into(), start.into(), step.into()])
        .unwrap();
    if let Value::I32(val) = result[0] {
        val
    } else {
        panic!("result not i32");
    }
}

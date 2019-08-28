use wasmer_runtime_core::backend::Compiler;
use wasmer_middleware_common::metering;
//use wasmer_runtime::Ctx;

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

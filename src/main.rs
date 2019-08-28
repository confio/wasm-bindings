mod compiler;

use wasmer_runtime_core::types::Value;

use wasmer_runtime::{
    imports,
    error,
    compile_with
};
use wasmer_middleware_common::metering;

// Make sure that the compiled wasm-sample-app is accessible at this path.
static WASM: &'static [u8] = include_bytes!("../contracts/hasher/target/wasm32-unknown-unknown/release/hasher.wasm");

fn main() -> error::Result<()> {
    let import_object = imports! { };

    let module = compile_with(&WASM, &compiler::get_metered_compiler(60000, true)).unwrap();

    // Compile our webassembly into an `Instance`.
    let instance = module.instantiate(&import_object)?;

    let used = metering::get_points_used(&instance);
    println!("Points used: {}", used);

    // Run hash
    let result = instance.call("hash", &[19.into(), 20.into()])?;
    let Value::I32(val) = result[0];
    println!("hash(19, 20) = {} - should be 2300", val);
    //    if let Value::I32(val) = result[0] {
//        println!("hash(19, 20) = {} - should be 2300", val);
//    } else {
//        println!("Got unknown result: {:?}", result[0]);
//    }

    let used = metering::get_points_used(&instance);
    println!("Points used: {}", used);

    Ok(())
}
mod compiler;
//mod bench;

use wasmer_runtime::{error};

use wasmer_middleware_common::metering;

fn main() -> error::Result<()> {
    let instance = compiler::setup()?;

    println!("Points used: {}", metering::get_points_used(&instance));

    let val = compiler::run(&instance, 19, 20);
    println!("hash(19, 20) = {} - should be 2300", val);

    println!("Points used: {}", metering::get_points_used(&instance));
    Ok(())
}

pub mod compiler;

pub use compiler::{run, setup};
use wasmer_runtime::{error};
use wasmer_middleware_common::metering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution() {
        let instance = compiler::setup().unwrap();

        assert_eq!(0, metering::get_points_used(&instance));

        let val = compiler::run(&instance, 19, 20);
        assert_eq!(2300, val);

        // TODO: reqire this to actually meter
        assert_eq!(0, metering::get_points_used(&instance));
    }
}


pub mod compiler;

pub use compiler::{run, setup};
use wasmer_middleware_common::metering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution() {
        let instance = compiler::setup().unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        assert_eq!(0, metering::get_points_used(&instance));
    }

    #[test]
    fn test_metered_execution() {
        // 20k limit
        let instance = compiler::setup_metered(20000).unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        // sha is around 10k gas?
        assert_eq!(metering::get_points_used(&instance), 9730);
    }

    #[test]
    fn test_metered_execution_100() {
        // 1 million limit
        let instance = compiler::setup_metered(1000000).unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 100, 19, 20);
        assert_eq!(2555, val);
        // 100 sha in a loop is a little cheaper than 100 separate calls
        assert_eq!(metering::get_points_used(&instance), 940535);
    }
}


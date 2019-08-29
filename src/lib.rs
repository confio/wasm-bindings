pub mod compiler;

pub use compiler::{run, setup_clif, setup_metered, setup_singlepass};

#[cfg(feature = "llvm")]
pub use compiler::{setup_llvm, setup_metered_llvm};

#[cfg(test)]
mod tests {
    use super::*;
    use wasmer_middleware_common::metering;

    #[test]
    fn test_execution_singlepass() {
        let instance = compiler::setup_singlepass().unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        assert_eq!(0, metering::get_points_used(&instance));
    }

    #[test]
    fn test_execution_clif() {
        let instance = compiler::setup_clif().unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        assert_eq!(0, metering::get_points_used(&instance));
    }

    #[cfg(feature = "llvm")]
    #[test]
    fn test_execution_llvm() {
        let instance = compiler::setup_llvm().unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        assert_eq!(0, metering::get_points_used(&instance));
    }

    #[test]
    fn test_metered_execution() {
        // 20k limit
        let instance = compiler::setup_metered(70000).unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        // sha is around 10k gas? depends on how we build the wasm file
        let used = metering::get_points_used(&instance);
        assert!(used > 9000, "used {}", used);
        assert!(used < 14000, "used {}", used);
    }

    // Note: this triggers an llvm compilation error
    #[cfg(feature = "llvm")]
    #[test]
    fn test_metered_execution_llvm() {
        // 20k limit
        let instance = compiler::setup_metered_llvm(70000).unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 1, 19, 20);
        assert_eq!(2300, val);
        // sha is around 10k gas? depends on how we build the wasm file
        let used = metering::get_points_used(&instance);
        assert!(used > 9000, "used {}", used);
        assert!(used < 14000, "used {}", used);
    }

    #[test]
    fn test_metered_execution_100() {
        // 1 million limit
        let instance = compiler::setup_metered(7000000).unwrap();
        assert_eq!(0, metering::get_points_used(&instance));
        let val = compiler::run(&instance, 100, 19, 20);
        assert_eq!(2555, val);
        // 100 sha is around 1.15m gas? depends on how we build the wasm file
        let used = metering::get_points_used(&instance);
        assert!(used > 1140000, "used {}", used);
        assert!(used < 1240000, "used {}", used);
    }
}

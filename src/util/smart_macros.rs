#[macro_export]
macro_rules! smart {
    ($x:expr, $p:expr) => {
        $crate::util::smart::SmartF64 {
            value: $x,
            precision: $p,
            mode: $crate::util::smart::Mode::Round,
        }
    };
    (trunc $x:expr, $p:expr) => {
        $crate::util::smart::SmartF64 {
            value: $x,
            precision: $p,
            mode: $crate::util::smart::Mode::Trunc,
        }
    };
}

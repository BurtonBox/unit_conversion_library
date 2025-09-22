use crate::unit_conversion::{Quantity, UnitConversion};

pub enum LengthDim {}
pub type Length = Quantity<Meter>;

pub struct Meter;
impl UnitConversion for Meter {
    type Dimension = LengthDim;
    #[inline] fn convert_to_base(v: f64) -> f64 { v }          // base is meter
    #[inline] fn convert_from_base(v: f64) -> f64 { v }
    const SYMBOL: &'static str = "m";
}

pub struct Kilometer;
impl UnitConversion for Kilometer {
    type Dimension = LengthDim;
    #[inline] fn convert_to_base(v: f64) -> f64 { v * 1_000.0 }
    #[inline] fn convert_from_base(v: f64) -> f64 { v / 1_000.0 }
    const SYMBOL: &'static str = "km";
}

pub struct Foot;
impl UnitConversion for Foot {
    type Dimension = LengthDim;
    #[inline] fn convert_to_base(v: f64) -> f64 { v * 0.3048 }
    #[inline] fn convert_from_base(v: f64) -> f64 { v / 0.3048 }
    const SYMBOL: &'static str = "ft";
}

#[cfg(test)]
mod tests {
    use super::*;

    // account for floating-point math rounding errors
    fn approx(a: f64, b: f64, eps: f64) -> bool { (a - b).abs() <= eps }

    #[test]
    fn meter_to_kilometer_and_back() {
        let d: Length = Quantity::<Meter>::from_unit::<Meter>(1234.5);
        let km = d.to_unit::<Kilometer>();
        assert!(approx(km, 1.2345, 1e-12));

        let back: Length = Quantity::<Meter>::from_unit::<Kilometer>(km);
        assert!(approx(back.in_base(), d.in_base(), 1e-12));
    }

    #[test]
    fn meter_to_foot_and_back() {
        // 1 m = 3.280839895... ft
        let one_m: Length = Quantity::<Meter>::from_unit::<Meter>(1.0);
        let ft = one_m.to_unit::<Foot>();
        assert!(approx(ft, 3.280_839_895, 1e-9));

        let back: Length = Quantity::<Meter>::from_unit::<Foot>(ft);
        assert!(approx(back.in_base(), 1.0, 1e-12));
    }

    #[test]
    fn kilometer_exact_scaling() {
        let km: Length = Quantity::<Meter>::from_unit::<Kilometer>(2.0);
        assert!(approx(km.in_base(), 2000.0, 1e-12));
        assert!(approx(km.to_unit::<Kilometer>(), 2.0, 1e-12));
    }

    #[test]
    fn length_symbols_exist() {
        assert_eq!(Meter::SYMBOL, "m");
        assert_eq!(Kilometer::SYMBOL, "km");
        assert_eq!(Foot::SYMBOL, "ft");
    }
}
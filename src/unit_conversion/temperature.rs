use crate::unit_conversion::{Quantity, UnitConversion};

pub enum TemperatureDimension {}
pub type Temperature = Quantity<Kelvin>;

pub struct Kelvin;
impl UnitConversion for Kelvin {
    type Dimension = TemperatureDimension;
    fn convert_to_base(value: f64) -> f64 {
        value
    }
    fn convert_from_base(value: f64) -> f64 {
        value
    }
    const SYMBOL: &'static str = "K";
}

pub struct Celsius;
impl UnitConversion for Celsius {
    type Dimension = TemperatureDimension;
    #[inline]
    fn convert_to_base(v: f64) -> f64 {
        v + 273.15
    }
    #[inline]
    fn convert_from_base(v: f64) -> f64 {
        v - 273.15
    }
    const SYMBOL: &'static str = "°C";
}

pub struct Fahrenheit;
impl UnitConversion for Fahrenheit {
    type Dimension = TemperatureDimension;
    #[inline]
    fn convert_to_base(v: f64) -> f64 {
        (v - 32.0) * 5.0 / 9.0 + 273.15
    }
    #[inline]
    fn convert_from_base(v: f64) -> f64 {
        (v - 273.15) * 9.0 / 5.0 + 32.0
    }
    const SYMBOL: &'static str = "°F";
}

#[cfg(test)]
mod tests {
    use super::*;

    // account for floating-point math rounding errors
    fn approx(a: f64, b: f64, eps: f64) -> bool { (a - b).abs() <= eps }

    #[test]
    fn celsius_to_kelvin_and_back() {
        // 100 °C = 373.15 K
        let t: Temperature = Quantity::<Kelvin>::from_unit::<Celsius>(100.0);
        assert!(approx(t.in_base(), 373.15, 1e-12));
        let c = t.to_unit::<Celsius>();
        assert!(approx(c, 100.0, 1e-12));
    }

    #[test]
    fn fahrenheit_to_kelvin_and_back() {
        // 32 °F = 273.15 K
        let t: Temperature = Quantity::<Kelvin>::from_unit::<Fahrenheit>(32.0);
        assert!(approx(t.in_base(), 273.15, 1e-12));
        let f = t.to_unit::<Fahrenheit>();
        assert!(approx(f, 32.0, 1e-12));
    }

    #[test]
    fn absolute_zero_cross_checks() {
        // -273.15 °C = 0 K
        let kz: Temperature = Quantity::<Kelvin>::from_unit::<Celsius>(-273.15);
        assert!(approx(kz.in_base(), 0.0, 1e-12));

        // -459.67 °F ≈ 0 K
        let fz: Temperature = Quantity::<Kelvin>::from_unit::<Fahrenheit>(-459.67);
        assert!(approx(fz.in_base(), 0.0, 1e-8));
    }

    #[test]
    fn kelvin_passthrough() {
        let t = Quantity::<Kelvin>::from_unit::<Kelvin>(250.0);
        assert!(approx(t.in_base(), 250.0, 1e-12));
        assert!(approx(t.to_unit::<Kelvin>(), 250.0, 1e-12));
    }

    #[test]
    fn temperature_symbols_exist() {
        assert_eq!(Kelvin::SYMBOL, "K");
        assert_eq!(Celsius::SYMBOL, "°C");
        assert_eq!(Fahrenheit::SYMBOL, "°F");
    }
}

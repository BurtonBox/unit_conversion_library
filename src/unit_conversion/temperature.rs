//! # Temperature Unit Conversions
//!
//! This module provides type-safe temperature conversions between Celsius, Fahrenheit, and Kelvin.
//! All temperatures are internally stored in Kelvin (the base unit) and converted on demand.
//!
//! ## Supported Units
//!
//! - **Kelvin (K)**: The base unit, absolute temperature scale
//! - **Celsius (°C)**: Water freezes at 0°C, boils at 100°C at standard pressure
//! - **Fahrenheit (°F)**: Water freezes at 32°F, boils at 212°F at standard pressure
//!
//! ## Examples
//!
//! ```
//! use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit, Kelvin};
//!
//! // Convert between units
//! let freezing = Temperature::from_unit::<Celsius>(0.0);
//! assert_eq!(freezing.to_unit::<Fahrenheit>(), 32.0);
//! assert_eq!(freezing.to_unit::<Kelvin>(), 273.15);
//!
//! // All units can be converted to each other
//! let temp = Temperature::from_unit::<Fahrenheit>(68.0); // Room temperature
//! assert_eq!(temp.to_unit::<Celsius>(), 20.0);
//! ```

use crate::unit_conversion::{Quantity, UnitConversion};

/// Constants for temperature conversions
const CELSIUS_TO_KELVIN_OFFSET: f64 = 273.15;
const FAHRENHEIT_FREEZING_POINT: f64 = 32.0;
const FAHRENHEIT_DEGREE_RATIO: f64 = 9.0 / 5.0;
const CELSIUS_DEGREE_RATIO: f64 = 5.0 / 9.0;

/// Marker type for the temperature dimension.
///
/// This prevents accidental conversions between temperature and other dimensions
/// like length or mass.
pub enum TemperatureDimension {}

/// A temperature quantity that stores values in Kelvin internally.
///
/// This is the main type for working with temperatures. It provides type-safe
/// conversions between Celsius, Fahrenheit, and Kelvin.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit};
///
/// let boiling = Temperature::from_unit::<Celsius>(100.0);
/// let fahrenheit_value = boiling.to_unit::<Fahrenheit>(); // 212.0
/// ```
pub type Temperature = Quantity<Kelvin>;

/// Kelvin temperature unit (absolute temperature scale).
///
/// Kelvin is the base unit for temperature in this system. It's an absolute
/// temperature scale where 0 K represents absolute zero.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::temperature::{Temperature, Kelvin};
///
/// let absolute_zero = Temperature::from_unit::<Kelvin>(0.0);
/// assert_eq!(absolute_zero.to_unit::<Kelvin>(), 0.0);
/// ```
pub struct Kelvin;

impl UnitConversion for Kelvin {
    type Dimension = TemperatureDimension;

    #[inline]
    fn convert_to(value: f64) -> f64 {
        value // Kelvin is the base unit
    }

    #[inline]
    fn convert_from(value: f64) -> f64 {
        value // Kelvin is the base unit
    }

    const SYMBOL: &'static str = "K";
}

/// Celsius temperature unit.
///
/// The Celsius scale sets the freezing point of water at 0°C and the boiling
/// point at 100°C under standard atmospheric pressure.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::temperature::{Temperature, Celsius, Kelvin};
///
/// let freezing = Temperature::from_unit::<Celsius>(0.0);
/// assert_eq!(freezing.to_unit::<Kelvin>(), 273.15);
/// ```
pub struct Celsius;

impl UnitConversion for Celsius {
    type Dimension = TemperatureDimension;

    fn convert_to(value: f64) -> f64 {
        value + CELSIUS_TO_KELVIN_OFFSET
    }

    fn convert_from(value: f64) -> f64 {
        value - CELSIUS_TO_KELVIN_OFFSET
    }

    const SYMBOL: &'static str = "°C";
}

/// Fahrenheit temperature unit.
///
/// The Fahrenheit scale sets the freezing point of water at 32°F and the boiling
/// point at 212°F under standard atmospheric pressure.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::temperature::{Temperature, Fahrenheit, Celsius};
///
/// let room_temp = Temperature::from_unit::<Fahrenheit>(68.0);
/// assert_eq!(room_temp.to_unit::<Celsius>(), 20.0);
/// ```
pub struct Fahrenheit;

impl UnitConversion for Fahrenheit {
    type Dimension = TemperatureDimension;

    fn convert_to(value: f64) -> f64 {
        (value - FAHRENHEIT_FREEZING_POINT) * CELSIUS_DEGREE_RATIO + CELSIUS_TO_KELVIN_OFFSET
    }

    fn convert_from(value: f64) -> f64 {
        (value - CELSIUS_TO_KELVIN_OFFSET) * FAHRENHEIT_DEGREE_RATIO + FAHRENHEIT_FREEZING_POINT
    }

    const SYMBOL: &'static str = "°F";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn celsius_to_fahrenheit() {
        let temp = Temperature::from_unit::<Celsius>(0.0);
        assert!(approx(temp.to_unit::<Fahrenheit>(), 32.0, 1e-12));

        let temp = Temperature::from_unit::<Celsius>(100.0);
        assert!(approx(temp.to_unit::<Fahrenheit>(), 212.0, 1e-12));

        let temp = Temperature::from_unit::<Celsius>(-40.0);
        assert!(approx(temp.to_unit::<Fahrenheit>(), -40.0, 1e-12));
    }

    #[test]
    fn fahrenheit_to_celsius() {
        let temp = Temperature::from_unit::<Fahrenheit>(32.0);
        assert!(approx(temp.to_unit::<Celsius>(), 0.0, 1e-12));

        let temp = Temperature::from_unit::<Fahrenheit>(212.0);
        assert!(approx(temp.to_unit::<Celsius>(), 100.0, 1e-12));

        let temp = Temperature::from_unit::<Fahrenheit>(-40.0);
        assert!(approx(temp.to_unit::<Celsius>(), -40.0, 1e-12));
    }

    #[test]
    fn celsius_to_kelvin() {
        let temp = Temperature::from_unit::<Celsius>(0.0);
        assert!(approx(temp.to_unit::<Kelvin>(), 273.15, 1e-12));

        let temp = Temperature::from_unit::<Celsius>(100.0);
        assert!(approx(temp.to_unit::<Kelvin>(), 373.15, 1e-12));

        let temp = Temperature::from_unit::<Celsius>(-273.15);
        assert!(approx(temp.to_unit::<Kelvin>(), 0.0, 1e-12));
    }

    #[test]
    fn kelvin_to_celsius() {
        let temp = Temperature::from_unit::<Kelvin>(273.15);
        assert!(approx(temp.to_unit::<Celsius>(), 0.0, 1e-12));

        let temp = Temperature::from_unit::<Kelvin>(373.15);
        assert!(approx(temp.to_unit::<Celsius>(), 100.0, 1e-12));

        let temp = Temperature::from_unit::<Kelvin>(0.0);
        assert!(approx(temp.to_unit::<Celsius>(), -273.15, 1e-12));
    }

    #[test]
    fn fahrenheit_to_kelvin() {
        let temp = Temperature::from_unit::<Fahrenheit>(32.0);
        assert!(approx(temp.to_unit::<Kelvin>(), 273.15, 1e-12));

        let temp = Temperature::from_unit::<Fahrenheit>(212.0);
        assert!(approx(temp.to_unit::<Kelvin>(), 373.15, 1e-12));

        let temp = Temperature::from_unit::<Fahrenheit>(-459.67);
        assert!(approx(temp.to_unit::<Kelvin>(), 0.0, 1e-8));
    }

    #[test]
    fn kelvin_to_fahrenheit() {
        let temp = Temperature::from_unit::<Kelvin>(273.15);
        assert!(approx(temp.to_unit::<Fahrenheit>(), 32.0, 1e-12));

        let temp = Temperature::from_unit::<Kelvin>(373.15);
        assert!(approx(temp.to_unit::<Fahrenheit>(), 212.0, 1e-12));

        let temp = Temperature::from_unit::<Kelvin>(0.0);
        assert!(approx(temp.to_unit::<Fahrenheit>(), -459.67, 1e-8));
    }

    #[test]
    fn temperature_symbols() {
        assert_eq!(Kelvin::SYMBOL, "K");
        assert_eq!(Celsius::SYMBOL, "°C");
        assert_eq!(Fahrenheit::SYMBOL, "°F");
    }
}

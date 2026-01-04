//! # Length Unit Conversions
//!
//! This module provides type-safe length conversions between meters, kilometers, and feet.
//! All lengths are internally stored in meters (the base unit) and converted on demand.
//!
//! ## Supported Units
//!
//! - **Meter (m)**: The base unit, SI unit of length
//! - **Kilometer (km)**: 1000 meters
//! - **Foot (ft)**: Imperial unit, exactly 0.3048 meters
//!
//! ## Examples
//!
//! ```
//! use uom::unit_conversion::length::{Length, Meter, Kilometer, Foot};
//!
//! // Convert between units
//! let distance = Length::from_unit::<Kilometer>(1.0);
//! assert_eq!(distance.to_unit::<Meter>(), 1000.0);
//! assert!((distance.to_unit::<Foot>() - 3280.839895).abs() < 1e-6);
//!
//! // Exact conversions
//! let one_foot = Length::from_unit::<Foot>(1.0);
//! assert_eq!(one_foot.to_unit::<Meter>(), 0.3048);
//! ```
//!
//!
/* TODO: SUPPORT ALL THESE.
// Gigaparsec (Gpc) – One billion parsecs, used in cosmology for intergalactic distances.
// Megaparsec (Mpc) – One million parsecs, commonly used in extragalactic astronomy.
// Kiloparsec (kpc) – One thousand parsecs.
// Parsec (pc) – Approximately 3.26 light-years or 3.08 × 10¹⁶ meters.
// Light-year (ly) – Distance light travels in one year (~9.46 × 10¹⁵ meters).
// Astronomical Unit (AU) – Average Earth-Sun distance (~1.496 × 10¹¹ meters).
//
// Kilometer (km) – 1,000 meters.
// Meter (m) – SI base unit of length.
// Decimeter (dm) – 0.1 meters.
// Centimeter (cm) – 0.01 meters.
// Millimeter (mm) – 0.001 meters.
// Micrometer (µm) – 10⁻⁶ meters.
// Nanometer (nm) – 10⁻⁹ meters.
// Picometer (pm) – 10⁻¹² meters.
// Femtometer (fm) – Also called a fermi, 10⁻¹⁵ meters, used for nuclear scales
// Planck length (ℓₚ) – ~1.616 × 10⁻³⁵ meters
// */

use crate::unit_conversion::{Quantity, UnitConversion};

/// Constants for length conversions
const METERS_PER_KILOMETER: f64 = 1000.0;
const METERS_PER_FOOT: f64 = 0.3048; // Exact definition

/// Marker type for the length dimension.
///
/// This prevents accidental conversions between length and other dimensions
/// like temperature or mass.
pub enum LengthDimension {}

/// A length quantity that stores values in meters internally.
///
/// This is the main type for working with lengths. It provides type-safe
/// conversions between meters, kilometers, and feet.
///
/// # Examples
///
/// ```
/// use uom::unit_conversion::length::{Length, Meter, Kilometer};
///
/// let distance = Length::from_unit::<Kilometer>(5.0);
/// let meter_value = distance.to_unit::<Meter>(); // 5000.0
/// ```
pub type Length = Quantity<Meter>;

/// Meter length unit (SI base unit).
///
/// The meter is the base unit for length in this system and in the International
/// System of Units (SI).
///
/// # Examples
///
/// ```
/// use uom::unit_conversion::length::{Length, Meter};
///
/// let length = Length::from_unit::<Meter>(100.0);
/// assert_eq!(length.to_unit::<Meter>(), 100.0);
/// ```
pub struct Meter;

impl UnitConversion for Meter {
    type Dimension = LengthDimension;

    #[inline]
    fn convert_to(value: f64) -> f64 {
        value // Meter is the base unit
    }

    #[inline]
    fn convert_from(value: f64) -> f64 {
        value // Meter is the base unit
    }

    const SYMBOL: &'static str = "m";
}

/// Kilometer length unit.
///
/// A kilometer is exactly 1000 meters. Commonly used for measuring longer
/// distances like travel distances or geographical measurements.
///
/// # Examples
///
/// ```
/// use uom::unit_conversion::length::{Length, Kilometer, Meter};
///
/// let distance = Length::from_unit::<Kilometer>(2.5);
/// assert_eq!(distance.to_unit::<Meter>(), 2500.0);
/// ```
pub struct Kilometer;

impl UnitConversion for Kilometer {
    type Dimension = LengthDimension;

    #[inline]
    fn convert_to(value: f64) -> f64 {
        value * METERS_PER_KILOMETER
    }

    #[inline]
    fn convert_from(value: f64) -> f64 {
        value / METERS_PER_KILOMETER
    }

    const SYMBOL: &'static str = "km";
}

/// Foot length unit (Imperial).
///
/// The international foot is defined as exactly 0.3048 meters. This is the
/// standard definition used in most countries.
///
/// # Examples
///
/// ```
/// use uom::unit_conversion::length::{Length, Foot, Meter};
///
/// let height = Length::from_unit::<Foot>(6.0);
/// assert!((height.to_unit::<Meter>() - 1.8288).abs() < 1e-10);
/// ```
pub struct Foot;

impl UnitConversion for Foot {
    type Dimension = LengthDimension;

    #[inline]
    fn convert_to(value: f64) -> f64 {
        value * METERS_PER_FOOT
    }

    #[inline]
    fn convert_from(value: f64) -> f64 {
        value / METERS_PER_FOOT
    }

    const SYMBOL: &'static str = "ft";
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() <= eps
    }

    #[test]
    fn meter_to_kilometer() {
        let length = Length::from_unit::<Meter>(1000.0);
        assert!(approx(length.to_unit::<Kilometer>(), 1.0, 1e-12));

        let length = Length::from_unit::<Meter>(2500.0);
        assert!(approx(length.to_unit::<Kilometer>(), 2.5, 1e-12));

        let length = Length::from_unit::<Meter>(0.5);
        assert!(approx(length.to_unit::<Kilometer>(), 0.0005, 1e-12));
    }

    #[test]
    fn kilometer_to_meter() {
        let length = Length::from_unit::<Kilometer>(1.0);
        assert!(approx(length.to_unit::<Meter>(), 1000.0, 1e-12));

        let length = Length::from_unit::<Kilometer>(2.5);
        assert!(approx(length.to_unit::<Meter>(), 2500.0, 1e-12));

        let length = Length::from_unit::<Kilometer>(0.001);
        assert!(approx(length.to_unit::<Meter>(), 1.0, 1e-12));
    }

    #[test]
    fn meter_to_foot() {
        let length = Length::from_unit::<Meter>(1.0);
        assert!(approx(length.to_unit::<Foot>(), 3.280839895, 1e-9));

        let length = Length::from_unit::<Meter>(0.3048);
        assert!(approx(length.to_unit::<Foot>(), 1.0, 1e-12));

        let length = Length::from_unit::<Meter>(10.0);
        assert!(approx(length.to_unit::<Foot>(), 32.80839895, 1e-8));
    }

    #[test]
    fn foot_to_meter() {
        let length = Length::from_unit::<Foot>(1.0);
        assert!(approx(length.to_unit::<Meter>(), 0.3048, 1e-12));

        let length = Length::from_unit::<Foot>(3.280839895);
        assert!(approx(length.to_unit::<Meter>(), 1.0, 1e-9));

        let length = Length::from_unit::<Foot>(10.0);
        assert!(approx(length.to_unit::<Meter>(), 3.048, 1e-12));
    }

    #[test]
    fn kilometer_to_foot() {
        let length = Length::from_unit::<Kilometer>(1.0);
        assert!(approx(length.to_unit::<Foot>(), 3280.839895, 1e-6));

        let length = Length::from_unit::<Kilometer>(0.0003048);
        assert!(approx(length.to_unit::<Foot>(), 1.0, 1e-9));

        let length = Length::from_unit::<Kilometer>(2.0);
        assert!(approx(length.to_unit::<Foot>(), 6561.67979, 1e-5));
    }

    #[test]
    fn foot_to_kilometer() {
        let length = Length::from_unit::<Foot>(3280.839895);
        assert!(approx(length.to_unit::<Kilometer>(), 1.0, 1e-9));

        let length = Length::from_unit::<Foot>(1.0);
        assert!(approx(length.to_unit::<Kilometer>(), 0.0003048, 1e-12));

        let length = Length::from_unit::<Foot>(6561.67979);
        assert!(approx(length.to_unit::<Kilometer>(), 2.0, 1e-8));
    }

    #[test]
    fn length_symbols() {
        assert_eq!(Meter::SYMBOL, "m");
        assert_eq!(Kilometer::SYMBOL, "km");
        assert_eq!(Foot::SYMBOL, "ft");
    }
}

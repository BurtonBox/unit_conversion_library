//! # Unit Conversion Library
//!
//! A type-safe unit conversion system that prevents unit mixing errors at compile time.
//! This library leverages Rust's type system to ensure that incompatible units cannot be
//! accidentally mixed (e.g., adding temperature to length).
//!
//! ## Core Design
//!
//! The library is built around two main concepts:
//! - **Dimensions**: Separate types for different kinds of measurements (temperature, length, etc.)
//! - **Units**: Specific units within a dimension (Celsius, Fahrenheit for temperature)
//!
//! All values are internally stored in base units (Kelvin for temperature, meters for length)
//! and converted on-demand to the requested unit type.
//!
//! ## Type Safety
//!
//! The type system prevents compilation of invalid operations:
//! ```compile_fail
//! # use utilities::unit_conversion::temperature::{Temperature, Celsius};
//! # use utilities::unit_conversion::length::{Length, Meter};
//! let temp = Temperature::from_unit::<Celsius>(25.0);
//! let dist = Length::from_unit::<Meter>(100.0);
//! // This won't compile - can't convert temperature to length:
//! let invalid = temp.to_unit::<Meter>();
//! ```
//!
//! ## Example Usage
//!
//! ```
//! use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit};
//! use utilities::unit_conversion::length::{Length, Meter, Kilometer};
//!
//! // Temperature conversions
//! let temp = Temperature::from_unit::<Celsius>(100.0);
//! let fahrenheit_value = temp.to_unit::<Fahrenheit>(); // 212.0
//!
//! // Length conversions  
//! let distance = Length::from_unit::<Kilometer>(5.0);
//! let meter_value = distance.to_unit::<Meter>(); // 5000.0
//! ```

pub mod length;
pub mod temperature;

use std::marker::PhantomData;

/// Defines how a unit type converts to and from base units within its dimension.
///
/// This trait must be implemented for each unit type (e.g., Celsius, Fahrenheit).
/// The `Dimension` associated type ensures that units can only be converted
/// within the same measurement dimension.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::{UnitConversion, Quantity};
/// use std::marker::PhantomData;
///
/// // Define a custom dimension
/// pub enum TimeDimension {}
///
/// // Define a unit within that dimension
/// pub struct Second;
/// impl UnitConversion for Second {
///     type Dimension = TimeDimension;
///     fn convert_to(value: f64) -> f64 { value }      // Second is base unit
///     fn convert_from(value: f64) -> f64 { value }    // Second is base unit  
///     const SYMBOL: &'static str = "s";
/// }
///
/// pub struct Minute;
/// impl UnitConversion for Minute {
///     type Dimension = TimeDimension;
///     fn convert_to(value: f64) -> f64 { value * 60.0 }   // Convert to seconds
///     fn convert_from(value: f64) -> f64 { value / 60.0 } // Convert from seconds
///     const SYMBOL: &'static str = "min";
/// }
/// ```
pub trait UnitConversion {
    /// The dimension this unit belongs to (e.g., TemperatureDimension, LengthDimension).
    /// This prevents cross-dimension conversions at compile time.
    type Dimension;

    /// Convert a value from this unit to the base unit of the dimension.
    ///
    /// For example, Celsius::convert_to(0.0) should return 273.15 (Kelvin).
    fn convert_to(value: f64) -> f64;

    /// Convert a value from the base unit to this unit.
    ///
    /// For example, Celsius::convert_from(273.15) should return 0.0.
    fn convert_from(value: f64) -> f64;

    /// The symbol used to display this unit (e.g., "°C", "m", "ft").
    const SYMBOL: &'static str;
}

/// A quantity with a specific unit type, storing values in base units internally.
///
/// This is the core type for type-safe unit conversions. It stores all values
/// internally in the base unit of the dimension and converts on-demand to
/// requested unit types.
///
/// # Type Parameters
///
/// * `U` - The unit type implementing `UnitConversion`. This determines both
///   the dimension and the "natural" unit for this quantity.
///
/// # Examples
///
/// ```
/// use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit};
///
/// // Create a temperature from Celsius
/// let temp = Temperature::from_unit::<Celsius>(100.0);
///
/// // Convert to different units
/// let f = temp.to_unit::<Fahrenheit>(); // 212.0
/// let c = temp.to_unit::<Celsius>();    // 100.0
/// ```
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Quantity<U: UnitConversion> {
    /// The value stored in base units (e.g., Kelvin for temperature, meters for length)
    base: f64,
    /// Phantom data to track the unit type at compile time
    _u: PhantomData<U>,
}

impl<U: UnitConversion> Quantity<U> {
    /// Creates a new quantity from a value in the specified unit type.
    ///
    /// The value is immediately converted to base units for internal storage,
    /// ensuring all quantities of the same dimension use consistent internal
    /// representation.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The unit type to convert from. Must be in the same dimension as `U`.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value in units of type `V`
    ///
    /// # Examples
    ///
    /// ```
    /// use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit};
    ///
    /// // Create temperature from Celsius
    /// let temp1 = Temperature::from_unit::<Celsius>(0.0);
    ///
    /// // Create temperature from Fahrenheit  
    /// let temp2 = Temperature::from_unit::<Fahrenheit>(32.0);
    ///
    /// // Both represent the same temperature (freezing point of water)
    /// assert!((temp1.to_unit::<Celsius>() - temp2.to_unit::<Celsius>()).abs() < 1e-10);
    /// ```
    pub fn from_unit<V>(value: f64) -> Self
    where
        V: UnitConversion<Dimension = U::Dimension>,
    {
        Self {
            base: V::convert_to(value),
            _u: PhantomData,
        }
    }

    /// Converts this quantity to the specified unit type.
    ///
    /// The internal base unit value is converted to the requested unit type
    /// using the target unit's conversion functions.
    ///
    /// # Type Parameters
    ///
    /// * `V` - The unit type to convert to. Must be in the same dimension as `U`.
    ///
    /// # Returns
    ///
    /// The numeric value in units of type `V`.
    ///
    /// # Examples
    ///
    /// ```
    /// use utilities::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit, Kelvin};
    ///
    /// let temp = Temperature::from_unit::<Celsius>(100.0);
    ///
    /// assert_eq!(temp.to_unit::<Celsius>(), 100.0);
    /// assert_eq!(temp.to_unit::<Fahrenheit>(), 212.0);
    /// assert_eq!(temp.to_unit::<Kelvin>(), 373.15);
    /// ```
    pub fn to_unit<V>(&self) -> f64
    where
        V: UnitConversion<Dimension = U::Dimension>,
    {
        V::convert_from(self.base)
    }

    /// Returns the raw value in base units.
    ///
    /// This is primarily useful for debugging or when you need to access
    /// the internal representation. In most cases, you should use `to_unit()`
    /// to convert to a specific unit type.
    ///
    /// # Examples
    ///
    /// ```
    /// use utilities::unit_conversion::temperature::{Temperature, Celsius};
    ///
    /// let temp = Temperature::from_unit::<Celsius>(0.0);
    /// assert_eq!(temp.in_base(), 273.15); // 0°C = 273.15K (base unit)
    /// ```
    #[allow(dead_code)]
    pub fn in_base(&self) -> f64 {
        self.base
    }
}

# Unit Conversion Library

A type-safe unit conversion system built in Rust that prevents unit mixing errors at compile time.

## Overview

This library demonstrates advanced Rust type system usage to create a zero-cost abstraction for unit conversions. The system prevents common programming errors like accidentally mixing incompatible units (e.g., adding temperature to length) while providing efficient runtime performance.

## Features

- **Type Safety**: Compile-time prevention of unit mixing errors
- **Zero Cost**: No runtime overhead for type safety
- **Extensible**: Easy to add new units and dimensions
- **Well Tested**: Comprehensive test suite with floating-point precision handling
- **Well Documented**: Full rustdoc documentation with examples

## Supported Units

### Temperature
- **Kelvin (K)**: Absolute temperature scale, base unit
- **Celsius (°C)**: Water freezes at 0°C, boils at 100°C
- **Fahrenheit (°F)**: Water freezes at 32°F, boils at 212°F

### Length
- **Meter (m)**: SI base unit for length
- **Kilometer (km)**: 1000 meters
- **Foot (ft)**: Imperial unit, exactly 0.3048 meters

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
playground = { path = "." }
```

### Basic Usage

```rust
use playground::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit, Kelvin};
use playground::unit_conversion::length::{Length, Meter, Kilometer, Foot};

fn main() {
    // Temperature conversions
    let temp = Temperature::from_unit::<Celsius>(100.0);
    println!("Boiling point: {}°C = {}°F", 
        temp.to_unit::<Celsius>(), 
        temp.to_unit::<Fahrenheit>()
    ); // Output: Boiling point: 100°C = 212°F
    
    // Length conversions
    let distance = Length::from_unit::<Kilometer>(5.0);
    println!("Distance: {}km = {}m", 
        distance.to_unit::<Kilometer>(), 
        distance.to_unit::<Meter>()
    ); // Output: Distance: 5km = 5000m
}
```

### Type Safety in Action

This code **won't compile** - the type system prevents unit mixing:

```rust
// ❌ Compile error: Cannot convert temperature to length
let temp = Temperature::from_unit::<Celsius>(25.0);
let length = Length::from_unit::<Meter>(100.0);
let invalid = temp.to_unit::<Meter>(); // ERROR!
```

## Architecture

The library uses three key concepts:

1. **Dimensions**: Marker types that group related units (`TemperatureDimension`, `LengthDimension`)
2. **Units**: Types implementing `UnitConversion` trait (`Celsius`, `Meter`, etc.)
3. **Quantities**: Generic wrapper `Quantity<U>` that stores values and enforces type safety

### Internal Storage

All values are stored internally in base units:
- Temperature: Kelvin
- Length: Meters

This ensures consistent precision and simplifies conversions.

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test temperature  # Temperature-related tests
cargo test length      # Length-related tests

# Run documentation tests
cargo test --doc

# Run a single test
cargo test celsius_to_fahrenheit
```

### Building Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate docs with dependencies
cargo doc --no-deps --open
```

### Code Formatting

```bash
# Format code (if rustfmt is configured)
cargo fmt

# Lint code (if clippy is configured)
cargo clippy
```

## Examples

### Temperature Conversions

```rust
use playground::unit_conversion::temperature::{Temperature, Celsius, Fahrenheit, Kelvin};

// Common temperature conversions
let freezing = Temperature::from_unit::<Celsius>(0.0);
assert_eq!(freezing.to_unit::<Fahrenheit>(), 32.0);
assert_eq!(freezing.to_unit::<Kelvin>(), 273.15);

let body_temp = Temperature::from_unit::<Fahrenheit>(98.6);
assert!((body_temp.to_unit::<Celsius>() - 37.0).abs() < 0.1);
```

### Length Conversions

```rust
use playground::unit_conversion::length::{Length, Meter, Kilometer, Foot};

// Distance calculations
let marathon = Length::from_unit::<Kilometer>(42.195);
let meters = marathon.to_unit::<Meter>(); // 42195.0
let feet = marathon.to_unit::<Foot>();   // ~138435.7

// Height conversions
let height = Length::from_unit::<Foot>(6.0);
assert!((height.to_unit::<Meter>() - 1.8288).abs() < 1e-10);
```

## Design Philosophy

This library demonstrates several Rust programming principles:

- **Zero-cost abstractions**: Type safety with no runtime overhead
- **Compile-time guarantees**: Prevent entire classes of bugs at compile time
- **Ergonomic APIs**: Easy to use while maintaining safety
- **Extensibility**: Simple to add new units and dimensions
- **Documentation**: Comprehensive docs with tested examples

## Adding New Units

To add a new unit, implement the `UnitConversion` trait:

```rust
use playground::unit_conversion::{UnitConversion, Quantity};

// Define your dimension (if new)
pub enum MassDimension {}

// Define your unit
pub struct Kilogram;
impl UnitConversion for Kilogram {
    type Dimension = MassDimension;
    fn convert_to(value: f64) -> f64 { value }      // Base unit
    fn convert_from(value: f64) -> f64 { value }    // Base unit
    const SYMBOL: &'static str = "kg";
}

// Create a type alias for convenience
pub type Mass = Quantity<Kilogram>;
```

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Documentation builds: `cargo doc`
3. Code is formatted: `cargo fmt`
4. Add tests for new functionality
5. Update documentation for public APIs

## License

This project is a playground/educational example. See the repository for license details.
mod unit_conversion;

use unit_conversion::UnitConversion;
use unit_conversion::length::{Foot, Kilometer, Length, Meter};
use unit_conversion::temperature::{Celsius, Fahrenheit, Kelvin, Temperature};

fn main() {
    let t_f = Temperature::from_unit::<Fahrenheit>(85.6);
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        t_f.to_unit::<Fahrenheit>(),
        Fahrenheit::SYMBOL,
        t_f.to_unit::<Celsius>(),
        Celsius::SYMBOL
    );
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        t_f.to_unit::<Fahrenheit>(),
        Fahrenheit::SYMBOL,
        t_f.to_unit::<Kelvin>(),
        Kelvin::SYMBOL
    );

    let t_c = Temperature::from_unit::<Celsius>(41.0);
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        t_c.to_unit::<Celsius>(),
        Celsius::SYMBOL,
        t_c.to_unit::<Fahrenheit>(),
        Fahrenheit::SYMBOL
    );

    let d_km = Length::from_unit::<Kilometer>(3.2);
    println!(
        "Length = {:.2} {} is {:.3} {}",
        d_km.to_unit::<Kilometer>(),
        Kilometer::SYMBOL,
        d_km.to_unit::<Meter>(),
        Meter::SYMBOL
    );
    println!(
        "Length = {:.2} {} is {:.3} {}",
        d_km.to_unit::<Kilometer>(),
        Kilometer::SYMBOL,
        d_km.to_unit::<Foot>(),
        Foot::SYMBOL
    );
}
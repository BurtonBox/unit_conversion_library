mod unit_conversion;

use unit_conversion::UnitConversion;
use unit_conversion::length::{Foot, Kilometer, Length, Meter};
use unit_conversion::temperature::{Celsius, Fahrenheit, Kelvin, Temperature};
use uom::smart;

fn main() {
    let t_f = Temperature::from_unit::<Fahrenheit>(85.6);
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        smart!(t_f.to_unit::<Fahrenheit>(), 2),
        Fahrenheit::SYMBOL,
        smart!(t_f.to_unit::<Celsius>(), 2),
        Celsius::SYMBOL
    );
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        smart!(t_f.to_unit::<Fahrenheit>(), 2),
        Fahrenheit::SYMBOL,
        smart!(t_f.to_unit::<Kelvin>(), 2),
        Kelvin::SYMBOL
    );

    let t_c = Temperature::from_unit::<Celsius>(41.0);
    println!(
        "Temp = {:.2} {} is {:.2} {}",
        smart!(t_c.to_unit::<Celsius>(), 2),
        Celsius::SYMBOL,
        smart!(t_c.to_unit::<Fahrenheit>(), 2),
        Fahrenheit::SYMBOL
    );

    let d_km = Length::from_unit::<Kilometer>(3.2);
    println!(
        "Length = {:.2} {} is {:.3} {}",
        smart!(d_km.to_unit::<Kilometer>(), 5),
        Kilometer::SYMBOL,
        smart!(d_km.to_unit::<Meter>(), 5),
        Meter::SYMBOL
    );
    println!(
        "Length = {:.2} {} is {:.3} {}",
        smart!(d_km.to_unit::<Kilometer>(), 5),
        Kilometer::SYMBOL,
        smart!(d_km.to_unit::<Foot>(), 5),
        Foot::SYMBOL
    );
}
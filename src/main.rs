use utilities::unit_conversion::length::{Foot, Kilometer, Length, Meter};
use utilities::unit_conversion::temperature::{Celsius, Fahrenheit, Kelvin, Temperature};
use utilities::unit_conversion::UnitConversion;

fn main() {
    let t_f = Temperature::from_unit::<Fahrenheit>(85.6);
    println!("T = {:.2} {}", t_f.to_unit::<Celsius>(), Celsius::SYMBOL);
    println!("T = {:.2} {}", t_f.to_unit::<Fahrenheit>(), Fahrenheit::SYMBOL);
    println!("T = {:.2} {}", t_f.to_unit::<Kelvin>(), Kelvin::SYMBOL);

    let t_c = Temperature::from_unit::<Celsius>(41.0);
    println!("T = {:.2} {}", t_c.to_unit::<Fahrenheit>(), Fahrenheit::SYMBOL);

    let d_km = Length::from_unit::<Kilometer>(3.2);
    println!("D = {:.3} {}", d_km.to_unit::<Meter>(), Meter::SYMBOL);
    println!("D = {:.3} {}", d_km.to_unit::<Foot>(), Foot::SYMBOL);

}
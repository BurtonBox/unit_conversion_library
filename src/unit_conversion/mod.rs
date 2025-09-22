pub mod temperature;
pub mod length;

use std::marker::PhantomData;

pub trait UnitConversion {
    type Dimension;
    fn convert_to_base(value: f64) -> f64;
    fn convert_from_base(value: f64) -> f64;
    const SYMBOL: &'static str;
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Quantity<U: UnitConversion> {
    base: f64,
    _u: PhantomData<U>,
}

impl<U: UnitConversion> Quantity<U> {
    pub fn from_unit<V>(value: f64) -> Self
    where
        V: UnitConversion<Dimension = U::Dimension>,
    {
        Self {
            base: V::convert_to_base(value),
            _u: PhantomData,
        }
    }

    pub fn to_unit<V>(&self) -> f64
    where
        V: UnitConversion<Dimension = U::Dimension>,
    {
        V::convert_from_base(self.base)
    }

    pub fn in_base(&self) -> f64 {
        self.base
    }
}

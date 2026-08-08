use crate::math::NaturalNumbers::NaturalNumber;
use std::ops::Add;

impl Add for NaturalNumber {
    type Output = NaturalNumber;

    fn add(mut self, other: NaturalNumber) -> NaturalNumber {
        self += other;
        self
    }
}

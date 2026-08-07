use crate::math::NaturalNumbers::NaturalNumber;

macro_rules! NaturalNumberFromPremitiveUinsigend {
    ($($t:ty),* $(,)?) => {
        $(impl From<$t> for NaturalNumber {
            fn from(value: $t) -> Self {
                NaturalNumber::new(vec![value as u128])
            }
        })*
    };
}
NaturalNumberFromPremitiveUinsigend!(u8, u16, u32, u64, u128);

macro_rules! NaturalNumberFromPremitiveSigned {
    ($($t:ty),* $(,)?) => {
        $(impl TryFrom<$t> for NaturalNumber {
            type Error = String;
            fn try_from(value: $t) -> Result<Self, String> {
                let sign = value > 0;
                match sign {
                    true => Ok(NaturalNumber::new(vec![value as u128])),
                    false => Err(String::from("Negative number are outside the domain of Natural numbers")),
                }
            }
        })*
    };
}
NaturalNumberFromPremitiveSigned!(i8, i16, i32, i64, i128);

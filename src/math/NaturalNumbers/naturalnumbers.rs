#[derive(Clone, Debug, Eq)]
pub struct NaturalNumber {
    pub limbs: Vec<u128>,
}

impl NaturalNumber {
    pub fn new(limbs: Vec<u128>) -> Self {
        NaturalNumber { limbs }
    }
}

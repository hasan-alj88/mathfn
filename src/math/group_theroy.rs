trait Group<Number> {
    fn operation(a: Number, b: Number) -> Number;
    fn identity() -> Number;
    fn inverse(a: Number) -> Number;
    fn absorb_number() -> Number;
}

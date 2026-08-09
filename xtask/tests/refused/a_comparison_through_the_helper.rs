//! The neighbour of `an_equality_assertion_over_floats.rs`. The same number is
//! checked against the same computation, through the comparison the suite
//! shares, with the distance carrying the reason it is that distance.

pub fn stopping_power(_energy: f64) -> f64 {
    2.35
}

#[cfg(test)]
mod tests {
    use super::stopping_power;
    use bremsweg_core::assert_relatively_close;
    use bremsweg_core::close::Tolerance;

    #[test]
    fn the_stopping_power_at_one_mega_electron_volt() {
        assert_relatively_close!(
            stopping_power(1.0e6),
            2.35,
            Tolerance::of(
                2e-3,
                "the measurements this value is read off agree with each other to about \
                 two parts in a thousand, and a tighter distance would be a claim about \
                 the fit that the data does not support"
            )
        );
    }
}

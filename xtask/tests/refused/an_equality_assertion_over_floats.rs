//! An equality assertion over a floating point number, which is the spelling
//! `clippy::float_cmp` cannot see inside.
//!
//! Not a member of any target. The fixture beside this directory hands the text
//! of this file to the check and expects one refusal, naming this line.

pub fn stopping_power(_energy: f64) -> f64 {
    2.35
}

#[cfg(test)]
mod tests {
    use super::stopping_power;

    #[test]
    fn the_stopping_power_at_one_mega_electron_volt() {
        assert_eq!(stopping_power(1.0e6), 2.35);
    }
}

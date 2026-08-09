//! Comparing two numbers that came out of floating point arithmetic.
//!
//! A test asserting that two such numbers are equal either fails on a different
//! compiler or passes because both sides are wrong in the same way. A test
//! asserting that they are close needs a distance, and a distance chosen by
//! trying numbers until the test passes proves nothing at all.
//!
//! So the distance is a value carrying the reason it is that size, and the
//! reason is a parameter rather than a comment. A comment beside a number can
//! be deleted while the number stays; a field cannot be, and the compiler
//! refuses a call that leaves it out.
//!
//! Three comparisons, because three are what this project needs. Relative
//! closeness for a quantity that spans orders of magnitude, which stopping
//! powers do across the energy range. Closeness with an absolute floor for a
//! quantity that legitimately reaches zero, which several tallies do. And
//! exactness, for the places that mean it: repeating a run with the same seed,
//! and running one across a different number of threads.
//!
//! The exact one is not softened by the other two and is not a fallback for
//! them. It compares the bits, so it separates a positive zero from a negative
//! one and calls two identical quiet NaNs equal, which is what a determinism
//! test is asking about and is not what `==` answers.

use core::fmt;

/// How far apart two numbers may be, and why that is the distance.
#[derive(Clone, Copy, Debug)]
pub struct Tolerance {
    size: f64,
    because: &'static str,
}

impl Tolerance {
    /// A tolerance of `size`, which is that size `because`.
    ///
    /// The reason belongs at the call, not in a table somewhere: it is about
    /// this comparison and no other. Measurement uncertainty, expected
    /// numerical error, or a documented disagreement with a reference value are
    /// the three that occur; anything else is worth arguing about in review.
    ///
    /// # Panics
    ///
    /// If the size is negative or not finite, and if the reason is blank. A
    /// blank reason is the failure this type exists against, so it is refused
    /// rather than accepted quietly.
    #[must_use]
    pub fn of(size: f64, because: &'static str) -> Self {
        assert!(
            size.is_finite() && size >= 0.0,
            "a tolerance of {size} is not a distance"
        );
        assert!(
            !because.trim().is_empty(),
            "a tolerance of {size} with no reason beside it, which is the number \
             nobody can check and nobody can retire"
        );
        Self { size, because }
    }

    /// The distance itself.
    #[must_use]
    pub fn size(&self) -> f64 {
        self.size
    }

    /// Why it is that distance.
    #[must_use]
    pub fn because(&self) -> &'static str {
        self.because
    }
}

impl fmt::Display for Tolerance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.size, self.because)
    }
}

/// Whether `computed` and `expected` differ by no more than `tolerance` of the
/// larger of the two.
///
/// # Panics
///
/// If either number is not finite. `0005` fixes a non-finite value as a stop
/// rather than a condition, and a comparison that answered `false` for a NaN
/// would report it as a disagreement about physics.
#[must_use]
pub fn relatively_close(computed: f64, expected: f64, tolerance: Tolerance) -> bool {
    refuse_the_non_finite(computed, expected);
    let scale = computed.abs().max(expected.abs());
    (computed - expected).abs() <= tolerance.size() * scale
}

/// Whether `computed` and `expected` are within `floor` of each other in
/// absolute terms, or within `tolerance` of each other relatively.
///
/// The floor is what makes this usable on a quantity that legitimately reaches
/// zero: at zero there is no scale for a relative distance to be relative to,
/// and every relative comparison there is a comparison against nothing.
///
/// # Panics
///
/// If either number is not finite, for the reason above.
#[must_use]
pub fn close_with_floor(
    computed: f64,
    expected: f64,
    tolerance: Tolerance,
    floor: Tolerance,
) -> bool {
    refuse_the_non_finite(computed, expected);
    (computed - expected).abs() <= floor.size() || relatively_close(computed, expected, tolerance)
}

/// Whether `computed` and `expected` are the same number, bit for bit.
///
/// For the places that mean exactness rather than settle for it. It takes no
/// tolerance because there is nothing to justify, and it is written in terms of
/// the bits so that a NaN equals itself and a negative zero does not equal a
/// positive one. Both of those are differences a determinism test wants to see
/// and `==` reports as agreement or as disagreement respectively.
#[must_use]
pub fn exactly_equal(computed: f64, expected: f64) -> bool {
    computed.to_bits() == expected.to_bits()
}

/// The two numbers and the distance between them, for a failure message.
#[must_use]
pub fn describe(computed: f64, expected: f64) -> String {
    let difference = computed - expected;
    let scale = computed.abs().max(expected.abs());
    let relative = if scale > 0.0 {
        format!("{:e}", (difference / scale).abs())
    } else {
        "none, both are zero".to_string()
    };
    format!(
        "computed {computed:e}, expected {expected:e}, apart by {:e} \
         (relative {relative})",
        difference.abs()
    )
}

fn refuse_the_non_finite(computed: f64, expected: f64) {
    assert!(
        computed.is_finite() && expected.is_finite(),
        "a comparison of {computed} with {expected}, one of which is not a number \
         this code may produce"
    );
}

/// Asserts that two numbers are relatively close, and says how far apart they
/// were and what the distance was for when they are not.
#[macro_export]
macro_rules! assert_relatively_close {
    ($computed:expr, $expected:expr, $tolerance:expr $(,)?) => {{
        let (computed, expected, tolerance) = ($computed, $expected, $tolerance);
        assert!(
            $crate::close::relatively_close(computed, expected, tolerance),
            "{}, and the tolerance is {tolerance}",
            $crate::close::describe(computed, expected)
        );
    }};
}

/// Asserts that two numbers are close, with an absolute floor for the region
/// around zero where a relative distance means nothing.
#[macro_export]
macro_rules! assert_close_with_floor {
    ($computed:expr, $expected:expr, $tolerance:expr, $floor:expr $(,)?) => {{
        let (computed, expected, tolerance, floor) = ($computed, $expected, $tolerance, $floor);
        assert!(
            $crate::close::close_with_floor(computed, expected, tolerance, floor),
            "{}, and the tolerance is {tolerance} above a floor of {floor}",
            $crate::close::describe(computed, expected)
        );
    }};
}

/// Asserts that two numbers are the same bit for bit. For the places that mean
/// it; the two above do not soften this one.
#[macro_export]
macro_rules! assert_exactly_equal {
    ($computed:expr, $expected:expr $(,)?) => {{
        let (computed, expected) = ($computed, $expected);
        assert!(
            $crate::close::exactly_equal(computed, expected),
            "{}, and this comparison is exact on purpose",
            $crate::close::describe(computed, expected)
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::{Tolerance, close_with_floor, exactly_equal, relatively_close};

    /// One part in a million, which is far coarser than anything this project
    /// will use and is deliberate: these tests are about the comparison and not
    /// about a number's accuracy.
    fn coarse() -> Tolerance {
        Tolerance::of(
            1e-6,
            "a distance chosen to be far wider than any real disagreement, so that \
             what these tests fail on is the comparison rather than the arithmetic",
        )
    }

    #[test]
    fn a_relative_distance_scales_with_the_numbers() {
        // The same fractional disagreement at two ends of the range the
        // stopping power spans. A relative comparison says the same thing about
        // both; an absolute one would not.
        assert!(relatively_close(1.0, 1.0 + 1e-9, coarse()));
        assert!(relatively_close(1e12, 1e12 * (1.0 + 1e-9), coarse()));
        assert!(!relatively_close(1.0, 1.0 + 1e-3, coarse()));
        assert!(!relatively_close(1e12, 1e12 * (1.0 + 1e-3), coarse()));
    }

    #[test]
    fn a_relative_distance_says_nothing_useful_at_zero() {
        // The case the floor exists for. A tally that received no counts is
        // zero, and a small absolute disagreement there is infinitely far away
        // in relative terms.
        assert!(!relatively_close(0.0, 1e-300, coarse()));
    }

    #[test]
    fn the_floor_covers_the_region_around_zero_and_nothing_else() {
        let floor = Tolerance::of(
            1e-12,
            "a tally that received no counts reads as zero, and a disagreement \
             below this is smaller than one count could produce",
        );

        assert!(close_with_floor(0.0, 1e-300, coarse(), floor));
        assert!(!close_with_floor(0.0, 1e-6, coarse(), floor));
        // Away from zero the floor is irrelevant and the relative distance is
        // what decides, so the pair disagreeing by a part in a thousand is
        // still refused.
        assert!(!close_with_floor(1.0, 1.0 + 1e-3, coarse(), floor));
    }

    #[test]
    fn exactness_is_about_the_bits() {
        assert!(exactly_equal(f64::NAN, f64::NAN));
        assert!(!exactly_equal(0.0, -0.0));
        assert!(!exactly_equal(1.0, 1.0 + f64::EPSILON));
    }

    #[test]
    fn the_close_comparisons_are_not_a_way_of_reaching_exactness() {
        // The neighbour of the test above, and the reason the exact comparison
        // is a separate function rather than a tolerance of zero: the other two
        // refuse a non-finite pair rather than answering about it.
        let refused = std::panic::catch_unwind(|| relatively_close(f64::NAN, f64::NAN, coarse()));
        assert!(refused.is_err(), "a NaN was compared instead of refused");
    }

    #[test]
    #[should_panic(expected = "no reason beside it")]
    fn a_tolerance_with_no_reason_is_refused() {
        let _ = Tolerance::of(1e-9, "   ");
    }

    #[test]
    #[should_panic(expected = "is not a distance")]
    fn a_negative_tolerance_is_refused() {
        let _ = Tolerance::of(-1e-9, "a distance cannot be negative");
    }
}

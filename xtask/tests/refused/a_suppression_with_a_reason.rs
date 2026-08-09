//! The neighbour of `a_suppression_with_no_reason.rs`. The same lint is
//! silenced at the same site over the same code, and the reason is the whole
//! difference between the two files.

#[expect(
    clippy::cast_possible_truncation,
    reason = "a history count above four thousand million is refused when the input is \
              read, so the value reaching this cast is inside a u32 by then"
)]
pub fn histories(requested: u64) -> u32 {
    requested as u32
}

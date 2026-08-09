//! SHA-256, because a provenance record's hash is the only thing that notices a
//! regenerated file.
//!
//! Written here rather than taken from a crate. The gate is what every other
//! check is judged by, so a dependency in it is a dependency the gate itself has
//! to be trusted about, and this is eighty lines against the published
//! algorithm. What makes that defensible rather than reckless is that the
//! function has known answers: the vectors at the bottom are the ones the
//! standard publishes, and a single wrong constant fails them.
//!
//! # This is a second copy and it says so
//!
//! `xtask/src/sha256.rs` holds the same function, because the gate needs it to
//! read a record and this crate needs it to write one. Neither crate may depend
//! on the other: the gate is the tool that judges this tree and would then be
//! judging a tree it links, and a product crate depending on the tooling would
//! put the tooling in what ships.
//!
//! What keeps two copies from drifting into two answers is that neither is
//! checked against the other. Both carry the vectors the standard publishes, so
//! a copy that lost a constant fails its own suite rather than agreeing with
//! itself. Removing the second copy means giving the two crates one place to
//! take it from, which is a change to `xtask/`, and issue #15 declares that
//! scope.

/// The first thirty-two bits of the fractional parts of the square roots of the
/// first eight primes.
const INITIAL: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// The first thirty-two bits of the fractional parts of the cube roots of the
/// first sixty-four primes.
const ROUND: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// The digest of `bytes`, lowercase hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    let mut hash = INITIAL;

    // The message, then a one bit, then zeroes, then the length in bits as a big
    // endian sixty-four bit number, padded to a multiple of sixty-four bytes.
    let mut padded = bytes.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    // Wrapping because the standard defines this field modulo two to the
    // sixty-four, not because an overflow here would be tolerable.
    let bit_length = (bytes.len() as u64).wrapping_mul(8);
    padded.extend_from_slice(&bit_length.to_be_bytes());

    for block in padded.chunks_exact(64) {
        let mut schedule = [0u32; 64];
        for (word, four) in schedule.iter_mut().zip(block.chunks_exact(4)) {
            *word = u32::from_be_bytes([four[0], four[1], four[2], four[3]]);
        }
        #[expect(
            clippy::arithmetic_side_effects,
            reason = "index starts at sixteen and the largest step back is sixteen, so \
                      every subtraction here is inside the array by construction"
        )]
        for index in 16..64 {
            let a = schedule[index - 15];
            let b = schedule[index - 2];
            let s0 = a.rotate_right(7) ^ a.rotate_right(18) ^ (a >> 3);
            let s1 = b.rotate_right(17) ^ b.rotate_right(19) ^ (b >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = hash;
        for (round, word) in ROUND.iter().zip(schedule.iter()) {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let first = h
                .wrapping_add(s1)
                .wrapping_add(choose)
                .wrapping_add(*round)
                .wrapping_add(*word);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let second = s0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(first);
            d = c;
            c = b;
            b = a;
            a = first.wrapping_add(second);
        }

        for (into, addend) in hash.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *into = into.wrapping_add(addend);
        }
    }

    hash.iter().map(|word| format!("{word:08x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::hex;

    // The three vectors the standard publishes for messages short enough to
    // paste. Each is a case rather than a repetition: the empty message is the
    // padding-only path, "abc" is one block, and the third is the case that
    // needs a second block for the length alone.

    #[test]
    fn the_empty_message() {
        assert_eq!(
            hex(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn one_block() {
        assert_eq!(
            hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn a_message_whose_length_forces_a_second_block() {
        assert_eq!(
            hex(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
            "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
        );
    }

    #[test]
    fn one_changed_bit_changes_the_digest() {
        // The neighbour to the vectors above. A function that returned a
        // constant would pass none of them, but a function that hashed only the
        // first block would pass the first two, and this is what refuses that.
        assert_ne!(hex(b"abc"), hex(b"abd"));
    }
}

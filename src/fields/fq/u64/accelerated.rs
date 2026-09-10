use super::{ArkworksFq, BigInt, Fq, PrimeField};

fn words(limbs: [u64; 4]) -> [u32; 8] {
    core::array::from_fn(|i| (limbs[i / 2] >> (32 * (i % 2))) as u32)
}

fn limbs(words: [u32; 8]) -> [u64; 4] {
    core::array::from_fn(|i| u64::from(words[2 * i]) | (u64::from(words[2 * i + 1]) << 32))
}

pub(super) fn mul(x: Fq, y: &Fq) -> Fq {
    // (xR) * y mod q is the Montgomery representation of xy.
    let x = words(x.0 .0 .0);
    let modulus = words(Fq::MODULUS_LIMBS);
    // R^-1 mod q, where the existing Arkworks representation uses R = 2^256.
    let r_inverse = words([
        4693556865881009154,
        13627521479405922421,
        9477077619711315503,
        554788901958500396,
    ]);
    let y = product(words(y.0 .0 .0), r_inverse, modulus);
    let result = product(x, y, modulus);
    Fq(ArkworksFq::new_unchecked(BigInt(limbs(result))))
}

pub(super) fn inverse(x: Fq) -> Option<Fq> {
    if x == Fq::ZERO {
        return None;
    }
    let modulus = words(Fq::MODULUS_LIMBS);
    // The inverse of xR times R² is R/x, the Montgomery representation of 1/x.
    let result = product(
        invert(words(x.0 .0 .0), modulus),
        words(ArkworksFq::R2.0),
        modulus,
    );
    Some(Fq(ArkworksFq::new_unchecked(BigInt(limbs(result)))))
}

#[cfg(all(feature = "risc0", target_os = "zkvm"))]
fn invert(x: [u32; 8], modulus: [u32; 8]) -> [u32; 8] {
    let mut result = [0; 8];
    risc0_bigint2::field::modinv_256(&x, &modulus, &mut result);
    result
}

#[cfg(all(test, not(all(feature = "risc0", target_os = "zkvm"))))]
fn invert(x: [u32; 8], modulus: [u32; 8]) -> [u32; 8] {
    use ark_ff::Field;
    assert_eq!(limbs(modulus), Fq::MODULUS_LIMBS);
    let x = ArkworksFq::from_bigint(BigInt(limbs(x))).unwrap();
    words(x.inverse().unwrap().into_bigint().0)
}

#[cfg(all(feature = "risc0", target_os = "zkvm"))]
fn product(x: [u32; 8], y: [u32; 8], modulus: [u32; 8]) -> [u32; 8] {
    let mut result = [0; 8];
    risc0_bigint2::field::modmul_256(&x, &y, &modulus, &mut result);
    result
}

#[cfg(all(test, not(all(feature = "risc0", target_os = "zkvm"))))]
fn product(x: [u32; 8], y: [u32; 8], modulus: [u32; 8]) -> [u32; 8] {
    assert_eq!(limbs(modulus), Fq::MODULUS_LIMBS);
    let x = ArkworksFq::from_bigint(BigInt(limbs(x))).unwrap();
    let y = ArkworksFq::from_bigint(BigInt(limbs(y))).unwrap();
    words((x * y).into_bigint().0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field;
    use ark_std::vec;

    #[test]
    fn accelerated_representation_matches_arkworks() {
        let mut values = vec![Fq::ZERO, Fq::ONE, -Fq::ONE];
        for seed in 0..256u64 {
            values.push(Fq::from_le_limbs([
                seed.wrapping_mul(0x9e3779b97f4a7c15),
                seed.wrapping_mul(0xbf58476d1ce4e5b9),
                seed.wrapping_mul(0x94d049bb133111eb),
                seed,
            ]));
        }
        for x in &values {
            assert_eq!(mul(*x, x), Fq(x.0.square()));
            assert_eq!(inverse(*x), x.0.inverse().map(Fq));
            for y in &values[..3] {
                assert_eq!(mul(*x, y), Fq(x.0 * y.0));
                assert_eq!(mul(*y, x), Fq(y.0 * x.0));
            }
        }
        for pair in values.windows(2) {
            assert_eq!(mul(pair[0], &pair[1]), Fq(pair[0].0 * pair[1].0));
        }
    }
}

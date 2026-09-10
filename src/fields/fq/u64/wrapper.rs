use ark_ed_on_bls12_377::Fq as ArkworksFq;
use ark_ff::{biginteger::BigInt, Field, PrimeField};
use ark_serialize::CanonicalSerialize;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use super::super::{N_64, N_8};

const N: usize = N_64;

#[cfg(any(all(feature = "risc0", target_os = "zkvm"), test))]
#[path = "accelerated.rs"]
mod accelerated;

#[derive(Copy, Clone)]
pub struct Fq(ArkworksFq);

impl PartialEq for Fq {
    fn eq(&self, other: &Self) -> bool {
        match (self.is_sentinel(), other.is_sentinel()) {
            (true, true) => true,
            (true, false) => false,
            (false, true) => false,
            (false, false) => self.0 == other.0,
        }
    }
}

impl Eq for Fq {}

impl zeroize::Zeroize for Fq {
    fn zeroize(&mut self) {
        self.0 .0.zeroize()
    }
}

impl Fq {
    pub(crate) fn from_le_limbs(limbs: [u64; N_64]) -> Fq {
        if let Some(value) = ArkworksFq::from_bigint(BigInt(limbs)) {
            return Self(value);
        }
        let mut bytes = [0u8; N_8];
        for i in 0..N_64 {
            let this_byte = limbs[i].to_le_bytes();
            for j in 0..8 {
                bytes[8 * i + j] = this_byte[j];
            }
        }

        Self::from_raw_bytes(&bytes)
    }

    pub(crate) fn from_raw_bytes(bytes: &[u8; N_8]) -> Fq {
        Self(ArkworksFq::from_le_bytes_mod_order(bytes))
    }

    pub(crate) fn to_le_limbs(&self) -> [u64; N_64] {
        debug_assert!(!self.is_sentinel());

        let le_bytes = self.to_bytes_le();
        let mut out = [0u64; N_64];
        for i in 0..N_64 {
            out[i] = u64::from_le_bytes([
                le_bytes[8 * i],
                le_bytes[8 * i + 1],
                le_bytes[8 * i + 2],
                le_bytes[8 * i + 3],
                le_bytes[8 * i + 4],
                le_bytes[8 * i + 5],
                le_bytes[8 * i + 6],
                le_bytes[8 * i + 7],
            ]);
        }
        out
    }

    pub fn to_bytes_le(&self) -> [u8; N_8] {
        debug_assert!(!self.is_sentinel());

        let mut bytes = [0u8; 32];
        self.0
            .serialize_compressed(&mut bytes[..])
            .expect("serialization into array should be infallible");
        bytes
    }

    /// Instantiate a constant field element from its montgomery limbs.
    ///
    /// This should only be used if you are familiar with the internals of the library.
    pub const fn from_montgomery_limbs(limbs: [u64; N]) -> Fq {
        // The Arkworks `Fp::new_unchecked` method does not perform montgomery reduction.
        Self(ArkworksFq::new_unchecked(BigInt::new(limbs)))
    }

    pub const ZERO: Self = Self(ArkworksFq::new(BigInt::new([0; N])));
    pub const ONE: Self = Self(ArkworksFq::new(BigInt::one()));

    /// A sentinel value which exists only to not be equal to any other field element.
    ///
    /// Operations involving this element are undefined.
    pub const SENTINEL: Self = Self::from_montgomery_limbs([u64::MAX; N_64]);

    fn is_sentinel(&self) -> bool {
        self.0 .0 == Self::SENTINEL.0 .0
    }

    pub fn square(&self) -> Fq {
        debug_assert!(!self.is_sentinel());
        #[cfg(all(feature = "risc0", target_os = "zkvm"))]
        return accelerated::mul(*self, self);
        #[cfg(not(all(feature = "risc0", target_os = "zkvm")))]
        Fq(self.0.square())
    }

    pub fn inverse(&self) -> Option<Fq> {
        debug_assert!(!self.is_sentinel());
        #[cfg(all(feature = "risc0", target_os = "zkvm"))]
        return accelerated::inverse(*self);
        #[cfg(not(all(feature = "risc0", target_os = "zkvm")))]
        {
            if self == &Fq::ZERO {
                return None;
            }
            Some(Fq(self.0.inverse()?))
        }
    }

    pub fn add(self, other: &Fq) -> Fq {
        debug_assert!(!self.is_sentinel() && !other.is_sentinel());
        Fq(self.0 + other.0)
    }

    pub fn sub(self, other: &Fq) -> Fq {
        debug_assert!(!self.is_sentinel() && !other.is_sentinel());
        Fq(self.0 - other.0)
    }

    pub fn mul(self, other: &Fq) -> Fq {
        debug_assert!(!self.is_sentinel() && !other.is_sentinel());
        #[cfg(all(feature = "risc0", target_os = "zkvm"))]
        return accelerated::mul(self, other);
        #[cfg(not(all(feature = "risc0", target_os = "zkvm")))]
        Fq(self.0 * other.0)
    }

    pub fn neg(self) -> Fq {
        debug_assert!(!self.is_sentinel());
        Fq(-self.0)
    }
}

impl ConditionallySelectable for Fq {
    fn conditional_select(a: &Self, b: &Self, choice: subtle::Choice) -> Self {
        let mut out = [0u64; 4];
        let a_limbs = a.0 .0 .0;
        let b_limbs = b.0 .0 .0;
        for i in 0..4 {
            out[i] = u64::conditional_select(&a_limbs[i], &b_limbs[i], choice);
        }
        let bigint = BigInt::new(out);
        Self(ArkworksFq::new(bigint))
    }
}

impl ConstantTimeEq for Fq {
    fn ct_eq(&self, other: &Fq) -> Choice {
        let self_limbs = self.0 .0 .0;
        let other_limbs = other.0 .0 .0;
        let mut is_equal = true;
        for i in 0..4 {
            is_equal &= self_limbs[i] == other_limbs[i];
        }
        Choice::from(is_equal as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::{vec, vec::Vec};

    #[test]
    fn limb_conversion_matches_modular_byte_conversion() {
        let mut values = vec![[0; 4], [1, 0, 0, 0], Fq::MODULUS_LIMBS, [u64::MAX; 4]];
        let mut below_modulus = Fq::MODULUS_LIMBS;
        below_modulus[0] -= 1;
        values.push(below_modulus);
        for seed in 0..256u64 {
            values.push(core::array::from_fn(|i| {
                seed.wrapping_mul(0x9e3779b97f4a7c15u64.wrapping_add(i as u64))
            }));
        }
        for limbs in values {
            let bytes: Vec<u8> = limbs.iter().flat_map(|limb| limb.to_le_bytes()).collect();
            assert_eq!(
                Fq::from_le_limbs(limbs).0,
                ArkworksFq::from_le_bytes_mod_order(&bytes)
            );
        }
    }
}

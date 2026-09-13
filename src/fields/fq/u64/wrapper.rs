use ark_ed_on_bls12_377::Fq as ArkworksFq;
use ark_ff::biginteger::BigInt;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use super::super::{N_64, N_8};

const N: usize = N_64;

#[derive(Copy, Clone)]
pub struct Fq(ArkworksFq);

impl PartialEq for Fq {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.0 .0 .0.ct_eq(&other.0 .0 .0))
    }
}

impl Eq for Fq {}

impl zeroize::Zeroize for Fq {
    fn zeroize(&mut self) {
        self.0 .0.zeroize()
    }
}

impl Fq {
    // Both representations use canonical Montgomery residues with R = 2^256.
    fn as_ct(&self) -> super::super::u32::wrapper::Fq {
        super::super::u32::wrapper::Fq::from_montgomery_limbs(self.0 .0 .0)
    }

    fn from_ct(value: super::super::u32::wrapper::Fq) -> Self {
        Self::from_montgomery_limbs(value.to_montgomery_limbs())
    }

    pub(crate) fn from_le_limbs(limbs: [u64; N_64]) -> Fq {
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
        Self::from_ct(super::super::u32::wrapper::Fq::from_raw_bytes(bytes))
    }

    pub(crate) fn to_montgomery_limbs(&self) -> [u64; N_64] {
        self.0 .0 .0
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
        self.as_ct().to_bytes_le()
    }

    /// Instantiate a constant field element from its montgomery limbs.
    ///
    /// This should only be used if you are familiar with the internals of the library.
    /// Arithmetic requires a canonical Montgomery residue strictly below the modulus.
    /// `SENTINEL` supports equality only; arithmetic on it is undefined.
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
        Self::from_ct(self.as_ct().square())
    }

    pub fn inverse(&self) -> Option<Fq> {
        self.as_ct().inverse().map(Self::from_ct)
    }

    pub fn add(self, other: &Fq) -> Fq {
        Self::from_ct(self.as_ct().add(&other.as_ct()))
    }

    pub fn sub(self, other: &Fq) -> Fq {
        Self::from_ct(self.as_ct().sub(&other.as_ct()))
    }

    pub fn mul(self, other: &Fq) -> Fq {
        Self::from_ct(self.as_ct().mul(&other.as_ct()))
    }

    pub fn neg(self) -> Fq {
        Self::from_ct(self.as_ct().neg())
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
        Self(ArkworksFq::new_unchecked(bigint))
    }
}

impl ConstantTimeEq for Fq {
    fn ct_eq(&self, other: &Fq) -> Choice {
        self.0 .0 .0.ct_eq(&other.0 .0 .0)
    }
}

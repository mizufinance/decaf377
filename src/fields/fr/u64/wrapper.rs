use ark_ed_on_bls12_377::Fr as ArkworksFr;
use ark_ff::biginteger::BigInt;

use subtle::ConstantTimeEq;

use super::super::{N_64, N_8};

const N: usize = N_64;

#[derive(Copy, Clone)]
pub struct Fr(ArkworksFr);

impl PartialEq for Fr {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.0 .0 .0.ct_eq(&other.0 .0 .0))
    }
}

impl Eq for Fr {}

impl zeroize::Zeroize for Fr {
    fn zeroize(&mut self) {
        self.0 .0.zeroize()
    }
}

impl Fr {
    // Both representations use canonical Montgomery residues with R = 2^256.
    fn as_ct(&self) -> super::super::u32::wrapper::Fr {
        super::super::u32::wrapper::Fr::from_montgomery_limbs(self.0 .0 .0)
    }

    fn from_ct(value: super::super::u32::wrapper::Fr) -> Self {
        Self::from_montgomery_limbs(value.to_montgomery_limbs())
    }

    pub(crate) fn from_le_limbs(limbs: [u64; N_64]) -> Fr {
        let mut bytes = [0u8; N_8];
        for i in 0..N_64 {
            let this_byte = limbs[i].to_le_bytes();
            for j in 0..8 {
                bytes[8 * i + j] = this_byte[j];
            }
        }

        Self::from_raw_bytes(&bytes)
    }

    pub(crate) fn from_raw_bytes(bytes: &[u8; N_8]) -> Fr {
        Self::from_ct(super::super::u32::wrapper::Fr::from_raw_bytes(bytes))
    }

    pub(crate) fn to_le_limbs(&self) -> [u64; N_64] {
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

    pub(crate) const fn from_montgomery_limbs(limbs: [u64; N]) -> Fr {
        Self(ArkworksFr::new_unchecked(BigInt::new(limbs)))
    }

    pub const ZERO: Self = Self(ArkworksFr::new(BigInt::new([0; N])));
    pub const ONE: Self = Self(ArkworksFr::new(BigInt::one()));

    pub fn square(&self) -> Fr {
        Self::from_ct(self.as_ct().square())
    }

    pub fn inverse(&self) -> Option<Fr> {
        self.as_ct().inverse().map(Self::from_ct)
    }

    pub fn add(self, other: &Fr) -> Fr {
        Self::from_ct(self.as_ct().add(&other.as_ct()))
    }

    pub fn sub(self, other: &Fr) -> Fr {
        Self::from_ct(self.as_ct().sub(&other.as_ct()))
    }

    pub fn mul(self, other: &Fr) -> Fr {
        Self::from_ct(self.as_ct().mul(&other.as_ct()))
    }

    pub fn neg(self) -> Fr {
        Self::from_ct(self.as_ct().neg())
    }
}

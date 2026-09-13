#![allow(dead_code)]

use subtle::ConstantTimeEq;

use super::{
    super::{B, N_32, N_64, N_8},
    generated as fiat,
};

const N: usize = N_32;

#[derive(Copy, Clone)]
pub struct Fr([u32; N]);

impl PartialEq for Fr {
    fn eq(&self, other: &Self) -> bool {
        bool::from(self.0.ct_eq(&other.0))
    }
}

impl Eq for Fr {}

impl zeroize::Zeroize for Fr {
    fn zeroize(&mut self) {
        self.0.zeroize()
    }
}

impl Fr {
    pub(crate) fn from_le_limbs(limbs: [u64; N_64]) -> Fr {
        let mut bytes = [0u8; N_8];
        for (i, limb) in limbs.iter().enumerate() {
            bytes[8 * i..8 * i + 8].copy_from_slice(&limb.to_le_bytes());
        }
        Self::from_raw_bytes(&bytes)
    }

    pub(crate) fn from_raw_bytes(bytes: &[u8; N_8]) -> Fr {
        // The input spans all 256-bit values. Keep every Fiat operand reduced.
        let mut result = Self::ZERO;
        for byte in bytes.iter().rev() {
            for bit in (0..8).rev() {
                let doubled = result.add(&result);
                let incremented = doubled.add(&Self::ONE);
                let mut out = [0u32; N];
                fiat::fr_selectznz(&mut out, (byte >> bit) & 1, &doubled.0, &incremented.0);
                result = Self(out);
            }
        }
        result
    }

    pub(crate) fn to_montgomery_limbs(&self) -> [u64; N_64] {
        let mut out = [0u64; N_64];
        for (i, limb) in out.iter_mut().enumerate() {
            *limb = self.0[2 * i] as u64 | ((self.0[2 * i + 1] as u64) << 32);
        }
        out
    }

    pub(crate) fn to_le_limbs(&self) -> [u64; N_64] {
        let mut x_non_montgomery = [0; N];
        fiat::fr_from_montgomery(&mut x_non_montgomery, &self.0);
        let limbs = x_non_montgomery;
        let mut out = [0u64; N_64];
        for i in 0..N_64 {
            out[i] = (limbs[2 * i] as u64) | ((limbs[2 * i + 1] as u64) << 32);
        }
        out
    }

    pub fn to_bytes_le(&self) -> [u8; N_8] {
        let mut bytes = [0u8; N_8];
        let mut x_non_montgomery = [0; N];
        fiat::fr_from_montgomery(&mut x_non_montgomery, &self.0);
        fiat::fr_to_bytes(&mut bytes, &x_non_montgomery);
        bytes
    }

    const fn from_montgomery_limbs_backend(limbs: [u32; N]) -> Fr {
        Self(limbs)
    }

    pub(crate) const fn from_montgomery_limbs(limbs: [u64; N_64]) -> Fr {
        Self::from_montgomery_limbs_backend([
            limbs[0] as u32,
            (limbs[0] >> 32) as u32,
            limbs[1] as u32,
            (limbs[1] >> 32) as u32,
            limbs[2] as u32,
            (limbs[2] >> 32) as u32,
            limbs[3] as u32,
            (limbs[3] >> 32) as u32,
        ])
    }

    pub const ZERO: Fr = Self([0; N]);

    pub const ONE: Fr = Self([
        3498574902, 3872500570, 2604314180, 2497411308, 588265454, 3867012838, 3735373809, 66463618,
    ]);

    pub fn square(&self) -> Fr {
        let mut result = [0; N];
        fiat::fr_square(&mut result, &self.0);
        Self(result)
    }

    pub fn inverse(&self) -> Option<Self> {
        if self == &Self::ZERO {
            return None;
        }

        const I: usize = (49 * B + 57) / 17;

        let mut a = [0; N];
        fiat::fr_from_montgomery(&mut a, &self.0);
        let mut d = 1;
        let mut f: [u32; N + 1] = [0u32; N + 1];
        fiat::fr_msat(&mut f);
        let mut g: [u32; N + 1] = [0u32; N + 1];
        let mut v: [u32; N] = [0u32; N];
        let mut r: [u32; N] = Self::ONE.0;
        let mut i = 0;
        let mut j = 0;

        while j < N {
            g[j] = a[j];
            j += 1;
        }

        let mut out1: u32 = 0;
        let mut out2: [u32; N + 1] = [0; N + 1];
        let mut out3: [u32; N + 1] = [0; N + 1];
        let mut out4: [u32; N] = [0; N];
        let mut out5: [u32; N] = [0; N];
        let mut out6: u32 = 0;
        let mut out7: [u32; N + 1] = [0; N + 1];
        let mut out8: [u32; N + 1] = [0; N + 1];
        let mut out9: [u32; N] = [0; N];
        let mut out10: [u32; N] = [0; N];

        while i < I - I % 2 {
            fiat::fr_divstep(
                &mut out1, &mut out2, &mut out3, &mut out4, &mut out5, d, &f, &g, &v, &r,
            );
            fiat::fr_divstep(
                &mut out6, &mut out7, &mut out8, &mut out9, &mut out10, out1, &out2, &out3, &out4,
                &out5,
            );
            d = out6;
            f = out7;
            g = out8;
            v = out9;
            r = out10;
            i += 2;
        }

        if I % 2 != 0 {
            fiat::fr_divstep(
                &mut out1, &mut out2, &mut out3, &mut out4, &mut out5, d, &f, &g, &v, &r,
            );
            v = out4;
            f = out2;
        }

        let s = ((f[f.len() - 1] >> (32 - 1)) & 1) as u8;
        let mut neg = [0; N];
        fiat::fr_opp(&mut neg, &v);

        let mut v_prime: [u32; N] = [0u32; N];
        fiat::fr_selectznz(&mut v_prime, s, &v, &neg);

        let mut pre_comp: [u32; N] = [0u32; N];
        fiat::fr_divstep_precomp(&mut pre_comp);

        let mut result = [0; N];
        fiat::fr_mul(&mut result, &v_prime, &pre_comp);

        Some(Fr(result))
    }

    pub fn add(self, other: &Fr) -> Fr {
        let mut result = [0; N];
        fiat::fr_add(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn sub(self, other: &Fr) -> Fr {
        let mut result = [0; N];
        fiat::fr_sub(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn mul(self, other: &Fr) -> Fr {
        let mut result = [0; N];
        fiat::fr_mul(&mut result, &self.0, &other.0);
        Fr(result)
    }

    pub fn neg(self) -> Fr {
        let mut result = [0; N];
        fiat::fr_opp(&mut result, &self.0);
        Fr(result)
    }
}

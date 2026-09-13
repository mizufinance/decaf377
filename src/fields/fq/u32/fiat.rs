//! Compatibility interface for the original typed arithmetic API.
//! Arithmetic is delegated to the shared generated Rust32 implementation.
//! Field operands must be canonical residues below the modulus; domain
//! conversions preserve the original Montgomery/non-Montgomery contracts.

/** FqU1 represents values of 1 bits, stored in one byte. */
pub type FqU1 = u8;
/** FqI1 represents values of 1 bits, stored in one byte. */
pub type FqI1 = i8;
/** FqU2 represents values of 2 bits, stored in one byte. */
pub type FqU2 = u8;
/** FqI2 represents values of 2 bits, stored in one byte. */
pub type FqI2 = i8;

/** The type FqMontgomeryDomainFieldElement is a field element in the Montgomery domain. */
/** Bounds: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]] */
#[derive(Clone, Copy)]
pub struct FqMontgomeryDomainFieldElement(pub [u32; 8]);

impl core::ops::Index<usize> for FqMontgomeryDomainFieldElement {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for FqMontgomeryDomainFieldElement {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/** The type FqNonMontgomeryDomainFieldElement is a field element NOT in the Montgomery domain. */
/** Bounds: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]] */
#[derive(Clone, Copy)]
pub struct FqNonMontgomeryDomainFieldElement(pub [u32; 8]);

impl core::ops::Index<usize> for FqNonMontgomeryDomainFieldElement {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for FqNonMontgomeryDomainFieldElement {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub fn fq_addcarryx_u32(out1: &mut u32, out2: &mut FqU1, arg1: FqU1, arg2: u32, arg3: u32) {
    super::generated::fq_addcarryx_u32(out1, out2, arg1, arg2, arg3);
}

pub fn fq_subborrowx_u32(out1: &mut u32, out2: &mut FqU1, arg1: FqU1, arg2: u32, arg3: u32) {
    super::generated::fq_subborrowx_u32(out1, out2, arg1, arg2, arg3);
}

pub fn fq_mulx_u32(out1: &mut u32, out2: &mut u32, arg1: u32, arg2: u32) {
    super::generated::fq_mulx_u32(out1, out2, arg1, arg2);
}

pub fn fq_cmovznz_u32(out1: &mut u32, arg1: FqU1, arg2: u32, arg3: u32) {
    super::generated::fq_cmovznz_u32(out1, arg1, arg2, arg3);
}

pub fn fq_mul(
    out1: &mut FqMontgomeryDomainFieldElement,
    arg1: &FqMontgomeryDomainFieldElement,
    arg2: &FqMontgomeryDomainFieldElement,
) {
    super::generated::fq_mul(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fq_square(out1: &mut FqMontgomeryDomainFieldElement, arg1: &FqMontgomeryDomainFieldElement) {
    super::generated::fq_square(&mut out1.0, &arg1.0);
}

pub fn fq_add(
    out1: &mut FqMontgomeryDomainFieldElement,
    arg1: &FqMontgomeryDomainFieldElement,
    arg2: &FqMontgomeryDomainFieldElement,
) {
    super::generated::fq_add(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fq_sub(
    out1: &mut FqMontgomeryDomainFieldElement,
    arg1: &FqMontgomeryDomainFieldElement,
    arg2: &FqMontgomeryDomainFieldElement,
) {
    super::generated::fq_sub(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fq_opp(out1: &mut FqMontgomeryDomainFieldElement, arg1: &FqMontgomeryDomainFieldElement) {
    super::generated::fq_opp(&mut out1.0, &arg1.0);
}

pub fn fq_from_montgomery(
    out1: &mut FqNonMontgomeryDomainFieldElement,
    arg1: &FqMontgomeryDomainFieldElement,
) {
    super::generated::fq_from_montgomery(&mut out1.0, &arg1.0);
}

pub fn fq_to_montgomery(
    out1: &mut FqMontgomeryDomainFieldElement,
    arg1: &FqNonMontgomeryDomainFieldElement,
) {
    super::generated::fq_to_montgomery(&mut out1.0, &arg1.0);
}

pub fn fq_nonzero(out1: &mut u32, arg1: &[u32; 8]) {
    super::generated::fq_nonzero(out1, arg1);
}

pub fn fq_selectznz(out1: &mut [u32; 8], arg1: FqU1, arg2: &[u32; 8], arg3: &[u32; 8]) {
    super::generated::fq_selectznz(out1, arg1, arg2, arg3);
}

pub fn fq_to_bytes(out1: &mut [u8; 32], arg1: &[u32; 8]) {
    super::generated::fq_to_bytes(out1, arg1);
}

pub fn fq_from_bytes(out1: &mut [u32; 8], arg1: &[u8; 32]) {
    super::generated::fq_from_bytes(out1, arg1);
}

pub fn fq_set_one(out1: &mut FqMontgomeryDomainFieldElement) {
    super::generated::fq_set_one(&mut out1.0);
}

pub fn fq_msat(out1: &mut [u32; 9]) {
    super::generated::fq_msat(out1);
}

pub fn fq_divstep_precomp(out1: &mut [u32; 8]) {
    super::generated::fq_divstep_precomp(out1);
}

pub fn fq_divstep(
    out1: &mut u32,
    out2: &mut [u32; 9],
    out3: &mut [u32; 9],
    out4: &mut [u32; 8],
    out5: &mut [u32; 8],
    arg1: u32,
    arg2: &[u32; 9],
    arg3: &[u32; 9],
    arg4: &[u32; 8],
    arg5: &[u32; 8],
) {
    super::generated::fq_divstep(out1, out2, out3, out4, out5, arg1, arg2, arg3, arg4, arg5);
}

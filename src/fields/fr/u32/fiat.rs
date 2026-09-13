//! Compatibility interface for the original typed arithmetic API.
//! Arithmetic is delegated to the shared generated Rust32 implementation.
//! Field operands must be canonical residues below the modulus; domain
//! conversions preserve the original Montgomery/non-Montgomery contracts.

/** FrU1 represents values of 1 bits, stored in one byte. */
pub type FrU1 = u8;
/** FrI1 represents values of 1 bits, stored in one byte. */
pub type FrI1 = i8;
/** FrU2 represents values of 2 bits, stored in one byte. */
pub type FrU2 = u8;
/** FrI2 represents values of 2 bits, stored in one byte. */
pub type FrI2 = i8;

/** The type FrMontgomeryDomainFieldElement is a field element in the Montgomery domain. */
/** Bounds: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]] */
#[derive(Clone, Copy)]
pub struct FrMontgomeryDomainFieldElement(pub [u32; 8]);

impl core::ops::Index<usize> for FrMontgomeryDomainFieldElement {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for FrMontgomeryDomainFieldElement {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/** The type FrNonMontgomeryDomainFieldElement is a field element NOT in the Montgomery domain. */
/** Bounds: [[0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff], [0x0 ~> 0xffffffff]] */
#[derive(Clone, Copy)]
pub struct FrNonMontgomeryDomainFieldElement(pub [u32; 8]);

impl core::ops::Index<usize> for FrNonMontgomeryDomainFieldElement {
    type Output = u32;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl core::ops::IndexMut<usize> for FrNonMontgomeryDomainFieldElement {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

pub fn fr_addcarryx_u32(out1: &mut u32, out2: &mut FrU1, arg1: FrU1, arg2: u32, arg3: u32) {
    super::generated::fr_addcarryx_u32(out1, out2, arg1, arg2, arg3);
}

pub fn fr_subborrowx_u32(out1: &mut u32, out2: &mut FrU1, arg1: FrU1, arg2: u32, arg3: u32) {
    super::generated::fr_subborrowx_u32(out1, out2, arg1, arg2, arg3);
}

pub fn fr_mulx_u32(out1: &mut u32, out2: &mut u32, arg1: u32, arg2: u32) {
    super::generated::fr_mulx_u32(out1, out2, arg1, arg2);
}

pub fn fr_cmovznz_u32(out1: &mut u32, arg1: FrU1, arg2: u32, arg3: u32) {
    super::generated::fr_cmovznz_u32(out1, arg1, arg2, arg3);
}

pub fn fr_mul(
    out1: &mut FrMontgomeryDomainFieldElement,
    arg1: &FrMontgomeryDomainFieldElement,
    arg2: &FrMontgomeryDomainFieldElement,
) {
    super::generated::fr_mul(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fr_square(out1: &mut FrMontgomeryDomainFieldElement, arg1: &FrMontgomeryDomainFieldElement) {
    super::generated::fr_square(&mut out1.0, &arg1.0);
}

pub fn fr_add(
    out1: &mut FrMontgomeryDomainFieldElement,
    arg1: &FrMontgomeryDomainFieldElement,
    arg2: &FrMontgomeryDomainFieldElement,
) {
    super::generated::fr_add(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fr_sub(
    out1: &mut FrMontgomeryDomainFieldElement,
    arg1: &FrMontgomeryDomainFieldElement,
    arg2: &FrMontgomeryDomainFieldElement,
) {
    super::generated::fr_sub(&mut out1.0, &arg1.0, &arg2.0);
}

pub fn fr_opp(out1: &mut FrMontgomeryDomainFieldElement, arg1: &FrMontgomeryDomainFieldElement) {
    super::generated::fr_opp(&mut out1.0, &arg1.0);
}

pub fn fr_from_montgomery(
    out1: &mut FrNonMontgomeryDomainFieldElement,
    arg1: &FrMontgomeryDomainFieldElement,
) {
    super::generated::fr_from_montgomery(&mut out1.0, &arg1.0);
}

pub fn fr_to_montgomery(
    out1: &mut FrMontgomeryDomainFieldElement,
    arg1: &FrNonMontgomeryDomainFieldElement,
) {
    super::generated::fr_to_montgomery(&mut out1.0, &arg1.0);
}

pub fn fr_nonzero(out1: &mut u32, arg1: &[u32; 8]) {
    super::generated::fr_nonzero(out1, arg1);
}

pub fn fr_selectznz(out1: &mut [u32; 8], arg1: FrU1, arg2: &[u32; 8], arg3: &[u32; 8]) {
    super::generated::fr_selectznz(out1, arg1, arg2, arg3);
}

pub fn fr_to_bytes(out1: &mut [u8; 32], arg1: &[u32; 8]) {
    super::generated::fr_to_bytes(out1, arg1);
}

pub fn fr_from_bytes(out1: &mut [u32; 8], arg1: &[u8; 32]) {
    super::generated::fr_from_bytes(out1, arg1);
}

pub fn fr_set_one(out1: &mut FrMontgomeryDomainFieldElement) {
    super::generated::fr_set_one(&mut out1.0);
}

pub fn fr_msat(out1: &mut [u32; 9]) {
    super::generated::fr_msat(out1);
}

pub fn fr_divstep_precomp(out1: &mut [u32; 8]) {
    super::generated::fr_divstep_precomp(out1);
}

pub fn fr_divstep(
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
    super::generated::fr_divstep(out1, out2, out3, out4, out5, arg1, arg2, arg3, arg4, arg5);
}

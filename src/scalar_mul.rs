//! Fixed-schedule extended Edwards arithmetic over the Fiat field backend.
use crate::{fields::fq::CtFq, Fq};
use subtle::{Choice, ConditionallySelectable};

#[derive(Clone, Copy)]
pub(crate) struct Point {
    x: CtFq,
    y: CtFq,
    z: CtFq,
    t: CtFq,
}

impl Point {
    const IDENTITY: Self = Self {
        x: CtFq::ZERO,
        y: CtFq::ONE,
        z: CtFq::ONE,
        t: CtFq::ZERO,
    };
    const D: CtFq = CtFq::from_montgomery_limbs([
        15008245758212136496,
        17341409599856531410,
        648869460136961410,
        719771289660577536,
    ]);

    pub(crate) fn from_projective([x, y, z, t]: [Fq; 4]) -> Self {
        let convert = |f: Fq| CtFq::from_montgomery_limbs(f.to_montgomery_limbs());
        Self {
            x: convert(x),
            y: convert(y),
            z: convert(z),
            t: convert(t),
        }
    }

    #[cfg(feature = "arkworks")]
    pub(crate) fn from_affine(x: Fq, y: Fq) -> Self {
        let x = CtFq::from_montgomery_limbs(x.to_montgomery_limbs());
        let y = CtFq::from_montgomery_limbs(y.to_montgomery_limbs());
        Self {
            x,
            y,
            z: CtFq::ONE,
            t: x.mul(&y),
        }
    }

    pub(crate) fn projective(self) -> [Fq; 4] {
        [self.x, self.y, self.z, self.t].map(|f| Fq::from_montgomery_limbs(f.to_montgomery_limbs()))
    }

    #[cfg(feature = "arkworks")]
    pub(crate) fn affine(self) -> [Fq; 2] {
        // z is nonzero for valid extended points. The exponent is public q - 2.
        let mut exponent = Fq::MODULUS_LIMBS;
        exponent[0] -= 2;
        let mut inverse = CtFq::ONE;
        for limb in exponent.iter().rev() {
            for bit in (0..64).rev() {
                inverse = inverse.square();
                if (limb >> bit) & 1 == 1 {
                    inverse = inverse.mul(&self.z);
                }
            }
        }
        [self.x.mul(&inverse), self.y.mul(&inverse)]
            .map(|f| Fq::from_montgomery_limbs(f.to_montgomery_limbs()))
    }

    fn add(self, other: Self) -> Self {
        // Complete extended twisted Edwards addition, a = -1 (Hisil et al.).
        let a = self.x.mul(&other.x);
        let b = self.y.mul(&other.y);
        let c = Self::D.mul(&self.t).mul(&other.t);
        let d = self.z.mul(&other.z);
        let e = self
            .x
            .add(&self.y)
            .mul(&other.x.add(&other.y))
            .sub(&a)
            .sub(&b);
        let f = d.sub(&c);
        let g = d.add(&c);
        let h = b.add(&a);
        Self {
            x: e.mul(&f),
            y: g.mul(&h),
            t: e.mul(&h),
            z: f.mul(&g),
        }
    }

    fn select(a: Self, b: Self, choice: Choice) -> Self {
        Self {
            x: CtFq::conditional_select(&a.x, &b.x, choice),
            y: CtFq::conditional_select(&a.y, &b.y, choice),
            z: CtFq::conditional_select(&a.z, &b.z, choice),
            t: CtFq::conditional_select(&a.t, &b.t, choice),
        }
    }

    // The limb count is public. Fr operators always supply four limbs.
    pub(crate) fn mul(self, limbs: &[u64]) -> Self {
        let mut result = Self::IDENTITY;
        let mut current = self;
        for limb in limbs {
            for bit in 0..64 {
                let sum = result.add(current);
                result = Self::select(result, sum, Choice::from(((limb >> bit) & 1) as u8));
                current = current.add(current);
            }
        }
        result
    }
}

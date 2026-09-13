//! Fixed-schedule extended Edwards arithmetic over the Fiat field backend.
use crate::{fields::fq::CtFq, Fq};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

// Exponents and loop bounds here are public field constants. In particular,
// inversion maps zero to zero without the exceptional branch in Option-based
// inversion. Compression needs this behavior at the identity.
fn pow_public(base: CtFq, exponent: &[u64; 4]) -> CtFq {
    let mut out = CtFq::ONE;
    for limb in exponent.iter().rev() {
        for bit in (0..64).rev() {
            out = out.square();
            if (limb >> bit) & 1 == 1 {
                out = out.mul(&base);
            }
        }
    }
    out
}

fn sqrt_fixed(x: CtFq) -> CtFq {
    let mut z = pow_public(x, &Fq::TRACE_MINUS_ONE_DIV_TWO_LIMBS);
    let mut t = z.square().mul(&x);
    z = z.mul(&x);
    let mut b = t;
    let mut c =
        CtFq::from_montgomery_limbs(Fq::QUADRATIC_NON_RESIDUE_TO_TRACE.to_montgomery_limbs());
    for i in (2..=Fq::TWO_ADICITY).rev() {
        for _ in 1..=i - 2 {
            b = b.square();
        }
        let choice = !b.ct_eq(&CtFq::ONE);
        z = CtFq::conditional_select(&z, &z.mul(&c), choice);
        c = c.square();
        t = CtFq::conditional_select(&t, &t.mul(&c), choice);
        b = t;
    }
    z
}

fn abs_fixed(x: CtFq) -> CtFq {
    CtFq::conditional_select(&x, &x.neg(), Choice::from(x.to_bytes_le()[0] & 1))
}

#[derive(Clone, Copy)]
pub(crate) struct Point {
    x: CtFq,
    y: CtFq,
    z: CtFq,
    t: CtFq,
}

impl Point {
    /// Fixed-schedule Decaf compression for valid group representatives.
    /// The group invariant makes the inverse-square-root argument a square
    /// (or zero at the identity); root sign is removed by the final abs.
    pub(crate) fn compress_to_field(self) -> Fq {
        let a_minus_d = CtFq::ONE.neg().sub(&Self::D);
        let u1 = self.x.add(&self.t).mul(&self.x.sub(&self.t));
        let denominator = u1.mul(&a_minus_d).mul(&self.x.square());
        let mut exponent = Fq::MODULUS_LIMBS;
        exponent[0] -= 2;
        let v = sqrt_fixed(pow_public(denominator, &exponent));
        let u2 = abs_fixed(v.mul(&u1));
        let u3 = u2.mul(&self.z).sub(&self.t);
        let s = abs_fixed(a_minus_d.mul(&v).mul(&u3).mul(&self.x));
        Fq::from_montgomery_limbs(s.to_montgomery_limbs())
    }

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

#[cfg(test)]
mod encoding_tests {
    use super::*;

    #[test]
    fn compression_preserves_projective_and_quotient_representatives() {
        let x = Fq::from_montgomery_limbs([
            5825153684096051627,
            16988948339439369204,
            186539475124256708,
            1230075515893193738,
        ]);
        let y = Fq::from_montgomery_limbs([
            9786171649960077610,
            13527783345193426398,
            10983305067350511165,
            1251302644532346138,
        ]);
        let expected = crate::Element::GENERATOR.vartime_compress_to_field();
        for scale in [Fq::ONE, -Fq::ONE, Fq::from(7u64), Fq::from(u128::MAX)] {
            for (x, y, encoding) in [
                (x, y, expected),
                (-x, -y, expected),
                (Fq::ZERO, Fq::ONE, Fq::ZERO),
                (Fq::ZERO, -Fq::ONE, Fq::ZERO),
            ] {
                let point = Point::from_projective([x * scale, y * scale, scale, x * y * scale]);
                assert_eq!(point.compress_to_field(), encoding);
            }
        }
    }
}

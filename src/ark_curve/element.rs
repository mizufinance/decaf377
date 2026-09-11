use ark_ec::{AffineRepr, CurveGroup, PrimeGroup, ScalarMul, VariableBaseMSM};
use ark_ff::AdditiveGroup;
use ark_serialize::Valid;
use ark_std::vec::Vec;
use core::ops::AddAssign;

use crate::{
    ark_curve::{edwards::EdwardsAffine, Decaf377EdwardsConfig, EdwardsProjective},
    Fq, Fr,
};

pub mod affine;
pub mod projective;

pub use affine::AffinePoint;
pub use projective::Element;

impl Valid for Element {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        Ok(())
    }
}

impl ScalarMul for Element {
    type MulBase = AffinePoint;

    const NEGATION_IS_CHEAP: bool = true;

    fn batch_convert_to_mul_base(bases: &[Self]) -> Vec<Self::MulBase> {
        let bases_inner = bases.iter().map(|g| g.inner).collect::<Vec<_>>();
        let result = EdwardsProjective::batch_convert_to_mul_base(&bases_inner[..]);
        result
            .into_iter()
            .map(|g| AffinePoint { inner: g })
            .collect::<Vec<_>>()
    }
}

impl VariableBaseMSM for Element {}

impl CurveGroup for Element {
    // We implement `CurveGroup` as it is required by the `CurveVar`
    // trait used in the R1CS feature. The `ProjectiveCurve` trait requires
    // an affine representation of `Element` to be defined, and `AffineRepr`
    // to be implemented on that type.
    type Config = Decaf377EdwardsConfig;

    type BaseField = Fq;

    type Affine = AffinePoint;

    // This type is supposed to represent an element of the entire elliptic
    // curve group, not just the prime-order subgroup. Since this is decaf,
    // this is just an `Element` again.
    type FullGroup = AffinePoint;

    fn normalize_batch(v: &[Self]) -> Vec<AffinePoint> {
        let v_inner = v.iter().map(|g| g.inner).collect::<Vec<_>>();
        let result = EdwardsProjective::normalize_batch(&v_inner[..]);
        result
            .into_iter()
            .map(|g| AffinePoint { inner: g })
            .collect::<Vec<_>>()
    }

    fn into_affine(self) -> Self::Affine {
        self.into()
    }
}

impl Valid for AffinePoint {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        Ok(())
    }
}

impl AffineRepr for AffinePoint {
    type Config = Decaf377EdwardsConfig;

    type ScalarField = Fr;

    type BaseField = Fq;

    type Group = Element;

    fn xy(&self) -> Option<(Self::BaseField, Self::BaseField)> {
        self.inner.xy()
    }

    fn zero() -> Self {
        AffinePoint {
            inner: EdwardsAffine::zero(),
        }
    }

    fn generator() -> Self {
        Element::GENERATOR.into()
    }

    fn from_random_bytes(bytes: &[u8]) -> Option<Self> {
        EdwardsAffine::from_random_bytes(bytes).map(|inner| AffinePoint { inner })
    }

    fn mul_bigint(&self, other: impl AsRef<[u64]>) -> Self::Group {
        let [x, y, z, t] = crate::scalar_mul::Point::from_affine(self.inner.x, self.inner.y)
            .mul(other.as_ref())
            .projective();
        Element {
            inner: EdwardsProjective::new_unchecked(x, y, t, z),
        }
    }

    fn clear_cofactor(&self) -> Self {
        // This is decaf so we're just returning the same point.
        *self
    }

    fn mul_by_cofactor_to_group(&self) -> Self::Group {
        self.into()
    }
}

impl From<Element> for AffinePoint {
    fn from(point: Element) -> Self {
        let p = point.inner;
        let [x, y] = crate::scalar_mul::Point::from_projective([p.x, p.y, p.z, p.t]).affine();
        Self {
            inner: EdwardsAffine::new_unchecked(x, y),
        }
    }
}

impl From<AffinePoint> for Element {
    fn from(point: AffinePoint) -> Self {
        let [x, y, z, t] =
            crate::scalar_mul::Point::from_affine(point.inner.x, point.inner.y).projective();
        Self {
            inner: EdwardsProjective::new_unchecked(x, y, t, z),
        }
    }
}

impl From<&Element> for AffinePoint {
    fn from(point: &Element) -> Self {
        (*point).into()
    }
}

impl From<&AffinePoint> for Element {
    fn from(point: &AffinePoint) -> Self {
        (*point).into()
    }
}

impl PrimeGroup for Element {
    type ScalarField = Fr;

    fn generator() -> Self {
        Self::GENERATOR
    }

    fn mul_bigint(&self, other: impl AsRef<[u64]>) -> Self {
        self.scalar_mul(other.as_ref())
    }
}

impl AdditiveGroup for Element {
    type Scalar = Fr;

    const ZERO: Self = Self::ZERO;

    fn double(&self) -> Self {
        let mut copy = *self;
        copy.double_in_place();
        copy
    }

    fn double_in_place(&mut self) -> &mut Self {
        self.add_assign(*self);
        self
    }

    fn neg_in_place(&mut self) -> &mut Self {
        *self = -(*self);
        self
    }
}

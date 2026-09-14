use decaf377::{Element, Fr};

#[test]
fn scalar_operators_cover_identity_and_high_bits() {
    let g = Element::GENERATOR;
    let mut high = [0u8; 32];
    high[31] = 4;
    let high = Fr::from_bytes_checked(&high).unwrap();
    for (s, expected) in [(Fr::ZERO, Element::IDENTITY), (Fr::ONE, g), (-Fr::ONE, -g)] {
        assert_eq!(g * s, expected);
        assert_eq!(&g * &s, expected);
        assert_eq!(s * g, expected);
        assert_eq!(&s * &g, expected);
        let mut assigned = g;
        assigned *= s;
        assert_eq!(assigned, expected);
        assigned = g;
        assigned *= &s;
        assert_eq!(assigned, expected);
        assert_eq!(Element::IDENTITY * s, Element::IDENTITY);
    }
    let mut expected = g;
    for _ in 0..250 {
        expected += expected;
    }
    assert_eq!(g * high, expected);
}

#[cfg(feature = "arkworks")]
#[test]
fn affine_and_bigint_paths_match_projective_results() {
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::PrimeField;
    let g = Element::GENERATOR;
    let affine = g.into_affine();
    assert_eq!(
        Element::from(Element::ZERO.into_affine()),
        Element::IDENTITY
    );
    assert_eq!(Element::ZERO * Fr::ONE, Element::IDENTITY);
    for s in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(123456789u64)] {
        let expected = g * s;
        let actual: Element = (&affine * &s).into();
        assert_eq!(actual, expected);
        assert_eq!(affine * s, expected);
        let mut assigned = affine;
        assigned *= s;
        assert_eq!(Element::from(assigned), expected);
        assigned = affine;
        assigned *= &s;
        assert_eq!(Element::from(assigned), expected);
        let limbs = s.into_bigint();
        assert_eq!(g.mul_bigint(limbs), expected);
        assert_eq!(affine.mul_bigint(limbs), expected);
        let mut scalar = s;
        assert_eq!(g * &mut scalar, expected);
        let mut assigned = g;
        assigned *= &mut scalar;
        assert_eq!(assigned, expected);
        assert_eq!(scalar, s);
    }
    // Integer APIs must not truncate to the 251-bit scalar-field width.
    let limbs = [0, 0, 0, 1u64 << 63];
    let mut expected = g;
    for _ in 0..255 {
        expected += expected;
    }
    assert_eq!(g.mul_bigint(limbs), expected);
    assert_eq!(affine.mul_bigint(limbs), expected);
}

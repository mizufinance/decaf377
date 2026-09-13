use decaf377::{Element, Encoding, Fq, Fr};

fn check(point: Element) {
    let expected = point.vartime_compress();
    let actual = point.compress();
    assert_eq!(actual.0, expected.0);
    assert_eq!(point.compress_to_field(), point.vartime_compress_to_field());
    assert_eq!(actual.0[31] >> 5, 0);
    assert_eq!(actual.0[0] & 1, 0);
    assert_eq!(Encoding::from(point).0, actual.0);
    assert_eq!(Encoding::from(&point).0, actual.0);
    assert_eq!(<[u8; 32]>::from(point), actual.0);
    #[cfg(feature = "arkworks")]
    {
        use ark_serialize::CanonicalSerialize;
        let mut bytes = [0u8; 32];
        point.serialize_compressed(&mut bytes[..]).unwrap();
        assert_eq!(bytes, actual.0);
    }
    assert_eq!(Encoding(actual.0).vartime_decompress().unwrap(), point);
}

#[test]
fn fixed_schedule_encoding_matches_existing_wire_encoding() {
    check(Element::IDENTITY);
    check(Element::GENERATOR);
    check(-Element::GENERATOR);
    check(Element::GENERATOR + -Element::GENERATOR);
    for scalar in [
        Fr::ZERO,
        Fr::ONE,
        -Fr::ONE,
        Fr::from(2u64),
        Fr::from(u128::MAX),
    ] {
        check(scalar * Element::GENERATOR);
    }
    let mut state = 0x123456789abcdef0u64;
    for _ in 0..64 {
        let mut bytes = [0u8; 64];
        for chunk in bytes.chunks_mut(8) {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        let p = Fr::from_le_bytes_mod_order(&bytes) * Element::GENERATOR;
        check(p);
        check(-p);
        check(p + Element::GENERATOR);
        check(p + p);
        let r1 = Fq::from_le_bytes_mod_order(&bytes[..32]);
        let r2 = Fq::from_le_bytes_mod_order(&bytes[32..]);
        check(Element::encode_to_curve(&r1));
        check(Element::hash_to_curve(&r1, &r2));
    }
}

#[cfg(feature = "r1cs")]
#[test]
fn constraint_field_and_allocation_preserve_encoding() {
    use ark_ff::ToConstraintField;
    use ark_r1cs_std::{alloc::AllocVar, R1CSVar};
    use ark_relations::r1cs::ConstraintSystem;
    use decaf377::r1cs::ElementVar;

    for scalar in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(17u64)] {
        let point = scalar * Element::GENERATOR;
        let expected = point.vartime_compress_to_field();
        assert_eq!(point.to_field_elements().unwrap(), vec![expected]);
        let cs = ConstraintSystem::<Fq>::new_ref();
        let input = ElementVar::new_input(cs.clone(), || Ok(point)).unwrap();
        let witness = ElementVar::new_witness(cs.clone(), || Ok(point)).unwrap();
        assert_eq!(input.compress_to_field().unwrap().value().unwrap(), expected);
        assert_eq!(witness.compress_to_field().unwrap().value().unwrap(), expected);
        assert!(cs.is_satisfied().unwrap());
    }
}

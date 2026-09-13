use decaf377::{Element, Encoding, Fq, Fr};

fn check(point: Element) {
    let expected = point.vartime_compress();
    let actual = point.compress();
    assert_eq!(actual.0, expected.0);
    assert_eq!(point.compress_to_field(), point.vartime_compress_to_field());
    assert_eq!(actual.0[31] >> 5, 0);
    assert_eq!(actual.0[0] & 1, 0);
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

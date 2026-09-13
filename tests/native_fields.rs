use decaf377::{Fq, Fr};
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

#[test]
fn conditional_selection_preserves_montgomery_residues() {
    let left = Fq::from(7u64);
    let right = Fq::from(123u64);
    for (choice, expected) in [(0, left), (1, right)] {
        let selected = Fq::conditional_select(&left, &right, Choice::from(choice));
        assert_eq!(selected.to_bytes(), expected.to_bytes());
        assert!(bool::from(selected.ct_eq(&expected)));
    }
    assert!(bool::from(Fq::SENTINEL.ct_eq(&Fq::SENTINEL)));
    assert!(!bool::from(Fq::SENTINEL.ct_eq(&Fq::ZERO)));
    assert!(bool::from(
        Fq::conditional_select(&Fq::ZERO, &Fq::SENTINEL, Choice::from(1)).ct_eq(&Fq::SENTINEL)
    ));
}

#[test]
fn public_typed_scalar_arithmetic_remains_compatible() {
    use decaf377::fields::fr::u32::fiat::*;
    let mut one = FrMontgomeryDomainFieldElement([0; 8]);
    fr_set_one(&mut one);
    let mut product = FrMontgomeryDomainFieldElement([0; 8]);
    fr_mul(&mut product, &one, &one);
    assert_eq!(product.0, one.0);
    let mut plain = FrNonMontgomeryDomainFieldElement([0; 8]);
    fr_from_montgomery(&mut plain, &product);
    assert_eq!(plain[0], 1);
    assert_eq!(&plain.0[1..], &[0; 7]);
    plain[0] = 7;
    fr_to_montgomery(&mut product, &plain);
    fr_from_montgomery(&mut plain, &product);
    assert_eq!(plain.0, [7, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn zero_inverse_and_basic_field_identities() {
    assert!(Fq::ZERO.inverse().is_none());
    assert!(Fr::ZERO.inverse().is_none());
    for value in [1, 2, 7, 31, u32::MAX as u64, u64::MAX] {
        let q = Fq::from(value);
        let r = Fr::from(value);
        assert_eq!((q * q.inverse().unwrap()).to_bytes(), Fq::ONE.to_bytes());
        assert_eq!((r * r.inverse().unwrap()).to_bytes(), Fr::ONE.to_bytes());
        assert_eq!((q + -q).to_bytes(), Fq::ZERO.to_bytes());
        assert_eq!((r + -r).to_bytes(), Fr::ZERO.to_bytes());
        assert_eq!(q.square().to_bytes(), (q * q).to_bytes());
        assert_eq!(r.square().to_bytes(), (r * r).to_bytes());
    }
}

#[cfg(feature = "arkworks")]
#[test]
fn native_fields_match_independent_arkworks_boundaries() {
    use ark_ff::{Field, PrimeField};
    use ark_serialize::CanonicalSerialize;

    fn bytes<F: CanonicalSerialize>(value: F) -> [u8; 32] {
        let mut output = [0u8; 32];
        value.serialize_compressed(&mut output[..]).unwrap();
        output
    }
    let mut inputs = vec![[0u8; 32], [0xff; 32]];
    for limbs in [Fq::MODULUS_LIMBS, Fr::MODULUS_LIMBS] {
        let mut modulus = [0u8; 32];
        for (i, limb) in limbs.iter().enumerate() {
            modulus[8 * i..8 * i + 8].copy_from_slice(&limb.to_le_bytes());
        }
        inputs.push(modulus);
        let mut lower = modulus;
        for byte in &mut lower {
            let (result, borrow) = byte.overflowing_sub(1);
            *byte = result;
            if !borrow {
                break;
            }
        }
        inputs.push(lower);
        let mut upper = modulus;
        for byte in &mut upper {
            let (result, carry) = byte.overflowing_add(1);
            *byte = result;
            if !carry {
                break;
            }
        }
        inputs.push(upper);
    }
    for bit in 0..256 {
        let mut input = [0u8; 32];
        input[bit / 8] = 1 << (bit % 8);
        inputs.push(input);
    }
    // Deterministic varied limbs supplement the carry/reduction boundaries.
    let mut state = 0x4d595df4d0f33173u64;
    for _ in 0..40 {
        let mut input = [0u8; 32];
        for chunk in input.chunks_mut(8) {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            chunk.copy_from_slice(&state.to_le_bytes());
        }
        inputs.push(input);
    }
    for (index, input) in inputs.iter().enumerate() {
        let other = &inputs[(index + 1) % inputs.len()];
        macro_rules! compare {
            ($ours:ty, $reference:ty) => {{
                let a = <$ours>::from_le_bytes_mod_order(input);
                let b = <$ours>::from_le_bytes_mod_order(other);
                let x = <$reference>::from_le_bytes_mod_order(input);
                let y = <$reference>::from_le_bytes_mod_order(other);
                assert_eq!(a.to_bytes(), bytes(x));
                assert_eq!((a + b).to_bytes(), bytes(x + y));
                assert_eq!((a - b).to_bytes(), bytes(x - y));
                assert_eq!((a * b).to_bytes(), bytes(x * y));
                assert_eq!((-a).to_bytes(), bytes(-x));
                assert_eq!(a.square().to_bytes(), bytes(x.square()));
                assert_eq!(a.inverse().map(|v| v.to_bytes()), x.inverse().map(bytes));
            }};
        }
        compare!(Fq, ark_ed_on_bls12_377::Fq);
        compare!(Fr, ark_ed_on_bls12_377::Fr);
    }
}

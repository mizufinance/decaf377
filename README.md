# decaf377

[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/decaf377.svg
[crates-url]: https://crates.io/crates/decaf377

Many zero-knowledge protocols require a cryptographic group that can be used
inside of an arithmetic circuit. This is accomplished by defining an “embedded”
elliptic curve whose base field is the scalar field of the proving curve used
by the proof system.

The Zexe paper, which defined BLS12-377, also defined (but did not name) a
cofactor-4 Edwards curve defined over the BLS12-377 scalar field for exactly
this purpose. However, non-prime-order groups are a leaky abstraction, forcing
all downstream constructions to pay attention to correct handling of the
cofactor. Although it is usually possible to do so safely, it requires
additional care, and as discussed below, the optimal technique for handling the
cofactor is different inside and outside of a circuit.

Instead, applying the Decaf construction to this curve gives decaf377, a clean
abstraction that provides a prime-order group, complete with hash-to-group
functionality, and works the same way inside and outside of a circuit.

More details are available on the [Penumbra
website](https://protocol.penumbra.zone/main/crypto/decaf377.html).

## Features

* `std`: default, for use in `std` environments,
* `alloc`: default, for use in `alloc` environments,
* `arkworks`: default, uses Arkworks crates for elliptic curve operations,
* `u32_backend`: uses 32-bit finite field arithmetic (default is 64-bit),
* `r1cs`: enables rank-1 constraint system gadgets,
* `parallel`: enables the use of parallelism.

## Benchmarks

Run `criterion` benchmarks using:

```
cargo bench
```

This will generate a report at `target/criterion/report/index.html`.

## Scalar multiplication

`Element` and `AffinePoint` scalar operators use a fixed-schedule extended
Edwards implementation over the existing 32-bit Fiat field arithmetic, even
when the public types use the Arkworks backend. Raw Montgomery bridges avoid
Arkworks field reduction during conversion; affine output uses fixed-exponent
inversion. `Fr` serialization also uses the Fiat conversion. No wire encoding
or scalar-field modulus changes.

`PrimeGroup::mul_bigint`, `AffineRepr::mul_bigint`, and the minimal backend's
`scalar_mul` process every supplied limb; the limb count must be public.
Ordinary `Fr` operators process four limbs. Explicitly variable-time APIs are
for public inputs. General Arkworks field operations, batch normalization,
point compression, and other surrounding cryptographic operations are outside
this multiplication guarantee and must be reviewed separately for secret use.

This contract applies to optimized builds without debug assertions and the
usual constant-time integer-instruction assumptions. It is not a formal
compiled-code proof or a claim of physical power/EM resistance. The independent
algebraic/encoding tests and `cargo run --release --example scalar_timing`
provide functional and timing regression evidence, not a constant-time proof.

//! Diagnostic only: timing measurements are not a constant-time proof.
use decaf377::{Element, Fr};
use std::{hint::black_box, time::Instant};

fn main() {
    measure("owned multiplication", |scalar| Element::GENERATOR * scalar);
    #[cfg(feature = "arkworks")]
    measure("mutable-reference assignment", |mut scalar| {
        let mut point = Element::GENERATOR;
        point *= &mut scalar;
        point
    });
}

fn measure(label: &str, operation: impl Fn(Fr) -> Element) {
    for scalar in [Fr::ZERO, Fr::ONE, -Fr::ONE] {
        let mut samples = [0u128; 9];
        for sample in &mut samples {
            let start = Instant::now();
            for _ in 0..100 {
                black_box(operation(black_box(scalar)));
            }
            *sample = start.elapsed().as_nanos() / 100;
        }
        samples.sort_unstable();
        println!("{label}: median ns/multiplication: {}", samples[4]);
    }
}

//! Diagnostic only: timing measurements are not a constant-time proof.
use decaf377::{Element, Fr};
use std::{hint::black_box, time::Instant};

fn main() {
    for scalar in [Fr::ZERO, Fr::ONE, -Fr::ONE] {
        let mut samples = [0u128; 9];
        for sample in &mut samples {
            let start = Instant::now();
            for _ in 0..100 {
                black_box(black_box(Element::GENERATOR) * black_box(scalar));
            }
            *sample = start.elapsed().as_nanos() / 100;
        }
        samples.sort_unstable();
        println!("median ns/multiplication: {}", samples[4]);
    }
}

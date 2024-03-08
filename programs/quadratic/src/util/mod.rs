use crate::state::QuadraticCoefficients;

pub fn convert_vote(input_voter_weight: u64, coefficients: &QuadraticCoefficients) -> f64 {
    let input_voter_weight = input_voter_weight as f64;
    let a = coefficients.a;
    let b = coefficients.b;
    let c = coefficients.c;

    a * input_voter_weight.powf(0.5) + b * input_voter_weight + c
}

use anchor_lang::prelude::*;
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub struct QuadraticCoefficients {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}
impl Default for QuadraticCoefficients {
    fn default() -> Self {
        QuadraticCoefficients {
            a: 1.0,
            b: 0.0,
            c: 0.0,
        }
    }
}
impl QuadraticCoefficients {
    pub const SPACE: usize = 8 + 8 + 8;
}

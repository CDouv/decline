
// Decline conversion functions

pub fn secant_effective_to_nominal(decline_rate: f32, exponent: f32) -> f32 {
    ((1.0 - decline_rate).powf(-exponent) - 1.0) / exponent
}

pub fn nominal_to_secant_effective(decline_rate: f32, exponent: f32) -> f32 {
    1.0 - (1.0 + exponent * decline_rate).powf(-1.0 / exponent)
}

pub fn tangent_effective_to_nominal(decline_rate: f32) -> f32 {
    -(1.0 - decline_rate).ln()
}

pub fn nominal_to_tangent_effective(decline_rate: f32) -> f32 {
    1.0 - (-decline_rate).exp()
}
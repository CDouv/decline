use serde::{Deserialize, Serialize};

use crate::exponential::Exponential;
use crate::hyperbolic::Hyperbolic;

pub const E: f32 = 2.71828;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum ForecastParameter<T> {
    Known(T),
    Unknown,
}

impl ForecastParameter<f32> {
    pub fn extract_value(&self) -> f32 {
        match *self {
            ForecastParameter::Known(x) => x,
            Unknown => panic!("{:?}", *self),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DeclineInput {
    pub text: String,
    pub symbol: String,
    pub units: String,
    pub calculate: bool,
    pub input: Option<f32>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DeclineSegment {
    pub product: String,
    pub segmentNumber: i32,
    pub forecastType: String,
    pub parameters: Vec<DeclineInput>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeclineParameters {
    pub parameters: Vec<f32>,
}

pub enum DeclineType {
    Exponential(Exponential<f32>),
    Hyperbolic(Hyperbolic<f32>),
}

//Functions to extract data from incoming JSON

pub fn createExponential(input: &DeclineSegment) -> Exponential<f32> {
    //Initializing the array
    let mut input_values: [ForecastParameter<f32>; 5] = [ForecastParameter::Unknown; 5];

    for (i, item) in input.parameters.iter().enumerate() {
        let val = match &item.input {
            None => ForecastParameter::Unknown,
            Some(x) => ForecastParameter::Known(*x),
        };

        input_values[i] = val;
    }

    let decline: Exponential<f32> = Exponential {
        qi: input_values[0],
        qf: input_values[1],
        d: input_values[2],
        duration: input_values[3],
        reserves: input_values[4],
    };

    return decline;
}

pub fn createHyperbolic(input: &DeclineSegment) -> Hyperbolic<f32> {
    //Initializing the array
    let mut input_values: [ForecastParameter<f32>; 7] = [ForecastParameter::Unknown; 7];

    for (i, item) in input.parameters.iter().enumerate() {
        let val = match &item.input {
            None => ForecastParameter::Unknown,
            Some(x) => ForecastParameter::Known(*x),
        };

        input_values[i] = val;
    }

    let decline: Hyperbolic<f32> = Hyperbolic {
        qi: input_values[0],
        q: input_values[1],
        di: input_values[2],
        d: input_values[3],
        t: input_values[4],
        np: input_values[5],
        b: input_values[6],
    };

    return decline;
}

pub fn convert_inputs(input: &DeclineSegment) -> DeclineType {
    let decline: DeclineType = match &input.forecastType {
        exponential => DeclineType::Exponential(createExponential(input)),
        hyperbolic => DeclineType::Hyperbolic(createHyperbolic(input)),
        _ => panic!(),
    };

    return decline;
}

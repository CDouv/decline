use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::time::Duration;

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

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Hyperbolic<f32> {
    pub qi: ForecastParameter<f32>,
    pub q: ForecastParameter<f32>,
    pub di: ForecastParameter<f32>,
    pub d: ForecastParameter<f32>,
    pub t: ForecastParameter<f32>,
    pub np: ForecastParameter<f32>,
    pub b: ForecastParameter<f32>,
}

// Constructor function for Exponential
impl Hyperbolic<f32> {
    pub fn new() -> Self {
        let qi = input_qi();
        let q = input_q();
        let di = input_di();
        let d = input_d();
        let t = input_t();
        let np = input_np();
        let b = input_b();

        Self {
            qi: qi,
            q: q,
            di: di,
            d: d,
            t: t,
            np: np,
            b: b,
        }
    }
}

impl Hyperbolic<f32> {
    pub fn to_array(&self) -> [ForecastParameter<f32>; 7] {
        let mut arr: [ForecastParameter<f32>; 7] = [ForecastParameter::Unknown; 7];

        arr[0] = *&self.qi;
        arr[1] = *&self.q;
        arr[2] = *&self.di;
        arr[3] = *&self.d;
        arr[4] = *&self.t;
        arr[5] = *&self.np;
        arr[6] = *&self.b;

        arr
    }

    //Come back to this, may not need it
    pub fn check_unknowns(&self) -> [i32; 7] {
        let arr = &self.to_array();

        let mut knowns: [i32; 7] = [0; 7];

        for (i, parameter) in arr.iter().enumerate() {
            match parameter {
                ForecastParameter::Known(f32) => knowns[i] = 0,
                ForecastParameter::Unknown => knowns[i] = 1,
            }
        }

        knowns
    }

    pub fn extract_parameters(&self) -> DeclineParameters {
        let arr = self.to_array();
        let mut params = vec![0.0; 7];

        for (i, param) in arr.iter().enumerate() {
            params[i] = param.extract_value();
        }

        let parameters = DeclineParameters { parameters: params };

        parameters
    }
}

//Next up:
//set up functions within Hyperbolic to solve each given scenario of knowns:

//Ex:
//pub fn solve_q_np_t(mut self) -> Self {

//}

//THEN create a solve_unknowns() function that:
//Checks what the unknowns are
//Picks appropriate list of helper functions to solve unknowns
//returns Hyperbolic with all Knowns

impl Hyperbolic<f32> {
    pub fn solve_q_np_t(mut self) -> Self {}
}

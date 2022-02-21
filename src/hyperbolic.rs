use crate::inputs::DeclineParameters;
use crate::inputs::ForecastParameter;

use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::time::Duration;

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

    pub fn check_unknowns(&self) -> [i32; 7] {
        let arr = &self.to_array();

        let mut unknowns: [i32; 7] = [0; 7];

        for (i, parameter) in arr.iter().enumerate() {
            match parameter {
                ForecastParameter::Known(f32) => unknowns[i] = 0,
                ForecastParameter::Unknown => unknowns[i] = 1,
            }
        }

        unknowns
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

//1. Define set of single variable functions to solve
//2. Create functions to solve each scenario
//3. Create solve_unknowns function that picks the correct function to use depending on unknowns

//Single variable functions
impl Hyperbolic<f32> {
    pub fn solve_q(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let b = self.b.extract_value();
        let di = self.di.extract_value();
        let t = self.t.extract_value();

        let q = (qi / ((1.0 + b * di * t).powf(1.0 / b))) / 365.25;

        self.q = ForecastParameter::Known(q);

        self
    }

    pub fn solve_di_1(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let b = self.b.extract_value();
        let q = self.q.extract_value() * 365.25;
        let np = self.np.extract_value() * 1000.0;

        let di = ((qi.powf(b)) * ((q.powf(1.0 - b)) - (qi.powf(1.0 - b)))) / ((b - 1.0) * np);

        println!("di {:?}", di);
        self.di = ForecastParameter::Known(di);
        println!("self.di {:?}", self.di);
        self
    }

    pub fn solve_di_2(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let q = self.q.extract_value() * 365.25;
        let b = self.b.extract_value();
        let t = self.t.extract_value();

        let di = (((qi / q).powf(b)) - 1.0) / (t * b);

        self.di = ForecastParameter::Known(di);

        self
    }

    pub fn solve_di_3(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let q = self.q.extract_value() * 365.25;
        let b = self.b.extract_value();
        let d = self.d.extract_value();

        let di = (((qi / q).powf(b)) - 1.0) * d + d;

        self.di = ForecastParameter::Known(di);

        self
    }

    pub fn solve_d(mut self) -> Self {
        let di = self.di.extract_value();
        let b = self.b.extract_value();
        let t = self.t.extract_value();

        let d = di / (1.0 + b * di * t);

        self.d = ForecastParameter::Known(d);

        self
    }

    pub fn solve_np(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let b = self.b.extract_value();
        let di = self.di.extract_value();
        let q = self.q.extract_value() * 365.25;

        let np =
            (((qi.powf(b)) / ((b - 1.0) * di)) * (q.powf(1.0 - b) - qi.powf(1.0 - b))) / 1000.0;

        self.np = ForecastParameter::Known(np);

        self
    }

    pub fn solve_t_1(mut self) -> Self {
        let di = self.di.extract_value();
        let d = self.d.extract_value();
        let b = self.b.extract_value();

        let t = (di - d) / (b * d * di);

        self.t = ForecastParameter::Known(t);

        self
    }

    pub fn solve_t_2(mut self) -> Self {
        let qi = self.qi.extract_value() * 365.25;
        let q = self.q.extract_value() * 365.25;
        let b = self.b.extract_value();
        let di = self.di.extract_value();

        let t = (((qi / q).powf(b)) - 1.0) / (di * b);

        self.t = ForecastParameter::Known(t);

        self
    }

    pub fn solve_t_3(mut self) -> Self {
        let di = self.di.extract_value();
        let d = self.d.extract_value();
        let b = self.b.extract_value();

        let t = (di - d) / (b * d * di);

        self.t = ForecastParameter::Known(t);

        self
    }
}

//Set up functions to solve missing set of parameters
impl Hyperbolic<f32> {
    pub fn solve_q_np_t(mut self) -> Self {
        self = self.solve_t_1();
        self = self.solve_q();
        self = self.solve_np();

        self
    }

    pub fn solve_t_di_d(mut self) -> Self {
        self = self.solve_di_1();
        self = self.solve_t_2();
        self = self.solve_d();

        self
    }

    pub fn solve_np_di_d(mut self) -> Self {
        self = self.solve_di_2();
        self = self.solve_d();
        self = self.solve_np();

        self
    }

    pub fn solve_np_t_d(mut self) -> Self {
        self = self.solve_np();
        self = self.solve_t_2();
        self = self.solve_d();

        self
    }

    pub fn solve_np_t_di(mut self) -> Self {
        self = self.solve_di_3();
        self = self.solve_t_3();
        self = self.solve_np();

        self
    }

    pub fn solve_q_np_d(mut self) -> Self {
        self = self.solve_q();
        self = self.solve_np();
        self = self.solve_d();

        self
    }

    pub fn solve_unknowns(mut self) -> Self {
        self = match self {
            Self {
                q: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => self.solve_q_np_t(),
            Self {
                t: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                d: ForecastParameter::Unknown,
                ..
            } => self.solve_t_di_d(),

            Self {
                np: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                d: ForecastParameter::Unknown,
                ..
            } => self.solve_np_di_d(),
            Self {
                np: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                d: ForecastParameter::Unknown,
                ..
            } => self.solve_np_t_d(),
            Self {
                np: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => self.solve_np_t_di(),
            Self {
                q: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                d: ForecastParameter::Unknown,
                ..
            } => self.solve_q_np_d(),
            _ => panic!(),
        };

        self
    }
}

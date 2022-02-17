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
    // pub fn new() -> Self {
    //     let qi = input_qi();
    //     let q = input_q();
    //     let di = input_di();
    //     let d = input_d();
    //     let t = input_t();
    //     let np = input_np();
    //     let b = input_b();

    //     Self {
    //         qi: qi,
    //         q: q,
    //         di: di,
    //         d: d,
    //         t: t,
    //         np: np,
    //         b: b,
    //     }
    // }
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
        let qi = self.qi.extract_value();
        let b = self.b.extract_value();
        let di = self.di.extract_value();
        let t = self.t.extract_value();

        let q = (qi / ((1 + b * di * t).powf(1 / b)));

        self.q = ForecastParameter::Known(q);

        self
    }

    pub fn solve_di_1(mut self) -> Self {
        let qi = self.qi.extract_value();
        let b = self.b.extract_value();
        let q = self.q.extract_value();
        let np = self.np.extract_value();

        let di = ((qi.powf(b)) * ((q.powf(1 - b)) - (qi.powf(1 - b)))) / ((b - 1) * np);

        self.di = ForecastParameter::Known(di);

        self
    }

    pub fn solve_di_2(mut self) -> Self {
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let b = self.b.extract_value();
        let t = self.t.extract_value();

        let di = (((qi / q).powf(b)) - 1) / (t * b);

        self.di = ForecastParameter::Known(di);

        self
    }

    pub fn solve_di_3(mut self) -> Self {
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let b = self.b.extract_value();
        let d = self.d.extract_value();

        self.di = ForecastParameter::Known(di);

        self
    }

    pub fn solve_d(mut self) -> Self {
        let di = self.di.extract_value();
        let b = self.b.extract_value();
        let t = self.t.extract_value();

        let d = di / (1 + b * di * t);

        self.d = ForecastParameter::Known(d);

        self
    }

    pub fn solve_np(mut self) -> Self {
        let qi = self.qi.extract_value();
        let b = self.b.extract_value();
        let di = self.di.extract_value();
        let q = self.q.extract_value();

        let np = (((qi.powf(b)) / ((b - 1) * di)) * (q.powf(1 - b) - qi.powf(1 - b)));

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
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let b = self.b.extract_value();
        let di = self.di.extract_value();

        let t = (((qi / q).powf(b)) - 1) / (di * b);

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
        self.solve_t_1();
        self.solve_q();
        self.solve_np();

        self
    }

    pub fn solve_t_di_d(mut self) -> Self {
        self.solve_di_1();
        self.solve_t_2();
        self.solve_d();

        self
    }

    pub fn solve_np_di_d(mut self) -> Self {
        self.solve_di_2();
        self.solve_d();
        self.solve_np();

        self
    }

    pub fn solve_np_t_d(mut self) -> Self {
        self.solve_np();
        self.solve_t_2();
        self.solve_d();

        self
    }

    pub fn solve_np_t_di(mut self) -> Self {
        self.solve_di_3();
        self.solve_t_3();
        self.solve_np();

        self
    }

    pub fn solve_q_np_d(mut self) -> Self {
        self.solve_q();
        self.solve_np();
        self.solve_d();

        self
    }

    pub fn solve_unknowns(mut self) -> Self {
        match self.check_unknowns() {
            [0, 1, 0, 0, 1, 1, 0] => self.solve_q_np_t(),
            [0, 0, 1, 1, 1, 0, 0] => self.solve_t_di_d(),
            [0, 0, 1, 1, 0, 1, 0] => self.solve_np_di_d(),
            [0, 0, 0, 1, 1, 1, 0] => self.solve_np_t_d(),
            [0, 0, 1, 0, 1, 1, 0] => self.solve_np_t_di(),
            [0, 1, 0, 1, 0, 1, 0] => self.solve_q_np_d(),
            _ => panic(),
        }
    }
}

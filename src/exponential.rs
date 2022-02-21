use serde::{Deserialize, Serialize};

use crate::inputs::DeclineParameters;
use crate::inputs::ForecastParameter;
pub const E: f32 = 2.71828;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Exponential<f32> {
    pub qi: ForecastParameter<f32>,
    pub q: ForecastParameter<f32>,
    pub di: ForecastParameter<f32>,
    pub d: ForecastParameter<f32>,
    pub t: ForecastParameter<f32>,
    pub np: ForecastParameter<f32>,
    pub b: ForecastParameter<f32>,
}

impl Exponential<f32> {
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

    // pub fn check_unknowns(&self) -> [i32; 5] {
    //     let arr = &self.to_array();

    //     let mut knowns: [i32; 5] = [0; 5];

    //     for (i, parameter) in arr.iter().enumerate() {
    //         // println!("check array");
    //         // println!("{:?}",arr);
    //         match parameter {
    //             ForecastParameter::Known(f32) => knowns[i] = 0,
    //             ForecastParameter::Unknown => knowns[i] = 1,
    //         }
    //     }

    //     knowns
    // }

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

//Solving for single variable equations

impl Exponential<f32> {
    //Solve for qi
    pub fn solve_qi(mut self) -> Self {
        let q = self.q.extract_value();
        let di = self.di.extract_value();
        let t = self.t.extract_value();

        let qi = q * E.powf(di * t);

        self.qi = ForecastParameter::Known(qi);

        self
    }

    //Solve for q
    pub fn solve_q(mut self) -> Self {
        let qi = self.qi.extract_value();

        let di = self.di.extract_value();

        let t = self.t.extract_value();

        let q = qi / (E.powf(di * t));

        self.q = ForecastParameter::Known(q);

        self
    }

    //Solve for decline rate
    pub fn solve_di(mut self) -> Self {
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let t = self.t.extract_value();

        let di = -((q / qi).ln() / t);

        self.di = ForecastParameter::Known(di);

        self
    }

    //Solve for t
    pub fn solve_t(mut self) -> Self {
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let di = self.di.extract_value();

        let t = -((q / qi).ln() / di);
        self.t = ForecastParameter::Known(t);

        self
    }

    //Solve for np
    pub fn solve_np(mut self) -> Self {
        let qi = self.qi.extract_value();
        let q = self.q.extract_value();
        let di = self.di.extract_value();

        let np = (qi - q) / di;
        self.np = ForecastParameter::Known(np);

        self
    }
}
//Substitutation equations used for bisection
impl Exponential<f32> {
    pub fn missing_qi_q(&self, q_guess: f32) -> f32 {
        let di = &self.di.extract_value();
        let t = &self.t.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with qf as only unknown
        let result = q_guess * 365.25 * E.powf(di * t) - np * 1000.0 * di - q_guess * 365.25;

        result
    }

    pub fn missing_qi_di(&self, di_guess: f32) -> f32 {
        let q = &self.q.extract_value();
        let t = &self.t.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with decline as the only unknown
        let result = di_guess * np * 1000.0 - q * E.powf(di_guess * t) * 365.25 + q * 365.25;

        result
    }

    pub fn missing_qi_t(&self, t_guess: f32) -> f32 {
        let q = &self.q.extract_value();
        let di = &self.di.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = di * t_guess + ((q * 365.25) / (np * 1000.0 * di + q * 365.25)).ln();

        result
    }

    pub fn missing_qi_np(&self, np_guess: f32) -> f32 {
        let q = &self.q.extract_value();
        let di = &self.di.extract_value();
        let t = &self.t.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = np_guess * 1000.0 * di - q * 365.25 * E.powf(di * t) + q * 365.25;

        result
    }

    pub fn missing_q_di(&self, di_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let t = &self.t.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = di_guess * np * 1000.0 - qi * 365.25 + qi * 365.25 * E.powf(-di_guess * t);

        println!("This is the result");
        result
    }

    pub fn missing_q_t(&self, t_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let di = &self.di.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = di * t_guess + (qi * 365.25 * E.powf(di * t_guess) / (qi * 365.25));

        result
    }

    pub fn missing_q_np(&self, np_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let di = &self.di.extract_value();
        let t = &self.t.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = di * np_guess * 1000.0 - qi * 365.25 + qi * 365.25 * E.powf(di * t);

        result
    }

    pub fn missing_di_t(&self, t_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let q = &self.q.extract_value();
        let np = &self.np.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = 365.25 * (qi - q) * t_guess + (q / qi).ln() * np * 1000.0;

        result
    }

    pub fn missing_di_np(&self, np_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let q = &self.q.extract_value();
        let t = &self.t.extract_value();

        //setting equation = 0 with t as the only unknown
        let result = np_guess * 1000.0 * ((q / qi).ln()) + t * (qi - q) * 365.25;

        result
    }
}

//Function used to solve unknowns for a given Exponential struct

impl Exponential<f32> {
    pub fn bisection(&self, bounds: (f32, f32)) -> f32 {
        //Use match against self to determine which substitution function to use
        let f = match &self {
            // Scenario 1 -Missing initial_rate and final_rate
            Self {
                qi: ForecastParameter::Unknown,
                q: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_qi_q,
            // Scenario 2 -Missing initial_rate and decline_rate
            Self {
                qi: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_qi_di,
            // //Scenario 3 - Missing initial_rate and t
            Self {
                qi: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_qi_t,
            // //Scenario 4 - Missing initial_rate and np
            Self {
                qi: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_qi_np,
            // //Scenario 5 - Missing final_rate and decline_rate
            Self {
                q: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_q_di,
            // //Scenario 6 - Missing final_rate and t
            Self {
                q: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_q_t,
            // //Scenario 7 - Missing final_rate and np
            Self {
                q: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_q_np,
            // //Scenario 8 - Missing decline_rate and t
            Self {
                di: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_di_t,
            // //Scenario 9 - issing decline_rate and np
            Self {
                di: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => Exponential::missing_di_np,
            // //Scenario 10 - Missing t and np
            Self {
                t: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => panic!("Missing t and np function not implemented yet"),
            _ => panic!(),
        };

        let mut a = bounds.0;
        let mut b = bounds.1;

        let mut c: f32 = ((a + b) / 2.0).abs();
        let mut iteration = 1;

        'outer: while ((f(self, a) - f(self, c)) / f(self, a)).abs() > 0.01
            && ((f(self, b) - f(self, c)) / f(self, b)).abs() > 0.01
        {
            println!("Start of iteration # {}", iteration);
            println!("\na {}\n b {} \n c {}", a, b, c);
            let mut res_a = f(self, a);
            let mut res_b = f(self, b);
            let mut res_c = f(self, c);

            println!("\nf(a) {}\n f(b) {} \n f(c) {}", res_a, res_b, res_c);
            println!("\n\n");

            if f(self, c) == 0.0 {
                break 'outer;
            }
            match (f(self, a) * f(self, c) < 0.0, f(self, b) * f(self, c) < 0.0) {
                (true, true) => match (f(self, a) - f(self, c)) < (f(self, b) - f(self, c)) {
                    true => b = c,
                    false => a = c,
                },

                //root is between b and c (a becomes c)
                (false, true) => a = c,

                // root is between a and c (b becomes c)
                (true, false) => b = c,

                (false, false) => {
                    if f(self, c) == 0.0 {
                        break;
                    } else {
                        panic!(
                            "\na {}\n b {} \n c {}\n f(a) {}\nf(b) {}\n f(c) {}\n",
                            a,
                            b,
                            c,
                            f(self, a),
                            f(self, b),
                            f(self, c)
                        )
                    }
                }
            }

            c = ((a + b) / 2.0).abs();

            iteration += 1;
        }

        c
    }
    pub fn solve_unknowns(mut self) -> Self {
        //Determine what bounds to use for bisection equation
        let bounds: (f32, f32) = match self {
            // Scenario 1 -Set bounds for qf
            Self {
                qi: ForecastParameter::Unknown,
                q: ForecastParameter::Unknown,
                ..
            } => (0.0, 10000.0),
            // Scenario 2 -Set bounds for decline
            Self {
                qi: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => (0.01, 0.99),
            // //Scenario 3 - Set bounds for t
            Self {
                qi: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => (0.0, 100.0),
            // //Scenario 4 - Set bounds for np
            Self {
                qi: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => (0.0, 100000.0),
            // //Scenario 5 - Set bounds for decline
            Self {
                q: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => (0.01, 0.99),
            // //Scenario 6 - Set bounds for t
            Self {
                q: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => (0.0, 100.0),
            // //Scenario 7 - Set bounds for np
            Self {
                q: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => (0.0, 100000.0),
            // //Scenario 8 - Set bounds for t
            Self {
                di: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => (0.0, 100.0),
            // //Scenario 9 - Set bounds for np
            Self {
                di: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => (0.0, 100000.0),
            _ => panic!(),
        };

        //Solving unknown 1 using bisection
        match self {
            // Scenario 1 - Missing qi, q -> Solve for q
            Self {
                qi: ForecastParameter::Unknown,
                q: ForecastParameter::Unknown,
                ..
            } => self.q = ForecastParameter::Known(self.bisection(bounds)),
            // Scenario 2 -Missing qi, did -> solve for di
            Self {
                qi: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => self.di = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 3 - Missing qi,t  -> solve for t
            Self {
                qi: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => self.t = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 4 - Missing qi, np -> solve for np
            Self {
                qi: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => self.np = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 5 - Missing q, di -> solve for di
            Self {
                q: ForecastParameter::Unknown,
                di: ForecastParameter::Unknown,
                ..
            } => {
                println!("test");
                self.di = ForecastParameter::Known(self.bisection(bounds));
                println!("After bisection {:?}", self);
            }

            // //Scenario 6 - Missing q, t -> solve for t
            Self {
                q: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => self.t = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 7 - Missing q, np -> solve for np
            Self {
                q: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => self.np = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 8 - Missing di, t -> solve for t
            Self {
                di: ForecastParameter::Unknown,
                t: ForecastParameter::Unknown,
                ..
            } => self.t = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 9 - Missing di, np -> solve for np
            Self {
                di: ForecastParameter::Unknown,
                np: ForecastParameter::Unknown,
                ..
            } => self.np = ForecastParameter::Known(self.bisection(bounds)),
            _ => panic!(),
        };
        //Solving unknown 2 using single unknown equations
        self = match self {
            // Scenario 1 -Solve qi
            Self {
                qi: ForecastParameter::Unknown,
                ..
            } => self.solve_qi(),
            // Scenario 2 -Solve q
            Self {
                q: ForecastParameter::Unknown,
                ..
            } => self.solve_q(),
            // //Scenario 3 - Solve decline
            Self {
                di: ForecastParameter::Unknown,
                ..
            } => self.solve_di(),
            // //Scenario 4 - Solve t
            Self {
                t: ForecastParameter::Unknown,
                ..
            } => self.solve_t(),
            // //Scenario 5 - Solve np
            Self {
                np: ForecastParameter::Unknown,
                ..
            } => self.solve_np(),
            _ => panic!("{:?}", "Error solving unknown #2"),
        };

        //set d = di and b = 0

        self.d = ForecastParameter::Known(self.di.extract_value());
        self.b = ForecastParameter::Known(0.0);

        self
    }
}

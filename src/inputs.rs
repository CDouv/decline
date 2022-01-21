use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::time::Duration;

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

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Exponential<f32> {
    pub qi: ForecastParameter<f32>,
    pub qf: ForecastParameter<f32>,
    pub d: ForecastParameter<f32>,
    pub duration: ForecastParameter<f32>,
    pub reserves: ForecastParameter<f32>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ExponentialInput {
    pub text: String,
    pub symbol: String,
    pub units: String,
    pub calculate: bool,
    pub input: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeclineParameters {
    pub parameters: Vec<f32>,
}

// Constructor function for Exponential
impl Exponential<f32> {
    pub fn new() -> Self {
        let qi = input_initial_rate();
        let qf = input_final_rate();
        let d = input_decline_rate();
        let duration = input_duration();
        let reserves = input_reserves();

        Self {
            qi: qi,
            qf: qf,
            d: d,
            duration: duration,
            reserves: reserves,
        }
    }
}

impl Exponential<f32> {
    pub fn to_array(&self) -> [ForecastParameter<f32>; 5] {
        let mut arr: [ForecastParameter<f32>; 5] = [ForecastParameter::Unknown; 5];

        arr[0] = *&self.qi;
        arr[1] = *&self.qf;
        arr[2] = *&self.d;
        arr[3] = *&self.duration;
        arr[4] = *&self.reserves;

        arr
    }

    pub fn check_unknowns(&self) -> [i32; 5] {
        let arr = &self.to_array();

        let mut knowns: [i32; 5] = [0; 5];

        for (i, parameter) in arr.iter().enumerate() {
            // println!("check array");
            // println!("{:?}",arr);
            match parameter {
                ForecastParameter::Known(f32) => knowns[i] = 0,
                ForecastParameter::Unknown => knowns[i] = 1,
            }
        }

        knowns
    }

    pub fn extract_parameters(&self) -> DeclineParameters {
        let arr = self.to_array();
        let mut params = vec![0.0; 5];

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
        let qf = self.qf.extract_value();
        let d = self.d.extract_value();
        let duration = self.duration.extract_value();

        let qi = qf * E.powf(d * duration);

        self.qi = ForecastParameter::Known(qi);

        self
    }

    //Solve for qf
    pub fn solve_qf(mut self) -> Self {
        let qi = self.qi.extract_value();
        let d = self.d.extract_value();
        let duration = self.duration.extract_value();

        let qf = qi / (E.powf(d * duration));

        self.qf = ForecastParameter::Known(qf);

        self
    }

    //Solve for decline rate
    pub fn solve_decline(mut self) -> Self {
        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let duration = self.duration.extract_value();

        let d = -((qf / qi).ln() / duration);
        self.d = ForecastParameter::Known(d);

        self
    }

    //Solve for duration
    pub fn solve_duration(mut self) -> Self {
        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let d = self.d.extract_value();

        let duration = -((qf / qi).ln() / d);
        self.duration = ForecastParameter::Known(duration);

        self
    }

    //Solve for reserves
    pub fn solve_reserves(mut self) -> Self {
        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let d = self.d.extract_value();

        let reserves = (qi - qf) / d;
        self.reserves = ForecastParameter::Known(reserves);

        self
    }
}
//Substitutation equations used for bisection
impl Exponential<f32> {
    pub fn missing_qi_qf(&self, qf_guess: f32) -> f32 {
        let d = &self.d.extract_value();
        let duration = &self.duration.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with qf as only unknown
        let result =
            qf_guess * 365.25 * E.powf(d * duration) - reserves * 1000.0 * d - qf_guess * 365.25;

        result
    }

    pub fn missing_qi_d(&self, d_guess: f32) -> f32 {
        let qf = &self.qf.extract_value();
        let duration = &self.duration.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with decline as the only unknown
        let result =
            d_guess * reserves * 1000.0 - qf * E.powf(d_guess * duration) * 365.25 + qf * 365.25;

        result
    }

    pub fn missing_qi_duration(&self, duration_guess: f32) -> f32 {
        let qf = &self.qf.extract_value();
        let d = &self.d.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result =
            d * duration_guess + ((qf * 365.25) / (reserves * 1000.0 * d + qf * 365.25)).ln();

        result
    }

    pub fn missing_qi_reserves(&self, reserves_guess: f32) -> f32 {
        let qf = &self.qf.extract_value();
        let d = &self.d.extract_value();
        let duration = &self.duration.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result = reserves_guess * 1000.0 * d - qf * 365.25 * E.powf(d * duration) + qf * 365.25;

        result
    }

    pub fn missing_qf_d(&self, d_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let duration = &self.duration.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result =
            d_guess * reserves * 1000.0 - qi * 365.25 + qi * 365.25 * E.powf(-d_guess * duration);

        result
    }

    pub fn missing_qf_duration(&self, duration_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let d = &self.d.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result =
            d * duration_guess + (qi * 365.25 * E.powf(d * duration_guess) / (qi * 365.25));

        result
    }

    pub fn missing_qf_reserves(&self, reserves_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let d = &self.d.extract_value();
        let duration = &self.duration.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result = d * reserves_guess * 1000.0 - qi * 365.25 + qi * 365.25 * E.powf(d * duration);

        result
    }

    pub fn missing_d_duration(&self, duration_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let qf = &self.qf.extract_value();
        let reserves = &self.reserves.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result = 365.25 * (qi - qf) * duration_guess + (qf / qi).ln() * reserves * 1000.0;

        result
    }

    pub fn missing_d_reserves(&self, reserves_guess: f32) -> f32 {
        let qi = &self.qi.extract_value();
        let qf = &self.qf.extract_value();
        let duration = &self.duration.extract_value();

        //setting equation = 0 with duration as the only unknown
        let result = reserves_guess * 1000.0 * ((qf / qi).ln()) + duration * (qi - qf) * 365.25;

        result
    }
}

//Five functions for inputting parameters qi,qf,d,duration,reserves
//Initial Rate
pub fn input_initial_rate() -> ForecastParameter<f32> {
    let mut line = String::new();
    println!("Enter initial rate (mcf/d or bbl/d):");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let initial_rate = if line.trim().is_empty() {
        ForecastParameter::Unknown
    } else {
        // parse the values
        let x: f32 = line.trim().parse().unwrap();
        ForecastParameter::Known(x)
    };

    initial_rate
}
// Final Rate
pub fn input_final_rate() -> ForecastParameter<f32> {
    let mut line = String::new();
    println!("Enter final rate (mcf/d or bbl/d):");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let final_rate = if line.trim().is_empty() {
        ForecastParameter::Unknown
    } else {
        // parse the values
        let x: f32 = line.trim().parse().unwrap();
        ForecastParameter::Known(x)
    };
    final_rate
}

// Decline Rate
pub fn input_decline_rate() -> ForecastParameter<f32> {
    let mut line = String::new();
    println!("Enter decline rate (fraction %/year):");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();

    let decline_rate = if line.trim().is_empty() {
        ForecastParameter::Unknown
    } else {
        // parse the values
        let x: f32 = line.trim().parse().unwrap();
        ForecastParameter::Known(x)
    };

    decline_rate
}
// Duration
pub fn input_duration() -> ForecastParameter<f32> {
    let mut line = String::new();
    println!("Enter segment duration (years):");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let incremental_duration = if line.trim().is_empty() {
        ForecastParameter::Unknown
    } else {
        // parse the values
        let x: f32 = line.trim().parse().unwrap();
        ForecastParameter::Known(x)
    };

    incremental_duration
}

// Duration
pub fn input_reserves() -> ForecastParameter<f32> {
    let mut line = String::new();
    println!("Enter segment reserves (mbbl or mmcf):");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let incremental_reserves = if line.trim().is_empty() {
        ForecastParameter::Unknown
    } else {
        // parse the values
        let x: f32 = line.trim().parse().unwrap();
        ForecastParameter::Known(x)
    };

    incremental_reserves
}

//Function used to solve unknowns for a given Exponential struct

impl Exponential<f32> {
    pub fn bisection(&self, bounds: (f32, f32)) -> f32 {
        //Use match against check_unknowns to determine which substitution function to use
        let f = match self.check_unknowns() {
            // Scenario 1 -Missing initial_rate and final_rate
            [1, 1, 0, 0, 0] => Exponential::missing_qi_qf,
            // Scenario 2 -Missing initial_rate and decline_rate
            [1, 0, 1, 0, 0] => Exponential::missing_qi_d,
            // //Scenario 3 - Missing initial_rate and duration
            [1, 0, 0, 1, 0] => Exponential::missing_qi_duration,
            // //Scenario 4 - Missing initial_rate and reserves
            [1, 0, 0, 0, 1] => Exponential::missing_qi_reserves,
            // //Scenario 5 - Missing final_rate and decline_rate
            [0, 1, 1, 0, 0] => Exponential::missing_qf_d,
            // //Scenario 6 - Missing final_rate and duration
            [0, 1, 0, 1, 0] => Exponential::missing_qf_duration,
            // //Scenario 7 - Missing final_rate and reserves
            [0, 1, 0, 0, 1] => Exponential::missing_qf_reserves,
            // //Scenario 8 - Missing decline_rate and duration
            [0, 0, 1, 1, 0] => Exponential::missing_d_duration,
            // //Scenario 9 - issing decline_rate and reserves
            [0, 0, 1, 0, 1] => Exponential::missing_d_reserves,
            // //Scenario 10 - Missing duration and reserves
            [0, 0, 0, 1, 1] => panic!("Missing duration and reserves function not implemented yet"),
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
        let bounds: (f32, f32) = match self.check_unknowns() {
            // Scenario 1 -Set bounds for qf
            [1, 1, 0, 0, 0] => (0.0, 10000.0),
            // Scenario 2 -Set bounds for decline
            [1, 0, 1, 0, 0] => (0.01, 0.99),
            // //Scenario 3 - Set bounds for duration
            [1, 0, 0, 1, 0] => (0.0, 100.0),
            // //Scenario 4 - Set bounds for reserves
            [1, 0, 0, 0, 1] => (0.0, 100000.0),
            // //Scenario 5 - Set bounds for decline
            [0, 1, 1, 0, 0] => (0.01, 0.99),
            // //Scenario 6 - Set bounds for duration
            [0, 1, 0, 1, 0] => (0.0, 100.0),
            // //Scenario 7 - Set bounds for reserves
            [0, 1, 0, 0, 1] => (0.0, 100000.0),
            // //Scenario 8 - Set bounds for duration
            [0, 0, 1, 1, 0] => (0.0, 100.0),
            // //Scenario 9 - Set bounds for reserves
            [0, 0, 1, 0, 1] => (0.0, 100000.0),
            _ => panic!(),
        };

        //Solving unknown 1 using bisection
        match self.check_unknowns() {
            // Scenario 1 -Set bounds for qf
            [1, 1, 0, 0, 0] => self.qf = ForecastParameter::Known(self.bisection(bounds)),
            // Scenario 2 -Set bounds for decline
            [1, 0, 1, 0, 0] => self.d = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 3 - Set bounds for duration
            [1, 0, 0, 1, 0] => self.duration = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 4 - Set bounds for reserves
            [1, 0, 0, 0, 1] => self.reserves = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 5 - Set bounds for decline
            [0, 1, 1, 0, 0] => self.d = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 6 - Set bounds for duration
            [0, 1, 0, 1, 0] => self.duration = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 7 - Set bounds for reserves
            [0, 1, 0, 0, 1] => self.reserves = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 8 - Set bounds for duration
            [0, 0, 1, 1, 0] => self.duration = ForecastParameter::Known(self.bisection(bounds)),
            // //Scenario 9 - Set bounds for reserves
            [0, 0, 1, 0, 1] => self.reserves = ForecastParameter::Known(self.bisection(bounds)),
            _ => panic!(),
        };
        //Solving unknown 2 using single unknown equations
        self = match self.check_unknowns() {
            // Scenario 1 -Solve qi
            [1, 0, 0, 0, 0] => self.solve_qi(),
            // Scenario 2 -Solve qf
            [0, 1, 0, 0, 0] => self.solve_qf(),
            // //Scenario 3 - Solve decline
            [0, 0, 1, 0, 0] => self.solve_decline(),
            // //Scenario 4 - Solve duration
            [0, 0, 0, 1, 0] => self.solve_duration(),
            // //Scenario 5 - Solve reserves
            [0, 0, 0, 0, 1] => self.solve_reserves(),
            _ => panic!("{:?}", self.check_unknowns()),
        };

        self
    }
}

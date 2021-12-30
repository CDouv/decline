use std::time::Duration;
use serde::{Serialize, Deserialize};
use serde_json::from_value;

pub const E: f32 = 2.718;

#[derive(Debug,Copy,Clone)]
pub enum ForecastParameter<T> {
    Known(T),
    Unknown,
}

impl ForecastParameter<f32> {
    pub fn extract_value(&self) -> f32 {
        match *self {
            ForecastParameter::Known(x) => x,
            Unknown => panic!(),
        }
    }
}


#[derive(Debug,Copy,Clone)]
pub struct Exponential<f32> {
    pub qi: ForecastParameter<f32>,
    pub qf: ForecastParameter<f32>,
    pub d:ForecastParameter<f32>,
    pub duration: ForecastParameter<f32>,
    pub reserves:ForecastParameter<f32>,

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
            qi:qi,
            qf:qf,
            d:d,
            duration:duration,
            reserves:reserves,
        }
    }   
}


impl Exponential<f32> {

    pub fn to_array(&self) -> [ForecastParameter<f32>;5] {
        let mut arr :[ForecastParameter<f32>;5] = [ForecastParameter::Unknown;5];

        arr[0] = *&self.qi;
        arr[1] = *&self.qf;
        arr[2] = *&self.d;
        arr[3] = *&self.duration;
        arr[4] = *&self.reserves;

        arr
    }
        
    
    pub fn check_unknowns(&self) -> [i32;5] {

        let arr = &self.to_array();

        let mut knowns: [i32;5] = [0;5];
        

        for (i, parameter) in arr.iter().enumerate() {
            // println!("check array");
            // println!("{:?}",arr);
            match parameter {
                ForecastParameter::Known(f32) => knowns[i] = 0,
                ForecastParameter::Unknown =>  knowns[i] = 1
                }
            }

            knowns
        }

    pub fn print_parameters(&self) -> () {
        println!("Decline parameters:\n");
        let arr = self.to_array();

        for (i,param) in arr.iter().enumerate() {
            
            match i {
                0 =>print!("Initial rate:") ,
                1 =>print!("Final rate:") ,
                2 =>print!("Decline rate:"),
                3 =>print!("Duration:") ,
                4 =>print!("Reserves:"),
                _ => panic!() 
            }
            
            match param {
                ForecastParameter::Known(x) => println!("{:?}",x),
                ForecastParameter::Unknown => println!("Unknown value")
            }
        }


    }
}


//Solving for single variable equations

impl Exponential<f32> {
    //Solve for qi
    pub fn solve_qi(mut self) -> Self {

        let qf = self.qf.extract_value();
        let d = self.d.extract_value();
        let duration = self.duration.extract_value();


        let qi = qf*E.powf(d*duration);

        self.qi = ForecastParameter::Known(qi);

        self

}

    //Solve for qf
    pub fn solve_qf(mut self: &mut Exponential<f32>) {

        let qi = self.qi.extract_value();
        let d = self.d.extract_value();
        let duration = self.duration.extract_value();

        let qf = qi/(E.powf(d*duration));

        self.qf = ForecastParameter::Known(qf);
    }

    //Solve for decline rate
    pub fn solve_decline(mut self: &mut Exponential<f32>) {

        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let duration = self.duration.extract_value();

        let d = -((qf/qi).ln()/duration);
        self.d = ForecastParameter::Known(d);
    }

    //Solve for duration
    pub fn solve_duration(mut self: &mut Exponential<f32>) {

        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let d = self.d.extract_value();

        let duration = -((qf/qi).ln()/d);
        self.duration = ForecastParameter::Known(duration);
    }

    //Solve for reserves
    pub fn solve_reserves(mut self: &mut Exponential<f32>) {

        let qi = self.qi.extract_value();
        let qf = self.qf.extract_value();
        let d = self.d.extract_value();

        let reserves = (qi-qf)/d;
        self.reserves = ForecastParameter::Known(reserves);

    }

}
//Substitutation equations used for bisection
impl Exponential<f32> {
        pub fn missing_qi_d(&self,d_guess:f32) -> f32 {
            let qf = &self.qf.extract_value();
            let duration = &self.duration.extract_value();
            let reserves = &self.reserves.extract_value();
            
            //setting equation = 0 with decline as the only unknown
            let result = d_guess * reserves*1000.0 - qf*E.powf(d_guess*duration)*365.0 + qf*365.0;

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

// Write a function to check which values are unknown




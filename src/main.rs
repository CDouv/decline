use std::array;
mod inputs;
use crate::inputs::ForecastParameter;




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

// Write a function to check which values are unknown

fn check_unknowns(mut arr: [ForecastParameter<f32>;5]) -> [i32;5] {

    let mut knowns: [i32;5] = [0;5];
    

    for (i, parameter) in arr.iter().enumerate() {
        
        
        match parameter {
            ForecastParameter::Known(T) => continue,
            ForecastParameter::Unknown =>  knowns[i] = 1
            }
        }

        return knowns
    }




fn main() {

    let mut inputs_check:bool = false;

    //Prepopulate an array to hold the parameters

    let mut parameters: [ForecastParameter<f32>;5] = [ForecastParameter::Unknown; 5];

    //Handing User Input
    while inputs_check == false {
    let parameters = inputs::input_initial_rate(parameters);
    let parameters = inputs::input_final_rate(parameters);
    let parameters = inputs::input_decline_rate(parameters);
    let parameters = inputs::input_duration(parameters);
    let parameters = inputs::input_reserves(parameters);


    println!("{:?}",parameters);

    //Check if unknowns > 2

    let unknowns_sum: i32 = check_unknowns(parameters).iter().sum();

    if unknowns_sum < 2 {
        println!("Not enough unknowns.")
    } else if unknowns_sum > 2 {
        println!("Too many unknowns.")
    } else {
        inputs_check = true;
    }

    }
}

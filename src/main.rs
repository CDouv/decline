
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

//Next steps: 
//Write out process of solving for one unknown for Exponential decline equations

#[derive(Debug,Copy,Clone)]
pub enum ForecastParameter<T> {
    Known(T),
    Unknown,
}

// #[derive(Debug)]
// pub struct Exponential {
//         initial_rate: ForecastParameter<f32>,
//         final_rate: ForecastParameter<f32>,
//         decline_rate: ForecastParameter<f32>,
//         incremental_duration: ForecastParameter<f32>,
//         incremental_reserves: ForecastParameter<f32>,  

//     }






fn main() {


//Prepopulate an array to hold the parameters

let mut parameters: [ForecastParameter<f32>;5] = [ForecastParameter::Unknown; 5];

//Handing User Input

//Initial Rate
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

println!("Initial rate is {:?}",initial_rate);

parameters[0] = initial_rate;

//Final Rate
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

println!("Final rate is {:?}",final_rate);

parameters[1] = final_rate;

//Decline Rate
let mut line = String::new();
println!("Enter decline rate (fraction/year):");

let b1 = std::io::stdin().read_line(&mut line).unwrap();

let decline_rate = if line.trim().is_empty() {
    ForecastParameter::Unknown
} else {
    // parse the values
    let x: f32 = line.trim().parse().unwrap();
    ForecastParameter::Known(x)
};

println!("Decline rate is {:?}",decline_rate);

parameters[2] = decline_rate;

//Incremental Duration
let mut line = String::new();
println!("Enter duration (years):");

let b1 = std::io::stdin().read_line(&mut line).unwrap();

let decline_rate = if line.trim().is_empty() {
    ForecastParameter::Unknown
} else {
    // parse the values
    let x: f32 = line.trim().parse().unwrap();
    ForecastParameter::Known(x)
};

println!("Decline rate is {:?}",decline_rate);

parameters[2] = decline_rate;

// Prepopulate data for the time being

// let initial_rate:ForecastParameter<f32> = ForecastParameter::Known(1000.0);
// let final_rate:ForecastParameter<f32> = ForecastParameter::Unknown;
// let decline_rate:ForecastParameter<f32> = ForecastParameter::Known(0.30);
// let incremental_duration:ForecastParameter<f32> = ForecastParameter::Unknown;
// let incremental_reserves:ForecastParameter<f32> = ForecastParameter::Known(1000.0);





// // Pushing values onto the array

// parameters[0] = initial_rate;
// parameters[1] = final_rate;
// parameters[2] = decline_rate;
// parameters[3] = incremental_duration;
// parameters[4] = incremental_reserves;

// println!("{:?}",parameters);

// // Creating "result" array of Knowns/Unknowns

// let mut knowns: [bool;5] = [false; 5];

// // Loop through parameters, determine knowns and unknowns
// for (i,parameter) in parameters.iter().enumerate() {
//     match parameter {
//         ForecastParameter::Known(T) => knowns[i] = true,
//         ForecastParameter::Unknown => knowns[i] = false,
//     }
// }

// println!("{:?}",knowns);
// // Check if number of unknowns > 2 --> return error message


// // match knowns/unknowns to determine functions to use
// match knowns {
// // Unknown initial_rate and final_rate
// [false, false, true, true, true] => {
//     parameters[0] = ForecastParameter::Known(100f32 / 3f32);
// }
// // Unknown initial_rate and decline_rate
// [false, true, false, true, true] => ,
// // Unknown initial_rate and incremental_duration
// [false, true, true, false, true] => ,
// // Unknown initial_rate and incremental_reserves
// [false, true, true, true, false] => ,
// // Unknown final_rate and decline_rate
// [true, false, false, true, true] => ,
// // Unknown final_rate and incremental_duration
// [true, false, true, false, true] => ,
// // Unknown final_rate and incremental_reserves
// [true, false, true, true, false] => ,
// // Unknown decline_rate and incremental_duration
// [true, true, false, false, true] => ,
// // Unknown decline_rate and incremental_reserves
// [true, true, false, true false] => ,
// // Unknown incremental_duration and incremental_reserves
// [true, true, true, false, false] => ,
// }
}



// //Check array for missing values


// //Return knowns an unknowns
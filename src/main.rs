use std::array;
mod inputs;
mod decline_calc;
use crate::inputs::ForecastParameter;
use crate::inputs::Exponential;








fn main() {


    // let mut inputs_check:bool = false;
    // let mut inputs = [ForecastParameter::Unknown;5];
    // let mut unknowns = [0;5];

  //TODO need to add back in while loop and get around instantiating the struct before the loop


    let mut inputs_check:bool = false;
    let mut unknowns = [0;5];



    //Handing User Input
    let decline_curve = Exponential::new();
    
    decline_curve.print_parameters();



//     //Check if unknowns == 2

     unknowns = decline_curve.check_unknowns();

     let unknowns_sum: i32 = unknowns.iter().sum();

     if unknowns_sum == 2 {
        inputs_check = true;
     }
     
     println!("\nThere are {} knowns and {} unknowns. Please enter 3 knowns and 2 unknowns",
            5-unknowns_sum,unknowns_sum);

}

    
// }
//     // We now know that the array has exactly 2 unknowns and 3 knowns
//     //There's probably a better way of doing this, but going to match all 10 possible scenarios for now
//     let outputs = match unknowns {
//         //Scenario 1 -Missing initial_rate and final_rate
//         [1,1,0,0,0] => decline_calc::missing_qi_qf(inputs),
//         // Scenario 2 -Missing initial_rate and decline_rate
//         // [1,0,1,0,0]=> println!("false"),
//         // //Scenario 3 - Missing initial_rate and duration
//         // [1,0,0,1,0]=> println!("false"),
//         // //Scenario 4 - Missing initial_rate and reserves
//         // [1,0,0,0,1]=> println!("false"),
//         // //Scenario 5 - Missing final_rate and decline_rate
//         // [0,1,1,0,0]=> println!("false"),
//         // //Scenario 6 - Missing final_rate and duration
//         // [0,1,0,1,0]=> println!("false"),
//         // //Scenario 7 - Missing final_rate and reserves
//         // [0,1,0,0,1]=> println!("false"),
//         // //Scenario 8 - Missing decline_rate and duration
//         // [0,0,1,1,0]=> println!("false"),
//         // //Scenario 9 - issing decline_rate and reserves
//         // [0,0,1,0,1]=> println!("false"),
//         // //Scenario 10 - Missing duration and reserves
//         // [0,0,0,1,1]=> println!("false"),
//         _ => panic!()
//     };
// println!("OUTPUTS");
// println!("{:?}",outputs);

 

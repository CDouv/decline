

#[derive(Debug,Copy,Clone)]
pub enum ForecastParameter<T> {
    Known(T),
    Unknown,
}

pub fn array_floats(arr: &[ForecastParameter<f32>;5]) -> [f32;5] {

    let mut array_floats:[f32;5] = [0.0;5];
    

    for (i,value) in arr.iter().enumerate() {
        match value {
            ForecastParameter::Known(x) => array_floats[i] = *x,
            ForecastParameter::Unknown => array_floats[i] = 0.0
        }
         
        }
        return array_floats;
    }

pub fn input_manager() ->  [ForecastParameter<f32>;5] {

    let mut arr:[ForecastParameter<f32>;5] = [ForecastParameter::Unknown;5];

    let arr = input_initial_rate(arr);
    let arr = input_final_rate(arr);
    let arr = input_decline_rate(arr);
    let arr = input_duration(arr);
    let arr = input_reserves(arr);

    return arr;
}

//Initial Rate
    pub fn input_initial_rate(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {
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
    
    arr[0] = initial_rate;
    return arr
    }
// Final Rate
    pub fn input_final_rate(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {
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
        
        arr[1] = final_rate;
        return arr
        }

        // Decline Rate
        pub fn input_decline_rate(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {
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
            
            arr[2] = decline_rate;
            return arr
            }
        // Duration
        pub fn input_duration(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {
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
            
            arr[3] = incremental_duration;
            return arr
            }

                    // Duration
       pub fn input_reserves(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {
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
            
            arr[4] = incremental_reserves;
            return arr
            }

// Write a function to check which values are unknown

pub fn check_unknowns(arr: &[ForecastParameter<f32>;5]) -> [i32;5] {

    let mut knowns: [i32;5] = [0;5];
    

    for (i, parameter) in arr.iter().enumerate() {
        // println!("check array");
        // println!("{:?}",arr);
        match parameter {
            ForecastParameter::Known(f32) => knowns[i] = 0,
            ForecastParameter::Unknown =>  knowns[i] = 1
            }
        }

        return knowns
    }


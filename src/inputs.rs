

#[derive(Debug,Copy,Clone)]
pub enum ForecastParameter<T> {
    Known(T),
    Unknown,
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
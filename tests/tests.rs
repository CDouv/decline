

use decline::inputs::Exponential;
use decline::inputs::ForecastParameter;



#[cfg(test)]

mod tests {
    use super::*;

    #[test]

    fn check_unknowns() {
        let decline:Exponential<f32> = Exponential {
            qi:ForecastParameter::Unknown,
            qf:ForecastParameter::Known(452.43),
            d:ForecastParameter::Unknown,
            duration:ForecastParameter::Known(7.93),
            reserves:ForecastParameter::Known(2000.0),
        };
        assert_eq!(decline.check_unknowns(),[1,0,1,0,0])
    }

    fn missing_qi_qf() {
        let decline:Exponential<f32> = Exponential { 
            qi: (), 
            qf: (), 
            d: (), 
            duration: (), 
            reserves: () 
        };
        assert_eq!(decline.check_unkn)

    }
}
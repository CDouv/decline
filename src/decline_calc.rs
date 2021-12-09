
pub const E: f32 = 2.718;

use crate::inputs::ForecastParameter;


pub fn missing_qi_qf(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {


let decline = arr[2].extract_value();
let duration = arr[3].extract_value();
let np = arr[4].extract_value();


let qi :f32 = (-np*decline/(E.powf(-decline*duration)-1.0))*(1000.0/365.25);
let qf: f32 = qi*E.powf(-decline*duration);

return [ForecastParameter::Known(qi),
        ForecastParameter::Known(qf),
        ForecastParameter::Known(decline),
        ForecastParameter::Known(duration),
        ForecastParameter::Known(np)];

}
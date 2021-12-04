use crate::inputs::array_floats;
pub const E: f32 = 2.718;

use crate::inputs::ForecastParameter;
pub fn missing_qi_qf(mut arr: [ForecastParameter<f32>;5]) -> [ForecastParameter<f32>;5] {

let input_values = array_floats(&arr);

let decline = &input_values[2];
let duration = &input_values[3];
let np = &input_values[4];


let qi :f32 = (-np*decline/(E.powf(-decline*duration)-1.0))*(1000.0/365.25);
let qf: f32 = qi*E.powf(-decline*duration);

arr[0] = ForecastParameter::Known(qi);

arr[1] = ForecastParameter::Known(qf);

return arr
}
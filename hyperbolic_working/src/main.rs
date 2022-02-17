mod hyperbolic;

use crate::hyperbolic::Hyperbolic;
use crate::hyperbolic::ForecastParameter;

fn main() {

    let decline = Hyperbolic {
        qi:ForecastParameter::Known(492750),
        q: ForecastParameter::Unknown,
        di:ForecastParameter::Known(3.5513),
        d:ForecastParameter::Known(0.1054),
        t:ForecastParameter::Unknown,
        np:ForecastParameter::Unknown,
        b:ForecastParameter::Known(0.95),

    }

    let results = decline.to_array();

    println!("{:?}",results);

};
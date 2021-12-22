
use crate::inputs::Exponential;

pub fn bisection(exponential: &mut Exponential<f32>,bounds:(f32,f32),
            f:&dyn Fn(&mut Exponential<f32>,f32) -> f32) -> f32 {

    let mut a = bounds.0;
    let mut b = bounds.1;

    let mut c:f32 = (a-b)/2.0;

    while (f(exponential,a)/f(exponential,c)) > 0.05 && (f(exponential,b)/f(exponential,c)) > 0.05 {

    match (f(exponential,a)*f(exponential,c)<0.0,f(exponential,b)*f(exponential,c)<0.0) {
        (true,true) => 
            match (f(exponential,a) - f(exponential,c)) < (f(exponential,b)-f(exponential,c)) {
                true => 
                    b = c,
                false =>
                    a = c,
                    }
            
        (false,true) => a = c,
        (true,false) => b = c,
        (false,false) => panic!()
    }

    c = (a-b)/2.0;
}

c

}



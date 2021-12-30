//TODO - Stuck on how to use method pointers - using a single generic function in bisection and calling
//       the specific function in main.rs dep
use crate::inputs::Exponential;

impl Exponential<f32> { 
pub fn bisection(&self,
            bounds:(f32,f32)) -> f32 {
    
    let f = Exponential::missing_qi_d;


    let mut a = bounds.0;
    let mut b = bounds.1;

    let mut c:f32 = ((a-b)/2.0).abs();

    while (f(self,a)/f(self,c)) > 0.05 && (f(self,b)/f(self,c)) > 0.05 {

    match (f(self,a)*f(self,c)<0.0,f(self,b)*f(self,c)<0.0) {
        (true,true) => 
            match (f(self,a) - f(self,c)) < (f(self,b)-f(self,c)) {
                true => 
                    b = c,
                false =>
                    a = c,
                    }
            
        (false,true) => println!("door number 1"),
        (true,false) => println!("door number 2"),
        (false,false) => panic!("\na {}\n b {} \n c {}",a,b,c)
    }

    c = (a-b)/2.0;
}

c

}

}


// (false,true) => a = c,
//         (true,false) => b = c,
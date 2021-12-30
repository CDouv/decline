//TODO - Stuck on how to use method pointers - using a single generic function in bisection and calling
//       the specific function in main.rs dep
use crate::inputs::Exponential;

impl Exponential<f32> { 
pub fn bisection(&self,
            bounds:(f32,f32)) -> f32 {
    
    let f = Exponential::missing_qi_d;


    let mut a = bounds.0;
    let mut b = bounds.1;
    


    let mut c:f32 = ((a+b)/2.0).abs();
    let mut iteration = 1;

    while ((f(self,a) -f(self,c)) / f(self,a)).abs() > 0.05 && 
    ((f(self,b) -f(self,c)) / f(self,b)).abs() > 0.05 {

        println!("Start of iteration # {}",iteration);
        println!("\na {}\n b {} \n c {}",a,b,c);
        let mut res_a = f(self,a);
        let mut res_b = f(self,b);
        let mut res_c = f(self,c);
     
        println!("\nf(a) {}\n f(b) {} \n f(c) {}",res_a,res_b,res_c);
        println!("\n\n");

        match (f(self,a)*f(self,c)<0.0,f(self,b)*f(self,c)<0.0) {
            (true,true) => 
                match (f(self,a) - f(self,c)) < (f(self,b)-f(self,c)) {
                    true => 
                        b = c,
                    false =>
                        a = c,
                        }
            
            //root is between b and c (a becomes c)
            (false,true) =>
                a = c,

            // root is between a and c (b becomes c)
            (true,false) =>
                b = c,

            (false,false) => panic!("\na {}\n b {} \n c {}",a,b,c)
        }   

    
    c = ((a+b)/2.0).abs();

    //DEBUG test code
    // println!("End of iteration # {}",iteration);
    // println!("\na {}\n b {} \n c {}",a,b,c);
    // let mut res_a = f(self,a);
    // let mut res_b = f(self,b);
    // let mut res_c = f(self,c);
 
    // println!("\nf(a) {}\n f(b) {} \n f(c) {}",res_a,res_b,res_c);
    // println!("\n\n");

    iteration +=1;
}

c

}

}


// (false,true) => a = c,
//         (true,false) => b = c,
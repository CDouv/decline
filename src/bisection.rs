fn bisection(bounds:(f32,f32), f:&dyn Fn(f32) -> f32) -> f32 {
    let mut a = bounds[0];
    let mut b = bounds[1];

    let mut c:f32 = (a-b)/2;

    while (f(a)/f(c)) > 0.05 && (f(b)/f(c)) > 0.05 {

    match (f(a)*f(c)<0,f(b)*f(c)<0) {
        (true,true) => 
            match (f(a) - f(c)) < (f(b)-f(c)) {
                true => 
                    b = *c,
                false =>
                    a = *c,
                    }
            
        (false,true) => a = *c,
        (true,false) => b = *c,
        (false,false) => panic!()
    }

    c = (a-b)/2;
}

c;

}


     //Scenario 1 -Missing initial_rate and final_rate
    
     //bisection:  qi - (-Np*D+qi)*exp(D*t) = 0
    

     // Scenario 2 -Missing initial_rate and decline_rate
     //bisection - Np*D - qf*exp(D*t) + qf = 0
 
     // //Scenario 3 - Missing initial_rate and duration

     // //Scenario 4 - Missing initial_rate and reserves

     // //Scenario 5 - Missing final_rate and decline_rate

     // //Scenario 6 - Missing final_rate and duration
 
     // //Scenario 7 - Missing final_rate and reserves

     // //Scenario 8 - Missing decline_rate and duration

     // //Scenario 9 - issing decline_rate and reserves

     // //Scenario 10 - Missing duration and reserves
 
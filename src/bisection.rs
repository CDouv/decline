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



}
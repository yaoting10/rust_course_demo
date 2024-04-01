#[derive(Debug)]
struct Circle{
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle{
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle{
            x: x,
            y: y,
            radius: radius,
        }
    }

    fn area(&self) -> f64{
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[test]
fn test_circle(){
    let cir  = Circle::new(1_f64, 2_f64, 1_f64);
    println!("cir: {:?}", cir);
    println!("area: {}",cir.area())
}
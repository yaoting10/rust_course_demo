
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn new(w: u32 , h: u32) -> Rectangle{
        Rectangle{width:w, height: h}
    }
}

/// 同个struct可以有多个impl，方便代码组织
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        return self.width > other.width && self.height > other.height
    }
}

#[test]
fn test_rectangle(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", rect1.area());
    println!("{}",rect1.width);

    let new_rect = Rectangle::new(3, 4);
    println!("{:?}", new_rect);

    let can_hold = rect1.can_hold(&new_rect);

    println!("rect1 can hold new_rect: {}", can_hold)
}
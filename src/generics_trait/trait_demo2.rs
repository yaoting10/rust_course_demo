use std::convert::TryInto;
use std::ops::Add;

fn try_into_demo1(){
    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred");
    }
}

#[test]
fn test_try_into_demo1(){
    try_into_demo1();
}

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T:Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T:Add<T,Output = T>> Add for Point<T>{
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a :T, b : T)-> T{
    return a + b;
}

#[test]
fn test_point_add(){
    let p1 = Point{x: 1i32, y: 1i32};
    let p2 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p1, p2));
    let p3 = Point{x: 1.1f32, y: 1.1f32};
    let p4 = Point{x: 2.2f32, y: 2.2f32};
    println!("{:?}", add(p3, p4));
}

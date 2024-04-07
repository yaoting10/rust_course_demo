trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若T 实现了Draw特征，则调用该函数时传入的Box<T> 可以被隐式转换成函数参数签名中Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) {
    // 由于实现了Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的`draw`方法
    x.draw();
}

fn draw2(x: &dyn Draw) {
    x.draw();
}

#[test]
fn test_trait_object_demo2() {
    let x = 1.1f64;
    let y = 12u8;
    // let z = 7u16;
    println!("{:?}", draw1(Box::new(x)));
    draw1(Box::new(y));
    // println!("{:?}", );
    // draw1(Box::new(z));
    println!("{:?}", draw2(&x));
    draw2(&y);
    // draw2(&z);
}

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[test]
fn test_screen_draw(){
    // let screen
}
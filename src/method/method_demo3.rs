
#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        // let b = eq(self, *Message::Write(String::from("ok")));
        // println!(" id:{:?}", b)
        //  这里定义方法体
    }
}

#[test]
fn test_demo3(){
    let m = Message::Write(String::from("hello"));
    m.call();
    println!("{:?}", Message::Quit)
}

fn option_demo1(){
    let some_number= Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}, some_string:{:?}, absent_number:{:?}", some_number, some_string, absent_number)
}

#[test]
fn test_option_demo1(){
    option_demo1();
}

fn option_demo2(){
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // error
    // let sum = x + y;
    let sum = x + y.unwrap();
    println!("sum: {:?}", sum)
}

#[test]
fn test_option_demo2(){
    option_demo2();
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
#[test]
fn test_plus_one(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six :{:?}, None:{:?}", six, none)
}
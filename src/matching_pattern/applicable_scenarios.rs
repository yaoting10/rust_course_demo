fn scenarios_demo1() {
    // Vec 是动态数组
    let mut stack = Vec::new();
    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
}

#[test]
fn test_scenarios_demo1() {
    scenarios_demo1()
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y)
}

fn scenarios_demo2() {
    let point = (2, 3);
    print_coordinates(&point);
}

#[test]
fn test_scenarios_demo2(){
    scenarios_demo2();
}

fn scenarios_demo3(){
    let y: Option<i32> = None;
    // error
    // let Some(x) = y;
    if let Some(x) = y{
        println!("{:?}", x)
    }
}

#[test]
fn test_scenarios_demo3(){
    scenarios_demo3();
}

fn possession_demo1(){
    let x = String::from("hello");
    let y = x;
    // println!("x is {}", x);
    println!("y is {}", y);
}

#[test]
fn test_possession_demo(){
    possession_demo1()
}

fn possession_demo2(){
    let s = String::from("hello");
    // let x = String::from("hello");
    takes_ownership(s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放


fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
#[test]
fn test_possession_demo2(){
    possession_demo2()
}

fn possession_demo3(){
    let s1 = gives_ownership();         // gives_ownership 将返回值移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中,
    // 它也将返回值移给 s3
    println!("s1 is {}", s1);
    // s2 被移动到 takes_and_gives_back 中, 所以不能在这里使用
    // println!("s2 is {}", s2);
    println!("s3 is {}", s3);
} // 这里, s3 被移出作用域并被丢弃。s2 被移出作用域但已被移走，所以什么也不会发生。s1 被移出作用域并被丢弃

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

#[test]
fn test_possession_demo3(){
    possession_demo3()
}
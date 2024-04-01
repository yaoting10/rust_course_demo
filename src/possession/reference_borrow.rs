
fn borrow() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // assert_eq!(5, y); // : This is wrong
    assert_eq!(5, *y);
}

#[test]
fn test_borrow() {
    borrow()
}

fn reference_demo(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}', is {}", s1, len)
}

fn calculate_length(s: &String) -> usize { // s 是一个指向 String 的引用
    s.len()
}

#[test]
fn test_reference_demo(){
    reference_demo()
}
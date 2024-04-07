#[test]
fn test_array_demo1() {
    // let v:Vec<i32> = Vec::new();
    // let mut v = Vec::new();
    // v.push(1);

    let v = vec![1, 2, 3, 4, 5];
    let th: &i32 = &v[2];
    println!("第三个元素是:{}", th);

    match v.get(2) {
        Some(third) => println!("第三个元素是: {third}"),
        None => println!("第三个元素，根本没有")
    }
}

#[test]
fn test_array_demo2(){
    let v  = vec![1,2,3];
    for i in &v {
        println!("{i}")
    }

    let mut  v2 = vec![1,2,3];
    for i in &mut v2 {
        *i +=10;
    }
    println!("{:?}", v2)
}
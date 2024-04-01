
fn largest<T: PartialOrd+Clone>(list :&[T]) -> T {
    let mut largest = list[0].clone();
    for item in list.iter() {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

#[test]
fn test_largest(){
    let number_list = vec![34, 50, 25, 100, 85];
    let result = largest(&number_list);
    println!("{:?}", &result);

    let char_list = vec!["t", "m", "a", "q"];
    let result = largest(&char_list);
    println!("{:?}", result)
}

fn add<T: std::ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[test]
fn test_add() {
    let a = 1;
    let b = 15;
    let c = add(a, b);
    println!("add: {:?}", c)
}
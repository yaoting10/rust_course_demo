// miss lifecycle
// fn longest(x :&str, y:&str) -> &str{
//     if x.len() > y.len() {
//         x
//     }else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str)-> &'a str{
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

#[test]
fn test_lifecycle_demo1(){
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(&string1, string2);
    println!("The longest string is {}", result)
}

#[test]
fn test_lifecycle_demo2(){
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
    }

}

#[test]
fn test_lifecycle_demo3(){
    //error demo
    // let string1 = "long string is long".to_string();
    // let result;
    // {
    //     let string2 = "xyz".to_string();
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

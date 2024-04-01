fn greet(name: String) {
    println!("Hello, {name} !", )
}

#[test]
fn test_greet(){
    // error occurs when you run this test
    // let my_name = "Nick";
    let my_name = String::from("Nick");
    greet(my_name);
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Person{
    name: String,
    age: i32,
}

impl Person{
    fn new(name: String, age: i32) -> Person{
        Person{name, age}
    }
}

#[test]
fn test_array6_inner_demo1(){
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}


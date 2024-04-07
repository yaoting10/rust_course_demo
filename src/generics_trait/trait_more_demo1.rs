trait Animal {
    fn baby_name()-> String;
}

struct Dog;

impl Dog{
    fn baby_name()-> String{
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn test_trait_more_demo1(){
    println!("A baby dog is call a {}", Dog::baby_name());
    println!("A baby dog is call a {}",<Dog as Animal >::baby_name());
}
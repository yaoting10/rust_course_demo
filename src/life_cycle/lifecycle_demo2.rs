
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn test_lifecycle2_inner_demo1(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt{
        part: first_sentence,
    };
    println!("{:?}", i)
}

#[test]
fn test_lifecycle2_inner_demo2(){
    // error demo miss lifecycle
    // let i;
    // {
    //     let novel = String::from("Call me Ishmael. Some years ago...");
    //     let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    //     i = ImportantExcerpt{
    //         part: first_sentence,
    //     };
    // }
    // println!("{:?}", i)
}
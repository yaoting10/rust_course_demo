
use std::collections::HashMap;

#[test]
fn test_map2_inner_demo1(){
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    let team_name = String::from("Blue");
    // let score : Option<&i32> = scores.get("OK");
    // let score : Option<&i32> = scores.get(&team_name);
    let score:i32 = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {:?}", score)
}

#[test]
fn test_map2_inner_demo2(){
    let mut scores =HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);
    for (k, v) in scores {
        println!("{}: {}", k, v)
    }
}

#[test]
fn test_map_inner_demo3(){
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));
    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5
    assert_eq!(v, &5); // 不存在，插入5

    println!("{}",v);
    let v = scores.entry("Yellow").or_insert(10);
    assert_eq!(*v, 5) // 已经存在，因此50没有插入
}

#[test]
fn test_map2_inner_demo3(){
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_ascii_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }

    println!("{:?}", map)
}
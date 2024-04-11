use std::collections::HashMap;

#[test]
fn test_map_inner_demo1(){
    let mut m1 =HashMap::new();
    m1.insert("123", 1);
    m1.insert("234", 2);
    m1.insert("345", 3);
    println!("{:?}", m1)
}

#[test]
fn test_map_inner_demo2(){
    let items_list = vec![
        ("中国".to_string(), 1),
        ("美国".to_string(), 2),
        ("日本".to_string(), 3),
    ];

    let mut team_map = HashMap::new();
    for item in &items_list {
        team_map.insert(&item.0, &item.1);
    }
    println!("{:?}", team_map);
}

#[test]
fn test_map_inner_demo3(){
    let items_list = vec![
        ("中国".to_string(), 1),
        ("美国".to_string(), 2),
        ("日本".to_string(), 3),
    ];

    let team_map: HashMap<_, _> = items_list.into_iter().collect();
    println!("{:?}", team_map);
}

#[test]
fn test_map_inner_demo4(){
    let name = String::from("Sunface");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(name, age);  //所有权被转移
    handsome_boys.insert(&name, age);
    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age)
}

#[test]
fn test_map_inner_demo5(){
    let name = String::from("Sunface");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    handsome_boys.insert(&name, age);
    // std::mem::drop(name);
    println!("因为过于无耻，{:?}已经被除名", handsome_boys);
    println!("还有，他的真实年龄远远不止{}岁", age);
}
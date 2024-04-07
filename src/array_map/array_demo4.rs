#[test]
fn test_array4_inner_demo1() {
    let v = vec![0; 3]; //默认值为0，初始长度为3;
    let v_from = Vec::from([0, 0, 0]);
    assert_eq!(v, v_from)
}

#[test]
fn test_array4_inner_demo2() {
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]);
    println!("Vector 长度: {}, 容量: {}", v.len(), v.capacity());
    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!("Vector (reserve) 长度: {}, 容量:{}", v.len(), v.capacity());
    v.shrink_to_fit();  //释放剩余容量，一般情况下，不会主动去释放容量
    println!("Vector (shrink_to_fit) 长度: {}, 容量:{}", v.len(), v.capacity());
}

#[test]
fn test_array4_inner_demo3() {
    let mut v = vec![1, 2];
    assert!(!v.is_empty());     //检查v是否为空
    v.insert(2, 3); // 在指定所以插入数据，索引值不能大于v的长度，v:[1,2,3]
    // let i = v.remove(1);
    // println!("{i}");
    assert_eq!(v.remove(1), 2);  //移除指定位置的元素
    assert_eq!(v.pop(), Some(3)); //删除并返回v 尾部的元素 v:[1]
    assert_eq!(v.pop(), Some(1)); // v:[]
    assert_eq!(v.pop(), None); //  记得pop方法返回的是 Option 枚举值

    v.clear(); //清空v, v:[]

    let mut v1 = [11, 12, 13].to_vec(); // append 操作会导致v1清空数据，增加可变声明
    v.append(&mut v1); // 将v1中的所以元素添加到v 中，v1:[]
    println!("v:{:?}, v1:{:?}", v, v1);
    v.truncate(2);  // 截断到指定长度，多余的元素被删除, v: [1]
    println!("{:?}", v);
    v.retain(|x| *x > 11); // 保留满足条件的元素，即删除不满足条件的元素。
    println!("{:?}", v);

    let mut v2 = vec![11, 22, 33, 44, 55];
    // 删除指定范围的元素，同时获取被删除元素的迭代器，v2:[11,55], m:[22,33,44]
    let mut m: Vec<_> = v2.drain(1..=3).collect();
    println!("v2:{:?} m:{:?}", v2, m);
    let v3 = m.split_off(1); //指定索引处切分成两个vec, m[22], v3:[33, 44]
    println!("m:{:?}, v3:{:?}", m, v3)
}

#[test]
fn test_array4_inner_demo4(){
    let v = vec![11,22,33,44,55];
    let slice = &v[1..=3];
    println!("slice: {:?}", slice);
    assert_eq!(slice, &[22,33,44]);
}

#[test]
fn test_array4_inner_demo5(){
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];
    // vec.sort_unstable();
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
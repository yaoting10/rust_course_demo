#[derive(Debug)]
enum IpAddr{
    V4(String),
    V6(String),
}

fn show_addr(ip: IpAddr){
    println!("{:?}", ip)
}

#[test]
fn test_array_demo1(){
    let v = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];

    for ip in v {
        show_addr(ip)
    }
}


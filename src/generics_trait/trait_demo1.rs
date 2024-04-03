
// trait 类似于其它编程语言的接口
pub trait Summary {
    fn summarize(&self) ->String;
}

pub struct Post{
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post{
    fn summarize(&self) -> String {
        format!("title {}, author:{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo{
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

#[test]
fn test_trait_demo1(){
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}
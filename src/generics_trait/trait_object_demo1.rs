

pub trait Draw{
    fn draw(&self);
}
pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self) {
        // 绘制按钮的代码
    }
}

pub struct SelectBox{
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self) {
        // 绘制selectBox代码
    }
}


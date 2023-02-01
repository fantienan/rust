// T恤颜色
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ShirtColor {
    Red,
    Blue,
}
// 公司库存
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Inventory {
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    pub fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

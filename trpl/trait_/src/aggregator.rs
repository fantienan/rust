pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more {}...)", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }
// trait bound语法
pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize())
}

use blog1::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我今天中午吃了一份沙拉");

    let mut post = post.request_review();

    let post = post.approve();
}

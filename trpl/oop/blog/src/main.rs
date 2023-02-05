// 这个博客的最终功能看起来像这样：
// 1.博文从空白的草案开始。
// 2.一旦草案完成，请求审核博文。
// 3.一旦博文过审，它将被发表。
// 4.只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("我今天中午吃了一份沙拉");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("我今天中午吃了一份沙拉", post.content());
}

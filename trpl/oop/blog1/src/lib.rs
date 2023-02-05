// 增加 reject 方法将博文的状态从 PendingReview 变回 Draft
// 在将状态变为 Published 之前需要两次 approve 调用
// 只允许博文处于 Draft 状态时增加文本内容。提示：让状态对象负责内容可能发生什么改变，但不负责修改 Post。
use std::cell::RefCell;
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_num: None,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approve_num: Option<RefCell<i32>>, // approve_num: Option<i32>
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        if let Some(num) = self.approve_num {
            *num.borrow_mut() += 1;
            self.approve_num.borrow_mut() = Some(num)
        }
        Post {
            content: String::new(),
        }
    }
    // pub fn approve(&mut self) -> Post {
    //     if let Some(num) = self.approve_num.take() {
    //         self.approve_num = Some(num);
    //     } else {
    //         self.approve_num = Some(1);
    //     }
    //     if self.approve_num == Some(2) {
    //         Post {
    //             content: self.content.clone(),
    //         }
    //     } else {
    //         Post {
    //             content: String::new(),
    //         }
    //     }
    // }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

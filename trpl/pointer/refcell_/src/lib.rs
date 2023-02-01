pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messager,
{
    pub fn new(messager: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messager,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let precentage_of_max = self.value as f64 / self.max as f64;

        if precentage_of_max >= 0.1 {
            self.messager.send("错误: 您超过了配额！");
        } else if precentage_of_max >= 0.9 {
            self.messager.send("紧急警告:你已经使用了超过90%");
        } else if precentage_of_max >= 0.75 {
            self.messager.send("紧急警告:你已经使用了超过75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        // sent_messages: Vec<String>
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![]),
                // sent_messages: vec![],
            }
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            // self.sent_messages.push(String::from(msg));
            self.sent_messages.borrow_mut().push(String::from(msg));

            // // 运行时报错，在同一个作用域中创建两个可变引用
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(msg));
            // two_borrow.push(String::from(msg))
        }
    }

    #[test]
    fn it_sends_an_over_75_precent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.sent_messages.borrow().len(), 1);
    }
}

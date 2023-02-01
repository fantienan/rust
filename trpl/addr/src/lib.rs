/**
 * cargo test 运行所有测试
 * cargo test -- --show-output 显示输出内容
 * cargo test it_adds_two 运行单个测试
 * cargo test it 过滤 
 * cargo test -- --ignored 运行被列为ingore的测试
 * cargo test -- --include-ignorde 运行所有测试报错ignore
 */
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width > rect.width && self.height > rect.height {
            true
        } else {
            false
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
        println!("adsf");
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explortion() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
    #[test]
    fn largest_can_hold_smapller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    #[ignore]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }
}

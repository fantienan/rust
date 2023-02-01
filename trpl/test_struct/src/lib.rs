// #[cfg(test)] 测试模块注解告诉Rust只再执行cargo test时才便宜和运行代码
// cfg 是configuration配置的意思，它告诉Rust其之后的项只应该被包含进特定配置选项中

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // 测试私有函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

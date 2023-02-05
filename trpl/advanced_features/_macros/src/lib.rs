// 使用macro_rules! 的声明宏用于通用元编程
#[macro_export]
macro_rules! vec1 {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_work() {
        let v = vec1![1,2,3,4];
        assert_eq!(vec![1,2,3,4], v)
    }
}
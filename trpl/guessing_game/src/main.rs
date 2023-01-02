// 本章内容：https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html
// 将io输入输出库引入当前作用域，io库来自于标准库，也被称为std
// cargo doc --open 构建所有本地依赖提供的文档
use std::io;
// Ordering枚举类型，比较两个值是可能出现的三种结果
use std::cmp::Ordering;
// 生成随机数
use rand::Rng;

fn main() {
    println!("猜数字!");
    // 1..=100范围表达式，指包含上下端点的范围
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("神秘数字是：{secret_number}");
    loop {
        println!("请输入数字");
        // String 是一个便准看提供的字符串类型，它是UTF-8编码的可增长文本块
        // String::new返回一个String新实例
        // ::new ::语法表明new是String类型的一个关联函数，关联函数是针对类型实现的，在例子中是String，而不是String的实例，一些语言中把它称为静态方法。
        let mut guess = String::new();
        // io::stdin 返回Stdin的实例，这代表终端标准输入句柄的类型
        io::stdin()
            // 从标准输入句柄中获取用户输入，并让其将用户输入的内容存储到&mut guess字符串中
            // Result 是一种枚举类型enum，枚举类型变量的值可以是多种可能状态中的一个，把每种可能的状态称为一种枚举成员
            // 这里Result类型将用来编码错误处理的信息
            .read_line(&mut guess)
            // expect 出现错误时立即让程序崩溃
            .expect("读行失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        // {} 预留在特定位置的占位符
        println!("你猜的数字是：{guess}");
        // match 表达式由分支构成，一个分支包含一个模式和表达式开头的值与分支模式相匹配时应该执行的代码
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}

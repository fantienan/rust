use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // 基于vector创建哈希map
    let teams = vec![String::from("Bllue"), String::from("Yellow")];
    let initial_scores = vec![10,50];
    let mut scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里field_name field_value不再有效
    // 通过get方法并提供对应的键来从哈希map中获取值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key,value) in &scores {
        println!("{}：{}", key,value)
    }
    // 覆盖一个值
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);
    // 只有键没有值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(10);
    println!("{:?}", scores);
    // 根据旧值更新一个值, 记录单词出现过几次
    let text = "hello world wonderfil world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map)
    
}

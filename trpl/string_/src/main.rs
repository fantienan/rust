fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "ininin".to_string();
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("{}", s);
    let s3 = s + s2;
    let s = String::from("hello");
    // let h = s[0];
    let hello = "adfadfasdf";
    let answer = &hello[0..4];
    for c in "नमस्ते".chars() {
        println!("{}", c)
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b)
    }
}

fn main() {
    let s = String::from("hook-coder");
    let len = first_word(&s);
    println!("len:{}",len);
    
    test_word();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // println!("{:?}",std::str::from_utf8(bytes));   
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn test_word(){
    let s = String::from("hello world");
    let hello = &s[0..5]; 
    let world = &s[6..11];

    println!("{},{}",hello,world);

    // 首位开始
    // &s[0..5] === &s[..5]
    // 末尾结束
    // &s[6..len] === &s[6..]
    // 首位到末尾 
    // &s[..]


    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
}



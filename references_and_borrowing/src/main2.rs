pub fn main(){
    let mut s = String::from("hello");
    change(&mut s);

    println!("{}",s);

    // 这里r1 不需要写mut是因为借用的s 已经是mut了
    let r1 = &mut s;
    r1.push_str("r1");
    println!("{}",r1);

    let r2 = &mut s;
    println!("{}",r2);

    test();

    let mut reference_to_noting = test2();
    reference_to_noting.insert_str(0,"wave-");
    println!("{}",reference_to_noting)
}

fn change(some_string: &mut String){
    // 引用的 参数 无法修改 引用没有所有权 所以不能改。
    some_string.push_str(", world");
}

fn test(){
    // let mut s = String::from("hook");
    // {
    //     let r1 = &mut s;
    // } //在这里的时候 r1 其实已经就被销毁了 所以println用不了r1
    // let r2 = &mut s;
    // println!("{},{}",r1, r2);

    // 这里出现的问题是 主要原因就是借用了可变和不可变所以报错
    // 多个不可变 是可以的 因为仅仅读取数据，没能力影响数据。
    // let mut s = String::from("hook");

    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    
    // println!("{},{} and {}",r1,r2,r3)
}

fn test2 () -> String {
    let s = String::from("hook");
    s
}
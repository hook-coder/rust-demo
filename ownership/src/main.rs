fn main() {
    let s = String::from("hello");
    let s1 = s;
    // println!("这里的s不能用, s = {},",s);
    println!("{},world!", s1);
    /*
        这里s1 复制了 s堆栈上的指针、长度和容量。
        但不会复制指针指引的堆上数据。（！！！）
    */

    // 克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}",s1,s2);

    // 为什么这里的xy都可以用?
    /*
        原因是在编译时具有已知大小的整数等类型完全存储在堆栈中，
        因此可以快速创建实际值的副本。这意味着我们没有理由x在创建变量后阻止其有效y。
        换句话说，这里的深拷贝和浅拷贝没有区别，所以调用clone不会做任何与通常的浅拷贝不同的事情，我们可以省略它。
    */
    let x = 7;
    let y = x;
    println!("x = {},y = {}",x,y);

    // 返回多个参数
    let s1 = String::from("hello");
    let (s2,len) = calculate_length(s1);
    
    println!("The length of '{}' is '{}'",s2, len);
}


fn calculate_length(s:String)->(String, usize){
    let length = s.len();
    (s,length)
}
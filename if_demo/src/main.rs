fn main() {
    let number = 7;
    if number < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    // 判断语句 必须是bool 如果不是则会报错
    // !!!!!!!!!
    // let my_number = 3;
    // if number {
    //     println!("number was three");
    // }
    let my_number = 3;
    if my_number != 0 { println!("number was something other then zero") };

    // 多条件else if
    let num = 6;
    if num % 4 == 0 {
        println!("num is divisible by 4")
    }else if num % 3 == 0 {
        println!("num is divisible by 3")
    }else if num % 2 == 0 {
        println!("num is divisible by 2")
    }else {
        println!("num is not divisible by 4, 3, or 2")
    }

    // if在let声明中使用
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // 如果类型不一致 也是会产生错误的 ！！！！
    // let num =  if condition { 5 } else { "six" };
    // println!("The value of num is:{num}"); 
    println!("The value of number is:{number}"); 

}

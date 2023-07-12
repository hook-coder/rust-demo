fn main() {
    another_function(3);
    print_labeled_measurement(7,'k');

    /*
    语句和表达式 
    语句 是执行某些操作但不返回值的指令
    表达式 计算结果值
    */
    let y = {
        let x = 3;
        x + 1 //这里要注意的是 没有分号 等价于 return x + 1;
    };
    println!("The value of y is:{y}");

    // 有返回值的函数
    let f = five();
    println!("The value of f is:{f}");

    // 有参数并且又返回值的函数
    let p = plus_one(f);
    println!("The value of p is:{p}");
}

fn another_function(x:i32){
    println!("The value of x is:{x}");
}
/*
 定义函数的参数 都要写参数的类型
*/
fn print_labeled_measurement(value:i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}


// 有返回值的函数
fn five()-> i32{
    100
}
// 有返回值 并且又传参的函数
fn plus_one(x:i32) -> i32{
    x + 1
}
fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{guess}");
    // demo1();
    // demo2();
    demo3();
}

fn demo1() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("{sum},{difference},{product},{quotient},{truncated},{remainder}");
}

/*
 * 复合类型有元组和数组 元组和数组一样 有固定的长度 一旦声明它们的大小就不能增加或缩小
 * 与元组不同的是 数组的每个元素必须具有相同的类型
*/
// 复合类型
fn demo2(){
    //  结构元组
    let tup = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("The value of tup is:{x},{y},{z}");

    // 访问元组元素
    let get_y = tup.1; // 拿到y的值 元组可以工具索引来获取到值
    println!("The value of get_y is:{get_y}");


    // 定义数组
    // let array = [1,2,3,'123']; 就像这样 数组类型不一致就会出错
    let my_array = [1,3,5,7];
    let my_str_array = ["2","3","4","dad"];
    println!("The value of my_array is:{:?}",my_array);
    println!("The value of my_str_array is:{:#?}",my_str_array);

    /*
     * 定义数组的类型和数组的长度 u32是类型 2则是数组长度
     * i32 和 u32的区别 
     * i32 支持整数和负数
     * u32 只支持整数
    */
    let a:[u32;2] = [1,0];
    println!("The value of a is:{:?}",a);
    let b = [7;5]; // == [7,7,7,7,7]
    println!("The value of b is:{:?}",b);

    // 访问数组元素
    let c = [1,2,3];
    let first = c[0];
    let second = c[1];
    println!("The value of first is:{first} \n The value of second is:{second}")
}

use std::io;
fn demo3(){
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index:usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];

    println!("The value of the element at index {index} is:{element}");
}
fn main() {
    demo1();
    demo2();
    demo3();
    demo4();
    demo5();
}

fn demo1(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn demo2(){
    const THEREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;
    println!("the value of THEREE_HOURS_IN_SECONDS is: {}",THEREE_HOURS_IN_SECONDS)
}

fn demo3(){
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x in the inner scope is: {x}");
}

fn demo4(){
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces length is:{spaces}");
}

fn demo5(){
    /*
     * 第一个spaces是字符串类型，第二个spaces是数字类型，
     * 所以不匹配 
     * 就算加上mut也不可以 
     * let之后就定义了该变量的类型 定以后的类型不可更改
    */

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("The value of spaces length is:{spaces}");
}
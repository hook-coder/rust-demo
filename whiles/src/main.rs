fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    demo();
    dmeo2();
}

fn demo(){
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn dmeo2(){
    let a = [1,2,3,4,5];
    for element in a {
        println!("the value is: {element}");
    }
}
mod main2;
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    let b = &s1;

    println!("b：{}",b);
    println!("The length of '{}' is '{}'.",s1,len);
    
    main2::main();
}

fn calculate_length(s: &String)-> usize{
    s.len()
}
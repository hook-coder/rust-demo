// 输出打印 struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn demo() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    // dbg! 返回表达式值的所有权
    dbg!(&rect1);
}
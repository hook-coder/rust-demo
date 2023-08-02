pub fn demo(){
  let width1 = 30;
  let height1 = 50;

  println!(
    "The area of the rectangle is {} square pixels.",
    area(width1,height1)
  )
}

fn area(width:u32, height:u32) -> u32 {
  width * height
}

// 这段代码的优化是把2个参数 是否有相关整合在一起了  一开始是2个参数 现在是一个 然它们更是一个整体
pub fn demo2(){
  let  rect1 = (30, 50);

  println!(
    "The area of the rectangle is {} square pixels.",
    area2(rect1)
  )
}

fn area2(dimensions:(u32,u32)) -> u32 {
  dimensions.0 * dimensions.1
}

// 这段代码的优化是用结构
struct Rectangle {
  width: u32,
  height: u32,
}

pub fn demo3(){
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "The area of the rectangle is {} square pixels.",
    area3(&rect1)
  )
}

fn area3(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}

// 这段代码优化 主要是用 impl来实现 让他更加完全的整合在一起
struct Rectangle2 {
  width: u32,
  height: u32,
}

impl Rectangle2 {
  fn width(&self) -> bool {
    self.width > 0
  }

  fn area(&self) -> u32 {
    self.width * self.height
  }
}

pub fn demo4(){
  let rect1 = Rectangle2 {
    width: 30,
    height: 50,
  };

  if rect1.width() {
      println!("大于0的宽度:{}",rect1.width)
  }
  println!(
    "The area of the rectangle is {} square pixels.",
    rect1.area()
  )
}







// 元组结构
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main(){
  let black = Color(0,0,0);
  let orgin = Point(0,0,0);
}


// 类单元结构
struct AlwaysEqual;

fn test(){
  let subject = AlwaysEqual;
}